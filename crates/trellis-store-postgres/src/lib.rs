// Rust guideline compliant 2026-02-21
//! Postgres-backed Trellis event storage.
//!
//! This crate owns the default production storage seam for the current
//! Phase-1 append runtime. It intentionally exposes Trellis-owned types and
//! does not leak `postgres` crate types through its public API.
//!
//! # TLS
//!
//! [`PostgresStore::connect`] currently uses `postgres::NoTls`. That is
//! intentional for the Phase-1 scaffold and local test clusters (cleartext on
//! the loopback interface only). Before any non-localhost deployment, wire in
//! explicit TLS (or restrict connection strings to trusted transports) so
//! credentials and ledger payloads are not exposed on the network.

#![forbid(unsafe_code)]

use std::backtrace::Backtrace;
use std::fmt::{Display, Formatter};

use postgres::{Client, NoTls};
use trellis_core::LedgerStore;
use trellis_types::StoredEvent;

const CREATE_EVENTS_TABLE_SQL: &str = "\
CREATE TABLE IF NOT EXISTS trellis_events (\
    scope BYTEA NOT NULL,\
    sequence BIGINT NOT NULL,\
    canonical_event BYTEA NOT NULL,\
    signed_event BYTEA NOT NULL,\
    PRIMARY KEY (scope, sequence)\
)";

/// Error returned when the Postgres store cannot complete an operation.
#[derive(Debug)]
pub struct PostgresStoreError {
    message: String,
    backtrace: Backtrace,
}

impl PostgresStoreError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            backtrace: Backtrace::capture(),
        }
    }

    /// Returns the captured backtrace for this store failure.
    pub fn backtrace(&self) -> &Backtrace {
        &self.backtrace
    }
}

impl Display for PostgresStoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for PostgresStoreError {}

/// Postgres-backed store for canonical and signed events.
pub struct PostgresStore {
    client: Client,
}

impl std::fmt::Debug for PostgresStore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PostgresStore").finish_non_exhaustive()
    }
}

impl PostgresStore {
    /// Connects to Postgres and ensures the Trellis schema exists.
    ///
    /// Uses cleartext transport ([`NoTls`]). Acceptable for localhost-only
    /// Phase-1 development; do not point this at untrusted networks without
    /// adding TLS.
    ///
    /// # Errors
    /// Returns an error when the database connection or schema creation fails.
    pub fn connect(connection_string: &str) -> Result<Self, PostgresStoreError> {
        let mut client = Client::connect(connection_string, NoTls).map_err(|error| {
            PostgresStoreError::new(format!("failed to connect to Postgres: {error}"))
        })?;
        client
            .batch_execute(CREATE_EVENTS_TABLE_SQL)
            .map_err(|error| {
                PostgresStoreError::new(format!(
                    "failed to initialize Trellis schema in Postgres: {error}"
                ))
            })?;

        Ok(Self { client })
    }

    /// Loads stored events for `scope` in canonical sequence order.
    ///
    /// # Errors
    /// Returns an error when the query fails or stored values do not fit the
    /// current Phase-1 type bounds.
    pub fn load_scope_events(
        &mut self,
        scope: &[u8],
    ) -> Result<Vec<StoredEvent>, PostgresStoreError> {
        let rows = self
            .client
            .query(
                "\
SELECT scope, sequence, canonical_event, signed_event \
FROM trellis_events \
WHERE scope = $1 \
ORDER BY sequence ASC",
                &[&scope],
            )
            .map_err(|error| {
                PostgresStoreError::new(format!("failed to query Trellis events: {error}"))
            })?;

        rows.into_iter()
            .map(|row| {
                let sequence_i64 = row.get::<_, i64>("sequence");
                let sequence = u64::try_from(sequence_i64).map_err(|_| {
                    PostgresStoreError::new(format!(
                        "stored sequence `{sequence_i64}` does not fit into u64"
                    ))
                })?;

                Ok(StoredEvent::new(
                    row.get("scope"),
                    sequence,
                    row.get("canonical_event"),
                    row.get("signed_event"),
                ))
            })
            .collect()
    }
}

impl LedgerStore for PostgresStore {
    type Error = PostgresStoreError;

    fn append_event(&mut self, event: StoredEvent) -> Result<(), Self::Error> {
        let sequence = i64::try_from(event.sequence()).map_err(|_| {
            PostgresStoreError::new(format!(
                "sequence `{}` does not fit into Postgres BIGINT",
                event.sequence()
            ))
        })?;

        self.client
            .execute(
                "\
INSERT INTO trellis_events (scope, sequence, canonical_event, signed_event) \
VALUES ($1, $2, $3, $4)",
                &[
                    &event.scope(),
                    &sequence,
                    &event.canonical_event(),
                    &event.signed_event(),
                ],
            )
            .map_err(|error| {
                PostgresStoreError::new(format!("failed to append Trellis event: {error}"))
            })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::ffi::OsStr;
    use std::net::{TcpListener, TcpStream};
    use std::path::{Path, PathBuf};
    use std::process::{Command, Stdio};
    use std::thread;
    use std::time::{Duration, Instant};

    use tempfile::TempDir;

    use super::PostgresStore;
    use trellis_core::LedgerStore;
    use trellis_types::StoredEvent;

    #[test]
    fn postgres_store_persists_and_reads_scope_events() {
        let cluster = TestCluster::start();
        let mut store = PostgresStore::connect(&cluster.connection_string()).unwrap();

        store
            .append_event(StoredEvent::new(
                b"scope-a".to_vec(),
                1,
                vec![0x01, 0x02],
                vec![0x03, 0x04],
            ))
            .unwrap();
        store
            .append_event(StoredEvent::new(
                b"scope-a".to_vec(),
                2,
                vec![0x05, 0x06],
                vec![0x07, 0x08],
            ))
            .unwrap();
        store
            .append_event(StoredEvent::new(
                b"scope-b".to_vec(),
                0,
                vec![0x09],
                vec![0x0a],
            ))
            .unwrap();

        let events = store.load_scope_events(b"scope-a").unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].sequence(), 1);
        assert_eq!(events[0].canonical_event(), &[0x01, 0x02]);
        assert_eq!(events[1].sequence(), 2);
        assert_eq!(events[1].signed_event(), &[0x07, 0x08]);
    }

    struct TestCluster {
        temp_dir: TempDir,
        port: u16,
        pg_ctl: PathBuf,
    }

    impl TestCluster {
        fn start() -> Self {
            let temp_dir = TempDir::new().unwrap();
            let data_dir = temp_dir.path().join("data");
            let socket_dir = temp_dir.path().join("socket");
            std::fs::create_dir_all(&socket_dir).unwrap();

            let initdb = find_pg_binary("initdb");
            let pg_ctl = find_pg_binary("pg_ctl");
            let port = reserve_port();

            run_command(
                Command::new(&initdb)
                    .arg("-D")
                    .arg(&data_dir)
                    .arg("--username=postgres")
                    .arg("--auth=trust")
                    .arg("--no-locale"),
            );
            run_command(
                Command::new(&pg_ctl)
                    .arg("-D")
                    .arg(&data_dir)
                    .arg("-o")
                    .arg(format!("-F -p {port} -k {}", socket_dir.display()))
                    .arg("start"),
            );
            wait_for_postgres(port);

            Self {
                temp_dir,
                port,
                pg_ctl,
            }
        }

        fn connection_string(&self) -> String {
            format!(
                "host=127.0.0.1 port={} user=postgres dbname=postgres",
                self.port
            )
        }
    }

    impl Drop for TestCluster {
        fn drop(&mut self) {
            let data_dir = self.temp_dir.path().join("data");
            let _ = Command::new(&self.pg_ctl)
                .arg("-D")
                .arg(&data_dir)
                .arg("-m")
                .arg("immediate")
                .arg("stop")
                .status();
        }
    }

    fn reserve_port() -> u16 {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.local_addr().unwrap().port()
    }

    fn wait_for_postgres(port: u16) {
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            if TcpStream::connect(("127.0.0.1", port)).is_ok() {
                return;
            }
            assert!(
                Instant::now() < deadline,
                "temporary Postgres cluster did not start listening on port {port}"
            );
            thread::sleep(Duration::from_millis(50));
        }
    }

    fn find_pg_binary(name: &str) -> PathBuf {
        for candidate in command_search_paths(name) {
            if candidate.exists() {
                return candidate;
            }
        }

        panic!("failed to locate required Postgres binary `{name}`");
    }

    fn command_search_paths(name: &str) -> Vec<PathBuf> {
        let mut candidates = Vec::new();

        if let Some(paths) = env::var_os("PATH") {
            for dir in env::split_paths(&paths) {
                candidates.push(dir.join(name));
            }
        }

        candidates.push(Path::new("/opt/homebrew/opt/postgresql@16/bin").join(name));
        candidates.push(Path::new("/usr/local/opt/postgresql@16/bin").join(name));
        candidates
    }

    fn run_command(command: &mut Command) {
        let rendered = render_command(command);
        command.stdout(Stdio::null()).stderr(Stdio::null());
        let status = command.status().unwrap();
        assert!(
            status.success(),
            "command `{rendered}` failed with status {status}",
        );
    }

    fn render_command(command: &Command) -> String {
        let program = command.get_program().to_string_lossy();
        let args = command
            .get_args()
            .map(OsStr::to_string_lossy)
            .collect::<Vec<_>>()
            .join(" ");
        format!("{program} {args}")
    }
}
