// Rust guideline compliant 2026-02-21
//! Postgres-backed Trellis event storage — production-hardened canonical schema.
//!
//! This crate owns the canonical-side of the wos-server `EventStore`
//! composition (per [`work-spec/crates/wos-server/VISION.md`] §IV / §VI):
//! `trellis_events` is the Trellis-shaped, hash-chained, signed canonical
//! schema, written in the same Postgres transaction as wos-server's
//! `projections` schema. Single Postgres database, two schemas, one
//! transaction per write — see [`append_event_in_tx`].
//!
//! # Public surface
//!
//! - [`PostgresStore`] — owns a single `postgres::Client`; impls
//!   [`trellis_core::LedgerStore`] for the simple per-store append path.
//! - [`append_event_in_tx`] — free function taking an externally-supplied
//!   `&mut postgres::Transaction`, so wos-server's `EventStore` composes
//!   the canonical write and projection writes in one atomic transaction
//!   (the load-bearing invariant that VISION.md §VIII rejection of
//!   dual-write rests on).
//! - [`PostgresStorePool`] — `r2d2`-managed connection pool;
//!   [`PostgresStorePool::checkout`] returns a borrowed `PostgresStore`.
//!
//! # TLS
//!
//! - [`PostgresStore::connect`] refuses non-loopback DSNs to prevent
//!   accidental cleartext on the wire — Phase-1 local-only scaffold.
//! - [`PostgresStore::connect_with_tls`] takes a
//!   [`native_tls::TlsConnector`] and accepts any DSN. Production
//!   deployments use this path.
//! - [`PostgresStorePool::builder`] mirrors both, gated identically.
//!
//! Cleartext on a non-loopback host is a category error: ledger payloads,
//! KMS-mediated wrap keys, and authenticated identities all flow through
//! these connections. Refusing the unsafe combination at the constructor
//! is the lowest-debt enforcement seam.
//!
//! # Schema migrations
//!
//! Schema lives behind a small versioned migration runner ([`migrations`]
//! module). Connecting through any constructor applies pending migrations
//! and records them in `trellis_schema_migrations`. The migration set is
//! append-only: never edit a landed migration; add a new one. Schema
//! parity tests live in this crate.

#![forbid(unsafe_code)]

use std::backtrace::Backtrace;
use std::fmt::{Display, Formatter};
use std::time::Duration;

use native_tls::TlsConnector;
use postgres::{Client, NoTls, Transaction};
use postgres_native_tls::MakeTlsConnector;
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::PostgresConnectionManager;
use trellis_core::LedgerStore;
use trellis_types::StoredEvent;

mod migrations;

/// Maximum byte length of an `idempotency_key` per Trellis Core §6.1 / §17.2
/// (`bstr .size (1..64)`). Postgres-side enforcement; full Rust threading
/// (parsing, hash preimage, verifier) is item #24 in `trellis/TODO.md`.
pub const IDEMPOTENCY_KEY_MAX_LEN: usize = 64;

/// Error returned when the Postgres store cannot complete an operation.
#[derive(Debug)]
pub struct PostgresStoreError {
    message: String,
    kind: PostgresStoreErrorKind,
    backtrace: Backtrace,
}

/// Failure category for [`PostgresStoreError`].
///
/// Stable taxonomy callers can match on without parsing message strings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum PostgresStoreErrorKind {
    /// TCP/auth/network failure connecting to Postgres.
    ConnectionFailed,
    /// Caller-supplied DSN rejected by safety gating (non-loopback without TLS).
    UnsafeDsn,
    /// Schema migration failed.
    MigrationFailed,
    /// SQL execution failed (other than idempotency-key conflict or domain failures below).
    QueryFailed,
    /// Two appends collided on `(ledger_scope, idempotency_key)` per Core §17.3.
    /// Postgres surface for `IdempotencyKeyPayloadMismatch` (Core §17.5).
    IdempotencyKeyPayloadMismatch,
    /// Stored data did not fit Phase-1 type bounds (e.g. sequence overflow).
    DomainViolation,
    /// `idempotency_key` exceeded `bstr .size (1..64)` per Core §6.1.
    IdempotencyKeyTooLong,
    /// Connection pool failure (acquire / build).
    PoolFailed,
}

impl PostgresStoreError {
    fn new(kind: PostgresStoreErrorKind, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            kind,
            backtrace: Backtrace::capture(),
        }
    }

    /// Returns the structured failure category.
    pub fn kind(&self) -> PostgresStoreErrorKind {
        self.kind
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

/// Postgres-backed store for canonical and signed Trellis events.
pub struct PostgresStore {
    client: Client,
}

impl std::fmt::Debug for PostgresStore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PostgresStore").finish_non_exhaustive()
    }
}

impl PostgresStore {
    /// Connects to Postgres over **cleartext** (`NoTls`) and applies migrations.
    ///
    /// **Refuses any DSN whose `host` is not a loopback address** (`localhost`,
    /// `127.0.0.0/8`, `::1`, or a Unix socket directory). Production deployments
    /// MUST use [`Self::connect_with_tls`] — cleartext credentials and ledger
    /// bytes on a non-loopback wire is a category error.
    ///
    /// # Errors
    ///
    /// - [`PostgresStoreErrorKind::UnsafeDsn`] when the DSN names a non-loopback host.
    /// - [`PostgresStoreErrorKind::ConnectionFailed`] on TCP/auth failure.
    /// - [`PostgresStoreErrorKind::MigrationFailed`] when schema initialization fails.
    pub fn connect(connection_string: &str) -> Result<Self, PostgresStoreError> {
        require_loopback_dsn(connection_string)?;
        let client = Client::connect(connection_string, NoTls).map_err(|error| {
            PostgresStoreError::new(
                PostgresStoreErrorKind::ConnectionFailed,
                format!("failed to connect to Postgres: {error}"),
            )
        })?;
        Self::from_client(client)
    }

    /// Connects to Postgres using the supplied [`native_tls::TlsConnector`] and applies migrations.
    ///
    /// Accepts any DSN. Production deployments MUST use this path.
    ///
    /// # Errors
    ///
    /// - [`PostgresStoreErrorKind::ConnectionFailed`] on TCP/auth/TLS failure.
    /// - [`PostgresStoreErrorKind::MigrationFailed`] when schema initialization fails.
    pub fn connect_with_tls(
        connection_string: &str,
        tls: TlsConnector,
    ) -> Result<Self, PostgresStoreError> {
        let connector = MakeTlsConnector::new(tls);
        let client = Client::connect(connection_string, connector).map_err(|error| {
            PostgresStoreError::new(
                PostgresStoreErrorKind::ConnectionFailed,
                format!("failed to connect to Postgres over TLS: {error}"),
            )
        })?;
        Self::from_client(client)
    }

    fn from_client(mut client: Client) -> Result<Self, PostgresStoreError> {
        migrations::apply(&mut client)?;
        Ok(Self { client })
    }

    /// Begins a Postgres transaction borrowed from this store's client.
    ///
    /// Callers can pass the returned [`Transaction`] to [`append_event_in_tx`] and
    /// then issue additional SQL through the same transaction (e.g. wos-server
    /// projection writes). On `commit` both the canonical event and the projection
    /// updates land atomically — single-transaction-per-write per VISION.md §VIII.
    ///
    /// # Errors
    /// Returns [`PostgresStoreErrorKind::QueryFailed`] if the transaction cannot start.
    pub fn begin(&mut self) -> Result<Transaction<'_>, PostgresStoreError> {
        self.client.transaction().map_err(|error| {
            PostgresStoreError::new(
                PostgresStoreErrorKind::QueryFailed,
                format!("failed to begin Postgres transaction: {error}"),
            )
        })
    }

    /// Loads stored events for `scope` in canonical sequence order.
    ///
    /// # Errors
    /// Returns [`PostgresStoreErrorKind::QueryFailed`] when the query fails.
    /// Returns [`PostgresStoreErrorKind::DomainViolation`] when a stored sequence
    /// does not fit `u64`.
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
                PostgresStoreError::new(
                    PostgresStoreErrorKind::QueryFailed,
                    format!("failed to query Trellis events: {error}"),
                )
            })?;

        rows.into_iter()
            .map(|row| {
                let sequence_i64 = row.get::<_, i64>("sequence");
                let sequence = u64::try_from(sequence_i64).map_err(|_| {
                    PostgresStoreError::new(
                        PostgresStoreErrorKind::DomainViolation,
                        format!("stored sequence `{sequence_i64}` does not fit into u64"),
                    )
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
        // Forward the threaded Core §6.1 / §17.2 `idempotency_key` to
        // `append_event_in_tx` so the partial unique index on
        // `(scope, idempotency_key)` enforces the §17.3 wire-contract identity.
        let mut tx = self.begin()?;
        let key = event.idempotency_key();
        append_event_in_tx(&mut tx, &event, key)?;
        tx.commit().map_err(|error| {
            PostgresStoreError::new(
                PostgresStoreErrorKind::QueryFailed,
                format!("failed to commit append transaction: {error}"),
            )
        })?;
        Ok(())
    }
}

/// Appends one canonical event into the supplied [`Transaction`].
///
/// **The single-transaction composition seam.** wos-server's `EventStore`
/// opens a transaction on its own `postgres::Client`, calls this function
/// to write the canonical event, then writes its `projections` rows
/// through the same `tx`, and commits. Trellis owns the canonical row
/// shape; the caller owns the connection.
///
/// `idempotency_key` is the Postgres half of Core §17.3 — when
/// `Some(key)`, the table's partial unique index on
/// `(scope, idempotency_key)` enforces "one canonical event per
/// `(ledger_scope, idempotency_key)` forever." Phase-1 Rust threading
/// closed Wave 24 (formerly item #24, renumbered to item #2 then closed):
/// `trellis-cddl::parse_authored_event` extracts the field, `trellis-core`
/// threads it through `StoredEvent::with_idempotency_key`, and the
/// `LedgerStore::append_event` impl above forwards it here. wos-server's
/// `(caseId, recordId)`-derived key per
/// [WOS ADR 0061](../../../work-spec/thoughts/adr/0061-custody-hook-trellis-wire-format.md)
/// composes through this same surface.
///
/// # Errors
/// - [`PostgresStoreErrorKind::IdempotencyKeyTooLong`] if `key.len() > 64`.
/// - [`PostgresStoreErrorKind::IdempotencyKeyPayloadMismatch`] when the
///   `(scope, idempotency_key)` partial unique index rejects the insert.
/// - [`PostgresStoreErrorKind::DomainViolation`] when the sequence does not fit i64.
/// - [`PostgresStoreErrorKind::QueryFailed`] for other SQL failures.
pub fn append_event_in_tx(
    tx: &mut Transaction<'_>,
    event: &StoredEvent,
    idempotency_key: Option<&[u8]>,
) -> Result<(), PostgresStoreError> {
    if let Some(key) = idempotency_key
        && (key.is_empty() || key.len() > IDEMPOTENCY_KEY_MAX_LEN)
    {
        return Err(PostgresStoreError::new(
            PostgresStoreErrorKind::IdempotencyKeyTooLong,
            format!(
                "idempotency_key length {} outside Core §6.1 bound 1..=64",
                key.len()
            ),
        ));
    }

    let sequence = i64::try_from(event.sequence()).map_err(|_| {
        PostgresStoreError::new(
            PostgresStoreErrorKind::DomainViolation,
            format!(
                "sequence `{}` does not fit into Postgres BIGINT",
                event.sequence()
            ),
        )
    })?;

    let scope = event.scope();
    let canonical = event.canonical_event();
    let signed = event.signed_event();

    let result = tx.execute(
        "\
INSERT INTO trellis_events (scope, sequence, canonical_event, signed_event, idempotency_key) \
VALUES ($1, $2, $3, $4, $5)",
        &[&scope, &sequence, &canonical, &signed, &idempotency_key],
    );

    match result {
        Ok(_) => Ok(()),
        Err(error) => {
            if is_unique_violation(&error, "trellis_events_scope_idempotency_uidx") {
                return Err(PostgresStoreError::new(
                    PostgresStoreErrorKind::IdempotencyKeyPayloadMismatch,
                    format!(
                        "Core §17.3 idempotency-key conflict on \
                         (ledger_scope, idempotency_key): {error}"
                    ),
                ));
            }
            Err(PostgresStoreError::new(
                PostgresStoreErrorKind::QueryFailed,
                format!("failed to append Trellis event: {error}"),
            ))
        }
    }
}

fn is_unique_violation(error: &postgres::Error, constraint: &str) -> bool {
    let Some(db_err) = error.as_db_error() else {
        return false;
    };
    if db_err.code() != &postgres::error::SqlState::UNIQUE_VIOLATION {
        return false;
    }
    db_err.constraint() == Some(constraint)
}

/// Returns `Ok(())` when the DSN names a loopback host (or Unix socket directory),
/// otherwise [`PostgresStoreErrorKind::UnsafeDsn`].
///
/// The check is conservative: it parses `key=value` pairs from a libpq-style
/// connection string and inspects the `host` parameter. URI-style DSNs
/// (`postgres://...`) are rejected unless the host portion is loopback.
/// Anything ambiguous (multiple hosts, env-var lookups, target_session_attrs
/// games) is rejected — production callers must use [`PostgresStore::connect_with_tls`].
fn require_loopback_dsn(dsn: &str) -> Result<(), PostgresStoreError> {
    let host = extract_dsn_host(dsn);
    if is_loopback_host(host.as_deref()) {
        Ok(())
    } else {
        Err(PostgresStoreError::new(
            PostgresStoreErrorKind::UnsafeDsn,
            format!(
                "PostgresStore::connect refused non-loopback host {:?}; \
                 use connect_with_tls for production deployments",
                host.as_deref().unwrap_or("<unset>")
            ),
        ))
    }
}

fn extract_dsn_host(dsn: &str) -> Option<String> {
    let trimmed = dsn.trim();
    if let Some(rest) = trimmed
        .strip_prefix("postgres://")
        .or_else(|| trimmed.strip_prefix("postgresql://"))
    {
        // <user[:pass]@>host[:port]/<dbname>?<params>
        let after_auth = rest.rsplit_once('@').map(|(_, h)| h).unwrap_or(rest);
        let host_port = after_auth.split(['/', '?']).next().unwrap_or("");
        // Bracketed IPv6 hosts: `[::1]` or `[::1]:5432`. The bracketed
        // substring is the host; anything after `]:` is the port. A naive
        // `rsplit_once(':')` would slice the literal IPv6 internally
        // (e.g. `[::1]` -> host=`[:`, port=`1]`).
        let host = if let Some(after_open) = host_port.strip_prefix('[') {
            match after_open.split_once(']') {
                Some((ipv6, _rest)) => ipv6,
                // Mismatched bracket — fall back to the whole substring; the
                // loopback classifier will reject anything non-trivial.
                None => host_port,
            }
        } else {
            host_port
                .rsplit_once(':')
                .map(|(h, _)| h)
                .unwrap_or(host_port)
        };
        return Some(host.to_string());
    }

    let mut host: Option<String> = None;
    for token in trimmed.split_whitespace() {
        if let Some(value) = token.strip_prefix("host=") {
            // Last `host=` wins per libpq.
            host = Some(value.to_string());
        }
    }
    host
}

fn is_loopback_host(host: Option<&str>) -> bool {
    let Some(host) = host else {
        // No host parameter — libpq defaults to local Unix socket: safe.
        return true;
    };
    if host.is_empty() {
        return true;
    }
    // Unix socket directory: starts with `/`.
    if host.starts_with('/') {
        return true;
    }
    if host.eq_ignore_ascii_case("localhost") {
        return true;
    }
    // IPv4 / IPv6 loopback literal parsing.
    if let Ok(addr) = host.parse::<std::net::IpAddr>() {
        return addr.is_loopback();
    }
    false
}

/// `r2d2`-managed connection pool for [`PostgresStore`].
///
/// Production deployments need a pool — opening a fresh `Client` per
/// request wastes ports and Postgres backend processes. Sync `r2d2` here
/// matches `postgres = "0.19"`'s sync model; `deadpool-postgres` is
/// async-only and would force a tokio runtime into Trellis (which Trellis
/// has not adopted). If async-Trellis is later chosen, swap behind the
/// same surface.
///
/// # Sizing guidance
///
/// - Default `max_size = 16`. Tune with [`PoolBuilder::max_size`].
/// - Postgres `max_connections` defaults to 100; for `N` server replicas
///   keep `N * max_size < max_connections - reserved_admin`.
/// - For Federal/Sovereign workloads with heavier write parallelism,
///   raise to 32 only after observing pool-acquire wait time. Postgres
///   backend processes are not free; more connections is not faster.
/// - `connection_timeout = 30s` is the r2d2 default; tune when
///   pool-acquire latency shows up as p99 tail.
pub struct PostgresStorePool {
    inner: InnerPool,
}

enum InnerPool {
    NoTls(Pool<PostgresConnectionManager<NoTls>>),
    NativeTls(Pool<PostgresConnectionManager<MakeTlsConnector>>),
}

impl InnerPool {
    fn max_size(&self) -> u32 {
        match self {
            InnerPool::NoTls(p) => p.max_size(),
            InnerPool::NativeTls(p) => p.max_size(),
        }
    }
}

/// Builder for [`PostgresStorePool`].
pub struct PoolBuilder {
    connection_string: String,
    tls: TlsChoice,
    max_size: u32,
    connection_timeout: Duration,
}

enum TlsChoice {
    NoTls,
    NativeTls(MakeTlsConnector),
}

impl PoolBuilder {
    /// Sets the maximum pool size. Default is 16.
    pub fn max_size(mut self, n: u32) -> Self {
        self.max_size = n;
        self
    }

    /// Sets the per-acquire connection timeout. Default is 30s.
    pub fn connection_timeout(mut self, timeout: Duration) -> Self {
        self.connection_timeout = timeout;
        self
    }

    /// Builds the pool, applying migrations on the first checkout.
    ///
    /// # Errors
    /// - [`PostgresStoreErrorKind::UnsafeDsn`] when the NoTls path is paired with a non-loopback DSN.
    /// - [`PostgresStoreErrorKind::PoolFailed`] when r2d2 cannot build the pool.
    /// - [`PostgresStoreErrorKind::MigrationFailed`] when migrations on first checkout fail.
    pub fn build(self) -> Result<PostgresStorePool, PostgresStoreError> {
        let config: postgres::Config = self.connection_string.parse().map_err(|error| {
            PostgresStoreError::new(
                PostgresStoreErrorKind::ConnectionFailed,
                format!("invalid Postgres connection string: {error}"),
            )
        })?;

        let inner = match self.tls {
            TlsChoice::NoTls => {
                require_loopback_dsn(&self.connection_string)?;
                let manager = PostgresConnectionManager::new(config, NoTls);
                let pool = Pool::builder()
                    .max_size(self.max_size)
                    .connection_timeout(self.connection_timeout)
                    .build(manager)
                    .map_err(|error| {
                        PostgresStoreError::new(
                            PostgresStoreErrorKind::PoolFailed,
                            format!("failed to build Postgres pool: {error}"),
                        )
                    })?;
                let mut conn = pool.get().map_err(|error| {
                    PostgresStoreError::new(
                        PostgresStoreErrorKind::PoolFailed,
                        format!("failed to acquire initial pooled connection: {error}"),
                    )
                })?;
                migrations::apply(&mut conn)?;
                InnerPool::NoTls(pool)
            }
            TlsChoice::NativeTls(connector) => {
                let manager = PostgresConnectionManager::new(config, connector);
                let pool = Pool::builder()
                    .max_size(self.max_size)
                    .connection_timeout(self.connection_timeout)
                    .build(manager)
                    .map_err(|error| {
                        PostgresStoreError::new(
                            PostgresStoreErrorKind::PoolFailed,
                            format!("failed to build Postgres pool: {error}"),
                        )
                    })?;
                let mut conn = pool.get().map_err(|error| {
                    PostgresStoreError::new(
                        PostgresStoreErrorKind::PoolFailed,
                        format!("failed to acquire initial pooled connection: {error}"),
                    )
                })?;
                migrations::apply(&mut conn)?;
                InnerPool::NativeTls(pool)
            }
        };

        Ok(PostgresStorePool { inner })
    }
}

impl PostgresStorePool {
    /// Starts a builder targeting a loopback DSN with `NoTls`.
    ///
    /// **Refuses non-loopback hosts at `build` time** — use [`Self::builder_with_tls`]
    /// for non-localhost deployments.
    pub fn builder(connection_string: impl Into<String>) -> PoolBuilder {
        PoolBuilder {
            connection_string: connection_string.into(),
            tls: TlsChoice::NoTls,
            max_size: 16,
            connection_timeout: Duration::from_secs(30),
        }
    }

    /// Starts a builder using the supplied [`TlsConnector`] — production deployments.
    pub fn builder_with_tls(
        connection_string: impl Into<String>,
        tls: TlsConnector,
    ) -> PoolBuilder {
        PoolBuilder {
            connection_string: connection_string.into(),
            tls: TlsChoice::NativeTls(MakeTlsConnector::new(tls)),
            max_size: 16,
            connection_timeout: Duration::from_secs(30),
        }
    }

    /// Returns the configured maximum pool size.
    pub fn max_size(&self) -> u32 {
        self.inner.max_size()
    }

    /// Acquires a connection from the pool.
    ///
    /// The returned [`PooledClient`] dereferences mutably to a
    /// `postgres::Client`; callers obtain a transaction via
    /// [`PostgresStorePool::with_transaction`] or by calling
    /// `client.transaction()` directly, then pass `&mut tx` to
    /// [`append_event_in_tx`].
    ///
    /// # Errors
    /// Returns [`PostgresStoreErrorKind::PoolFailed`] when no connection is
    /// available before the configured timeout.
    pub fn checkout(&self) -> Result<PooledClient, PostgresStoreError> {
        match &self.inner {
            InnerPool::NoTls(pool) => pool.get().map(PooledClient::NoTls).map_err(|error| {
                PostgresStoreError::new(
                    PostgresStoreErrorKind::PoolFailed,
                    format!("failed to acquire pooled Postgres connection: {error}"),
                )
            }),
            InnerPool::NativeTls(pool) => {
                pool.get().map(PooledClient::NativeTls).map_err(|error| {
                    PostgresStoreError::new(
                        PostgresStoreErrorKind::PoolFailed,
                        format!("failed to acquire pooled Postgres connection: {error}"),
                    )
                })
            }
        }
    }
}

/// Pooled connection handle.
///
/// Internally tagged by TLS variant; both arms expose the same `postgres::Client`
/// surface via [`PooledClient::client`] / [`PooledClient::client_mut`].
pub enum PooledClient {
    /// Loopback-only `NoTls` arm.
    NoTls(PooledConnection<PostgresConnectionManager<NoTls>>),
    /// `native_tls`-secured arm.
    NativeTls(PooledConnection<PostgresConnectionManager<MakeTlsConnector>>),
}

impl PooledClient {
    /// Returns a shared reference to the underlying `postgres::Client`.
    pub fn client(&self) -> &Client {
        match self {
            PooledClient::NoTls(c) => c,
            PooledClient::NativeTls(c) => c,
        }
    }

    /// Returns a mutable reference to the underlying `postgres::Client`.
    pub fn client_mut(&mut self) -> &mut Client {
        match self {
            PooledClient::NoTls(c) => c,
            PooledClient::NativeTls(c) => c,
        }
    }
}

impl std::ops::Deref for PooledClient {
    type Target = Client;
    fn deref(&self) -> &Client {
        self.client()
    }
}

impl std::ops::DerefMut for PooledClient {
    fn deref_mut(&mut self) -> &mut Client {
        self.client_mut()
    }
}

impl std::fmt::Debug for PostgresStorePool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PostgresStorePool")
            .field("max_size", &self.inner.max_size())
            .finish_non_exhaustive()
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

    use super::{
        IDEMPOTENCY_KEY_MAX_LEN, PostgresStore, PostgresStoreErrorKind, PostgresStorePool,
        append_event_in_tx, extract_dsn_host, is_loopback_host, require_loopback_dsn,
    };
    use trellis_core::LedgerStore;
    use trellis_types::StoredEvent;

    // ----- Pure unit tests (no Postgres) ---------------------------------

    #[test]
    fn loopback_host_classification_recognizes_safe_hosts() {
        assert!(is_loopback_host(None));
        assert!(is_loopback_host(Some("")));
        assert!(is_loopback_host(Some("localhost")));
        assert!(is_loopback_host(Some("LOCALHOST")));
        assert!(is_loopback_host(Some("127.0.0.1")));
        assert!(is_loopback_host(Some("127.0.0.5")));
        assert!(is_loopback_host(Some("::1")));
        assert!(is_loopback_host(Some("/var/run/postgresql")));

        assert!(!is_loopback_host(Some("10.0.0.1")));
        assert!(!is_loopback_host(Some("db.internal.example.com")));
        assert!(!is_loopback_host(Some("203.0.113.5")));
        assert!(!is_loopback_host(Some("2001:db8::1")));
    }

    #[test]
    fn dsn_host_extraction_handles_kv_and_uri_forms() {
        assert_eq!(
            extract_dsn_host("host=127.0.0.1 port=5432 user=postgres").as_deref(),
            Some("127.0.0.1")
        );
        assert_eq!(
            extract_dsn_host("postgres://u:p@db.example.com:5432/dbn").as_deref(),
            Some("db.example.com")
        );
        assert_eq!(
            extract_dsn_host("postgresql://localhost/dbn").as_deref(),
            Some("localhost")
        );
        assert_eq!(
            extract_dsn_host("postgres://[::1]:5432/dbn").as_deref(),
            Some("::1")
        );
        // No host param → libpq uses local socket → host_extraction returns None,
        // and require_loopback_dsn must accept this.
        assert_eq!(extract_dsn_host("user=postgres dbname=postgres"), None);
    }

    #[test]
    fn require_loopback_dsn_rejects_remote_hosts() {
        assert!(matches!(
            require_loopback_dsn("host=db.internal.example.com user=postgres")
                .unwrap_err()
                .kind(),
            PostgresStoreErrorKind::UnsafeDsn
        ));
        assert!(matches!(
            require_loopback_dsn("postgres://u:p@10.0.0.5/dbn")
                .unwrap_err()
                .kind(),
            PostgresStoreErrorKind::UnsafeDsn
        ));
    }

    #[test]
    fn require_loopback_dsn_accepts_loopback_and_socket_paths() {
        require_loopback_dsn("host=127.0.0.1 port=5432 user=postgres").unwrap();
        require_loopback_dsn("host=::1 user=postgres").unwrap();
        require_loopback_dsn("host=localhost user=postgres").unwrap();
        require_loopback_dsn("host=/var/run/postgresql user=postgres").unwrap();
        require_loopback_dsn("postgres://u:p@localhost/dbn").unwrap();
        // No host parameter → libpq local socket fallback — accepted.
        require_loopback_dsn("user=postgres dbname=postgres").unwrap();
    }

    /// Comma-separated host list: libpq supports multi-host failover, but the
    /// classifier compares the literal value against loopback markers. A
    /// list like `host=localhost,db.example.com` does NOT match `"localhost"`
    /// exactly nor parse as an IP — so the gate rejects it. False-negative
    /// (a list that *would* prefer `localhost` first is still rejected) is
    /// the safe direction; production multi-host setups must use
    /// `connect_with_tls`.
    #[test]
    fn require_loopback_dsn_rejects_comma_separated_host_list() {
        // Two non-loopback names — obvious reject.
        assert!(matches!(
            require_loopback_dsn("host=a,b user=postgres")
                .unwrap_err()
                .kind(),
            PostgresStoreErrorKind::UnsafeDsn
        ));
        // First entry would be loopback if parsed individually — still rejected
        // because the literal compare does not split on `,`. Conservative-correct.
        assert!(matches!(
            require_loopback_dsn("host=localhost,db.example.com user=postgres")
                .unwrap_err()
                .kind(),
            PostgresStoreErrorKind::UnsafeDsn
        ));
        assert!(matches!(
            require_loopback_dsn("host=127.0.0.1,10.0.0.5 user=postgres")
                .unwrap_err()
                .kind(),
            PostgresStoreErrorKind::UnsafeDsn
        ));
    }

    /// Empty-string `host=` is libpq's local-socket fallback (same as omitting
    /// the parameter entirely). Accept it — the wire is the local AF_UNIX
    /// socket per the existing comment in `is_loopback_host`.
    #[test]
    fn require_loopback_dsn_accepts_empty_host_value() {
        require_loopback_dsn("host= user=postgres dbname=postgres").unwrap();
        // Just `host=` with no other tokens.
        require_loopback_dsn("host=").unwrap();
    }

    /// Relative-path "Unix socket" attempt — libpq's `host` parameter for a
    /// socket directory is documented to be absolute. A relative path like
    /// `tmp/sock` is not a loopback marker (does not start with `/`, is not
    /// `localhost`, does not parse as an IP) — reject. libpq itself would
    /// fail to dial, but the safety gate is "no cleartext on a wire" first;
    /// rejecting at the constructor is the lowest-debt enforcement seam.
    #[test]
    fn require_loopback_dsn_rejects_relative_socket_paths() {
        assert!(matches!(
            require_loopback_dsn("host=relative/path user=postgres")
                .unwrap_err()
                .kind(),
            PostgresStoreErrorKind::UnsafeDsn
        ));
        assert!(matches!(
            require_loopback_dsn("host=./sock user=postgres")
                .unwrap_err()
                .kind(),
            PostgresStoreErrorKind::UnsafeDsn
        ));
        assert!(matches!(
            require_loopback_dsn("host=tmp user=postgres")
                .unwrap_err()
                .kind(),
            PostgresStoreErrorKind::UnsafeDsn
        ));
    }

    /// Reaffirm IPv6 loopback acceptance in both DSN forms — kv-style
    /// `host=::1` (already covered above) and URI-style `postgres://[::1]/dbn`.
    /// Other IPv6 addresses (including documentation-only `2001:db8::1`)
    /// remain rejected.
    #[test]
    fn require_loopback_dsn_handles_ipv6_loopback_in_both_dsn_forms() {
        require_loopback_dsn("host=::1 user=postgres").unwrap();
        require_loopback_dsn("postgres://u:p@[::1]:5432/dbn").unwrap();
        require_loopback_dsn("postgresql://[::1]/dbn").unwrap();
        assert!(matches!(
            require_loopback_dsn("postgres://u:p@[2001:db8::1]/dbn")
                .unwrap_err()
                .kind(),
            PostgresStoreErrorKind::UnsafeDsn
        ));
    }

    #[test]
    fn idempotency_key_max_len_matches_core_61() {
        // Sanity peg on the published Core §6.1 / §17.2 bound.
        assert_eq!(IDEMPOTENCY_KEY_MAX_LEN, 64);
    }

    // ----- Integration tests (TestCluster) -------------------------------

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

    #[test]
    fn migrations_apply_idempotently_across_reconnects() {
        let cluster = TestCluster::start();
        // First connect — applies all migrations.
        let _store_one = PostgresStore::connect(&cluster.connection_string()).unwrap();
        // Second connect — must not double-apply or fail.
        let _store_two = PostgresStore::connect(&cluster.connection_string()).unwrap();

        // Inspect the migrations table — should record both v1 + v2.
        let mut probe = PostgresStore::connect(&cluster.connection_string()).unwrap();
        let rows = probe
            .client
            .query(
                "SELECT version FROM trellis_schema_migrations ORDER BY version",
                &[],
            )
            .unwrap();
        let versions: Vec<i32> = rows.iter().map(|r| r.get::<_, i32>("version")).collect();
        assert_eq!(versions, vec![1, 2]);
    }

    /// Refuse-on-future-version guard: if the database records a migration
    /// version higher than anything this binary declares, `apply` MUST refuse
    /// rather than silently no-op. "Append-only migrations" is convention;
    /// this asserts the runtime surface that converts the convention into
    /// a hard failure when an older binary connects to a forward-rolled
    /// schema (e.g. during a botched rollback).
    #[test]
    fn migrations_refuse_when_schema_ahead_of_binary() {
        let cluster = TestCluster::start();
        // First connect lands the declared migrations cleanly.
        let _bootstrap = PostgresStore::connect(&cluster.connection_string()).unwrap();

        // Forge a future-version row so the next `apply` sees a schema ahead
        // of MIGRATIONS' max declared version.
        {
            let mut probe = PostgresStore::connect(&cluster.connection_string()).unwrap();
            probe
                .client
                .execute(
                    "INSERT INTO trellis_schema_migrations (version) VALUES ($1)",
                    &[&999_i32],
                )
                .unwrap();
        }

        // Reconnect — `migrations::apply` must refuse with MigrationFailed.
        let err = PostgresStore::connect(&cluster.connection_string()).unwrap_err();
        assert_eq!(err.kind(), PostgresStoreErrorKind::MigrationFailed);
        let msg = err.to_string();
        assert!(
            msg.contains("schema ahead of binary"),
            "expected 'schema ahead of binary' in error message, got: {msg}"
        );
        assert!(
            msg.contains("v999"),
            "expected applied version v999 in error message, got: {msg}"
        );
    }

    #[test]
    fn idempotency_key_unique_index_rejects_duplicates() {
        let cluster = TestCluster::start();
        let mut store = PostgresStore::connect(&cluster.connection_string()).unwrap();

        let event = StoredEvent::new(b"scope-z".to_vec(), 0, vec![0xaa], vec![0xbb]);
        let key = b"idem-key-001".to_vec();
        let mut tx = store.begin().unwrap();
        append_event_in_tx(&mut tx, &event, Some(&key)).unwrap();
        tx.commit().unwrap();

        let event_two = StoredEvent::new(b"scope-z".to_vec(), 1, vec![0xcc], vec![0xdd]);
        let mut tx2 = store.begin().unwrap();
        let err = append_event_in_tx(&mut tx2, &event_two, Some(&key)).unwrap_err();
        assert_eq!(
            err.kind(),
            PostgresStoreErrorKind::IdempotencyKeyPayloadMismatch,
        );
    }

    #[test]
    fn idempotency_key_unique_index_allows_distinct_keys_in_same_scope() {
        let cluster = TestCluster::start();
        let mut store = PostgresStore::connect(&cluster.connection_string()).unwrap();

        let mut tx = store.begin().unwrap();
        append_event_in_tx(
            &mut tx,
            &StoredEvent::new(b"scope-y".to_vec(), 0, vec![0x01], vec![0x02]),
            Some(b"key-A"),
        )
        .unwrap();
        append_event_in_tx(
            &mut tx,
            &StoredEvent::new(b"scope-y".to_vec(), 1, vec![0x03], vec![0x04]),
            Some(b"key-B"),
        )
        .unwrap();
        tx.commit().unwrap();
    }

    #[test]
    fn idempotency_key_partial_index_allows_multiple_null_keys() {
        // Phase-1 callers pass `None`; the partial index must NOT fire on NULLs.
        let cluster = TestCluster::start();
        let mut store = PostgresStore::connect(&cluster.connection_string()).unwrap();

        store
            .append_event(StoredEvent::new(
                b"scope-w".to_vec(),
                0,
                vec![0x01],
                vec![0x02],
            ))
            .unwrap();
        store
            .append_event(StoredEvent::new(
                b"scope-w".to_vec(),
                1,
                vec![0x03],
                vec![0x04],
            ))
            .unwrap();
    }

    #[test]
    fn idempotency_key_too_long_rejected_at_input() {
        let cluster = TestCluster::start();
        let mut store = PostgresStore::connect(&cluster.connection_string()).unwrap();

        let oversize = vec![0x42; IDEMPOTENCY_KEY_MAX_LEN + 1];
        let mut tx = store.begin().unwrap();
        let err = append_event_in_tx(
            &mut tx,
            &StoredEvent::new(b"scope-x".to_vec(), 0, vec![], vec![]),
            Some(&oversize),
        )
        .unwrap_err();
        assert_eq!(err.kind(), PostgresStoreErrorKind::IdempotencyKeyTooLong);

        let empty: Vec<u8> = Vec::new();
        let err = append_event_in_tx(
            &mut tx,
            &StoredEvent::new(b"scope-x".to_vec(), 0, vec![], vec![]),
            Some(&empty),
        )
        .unwrap_err();
        assert_eq!(err.kind(), PostgresStoreErrorKind::IdempotencyKeyTooLong);
    }

    #[test]
    fn transaction_composition_rolls_back_canonical_with_caller_failure() {
        // The load-bearing wos-server seam: caller writes canonical event AND
        // a projection update inside one transaction; if the projection write
        // fails, the canonical event MUST roll back too.
        let cluster = TestCluster::start();
        let mut store = PostgresStore::connect(&cluster.connection_string()).unwrap();

        // Caller-owned projection table.
        store
            .client
            .batch_execute("CREATE TABLE projections_test (id BIGINT PRIMARY KEY)")
            .unwrap();

        let event = StoredEvent::new(b"scope-tx".to_vec(), 0, vec![0xff], vec![0xee]);
        let mut tx = store.begin().unwrap();
        append_event_in_tx(&mut tx, &event, None).unwrap();
        // First projection insert succeeds.
        tx.execute("INSERT INTO projections_test (id) VALUES ($1)", &[&1_i64])
            .unwrap();
        // Second insert violates the primary key; tx returns Err; we explicitly
        // do NOT commit and let `tx` drop, which rolls back.
        let projection_err = tx.execute("INSERT INTO projections_test (id) VALUES ($1)", &[&1_i64]);
        assert!(projection_err.is_err());
        drop(tx);

        // The canonical event MUST NOT be visible.
        let events = store.load_scope_events(b"scope-tx").unwrap();
        assert!(
            events.is_empty(),
            "rolled-back tx left canonical event visible: {events:?}"
        );
    }

    #[test]
    fn transaction_composition_commits_canonical_with_caller_projection() {
        let cluster = TestCluster::start();
        let mut store = PostgresStore::connect(&cluster.connection_string()).unwrap();

        store
            .client
            .batch_execute("CREATE TABLE projections_ok (id BIGINT PRIMARY KEY)")
            .unwrap();

        let event = StoredEvent::new(b"scope-ok".to_vec(), 5, vec![0x11], vec![0x22]);
        let mut tx = store.begin().unwrap();
        append_event_in_tx(&mut tx, &event, None).unwrap();
        tx.execute("INSERT INTO projections_ok (id) VALUES ($1)", &[&42_i64])
            .unwrap();
        tx.commit().unwrap();

        let events = store.load_scope_events(b"scope-ok").unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].sequence(), 5);

        let row = store
            .client
            .query_one("SELECT id FROM projections_ok WHERE id = $1", &[&42_i64])
            .unwrap();
        assert_eq!(row.get::<_, i64>("id"), 42);
    }

    #[test]
    fn connect_refuses_non_loopback_dsn() {
        // No cluster needed — the safety gate is pre-connection.
        let err = PostgresStore::connect("host=db.internal.example.com user=postgres").unwrap_err();
        assert_eq!(err.kind(), PostgresStoreErrorKind::UnsafeDsn);
    }

    #[test]
    fn pool_builder_refuses_non_loopback_without_tls() {
        let err = PostgresStorePool::builder("host=db.example.com user=postgres")
            .build()
            .unwrap_err();
        assert_eq!(err.kind(), PostgresStoreErrorKind::UnsafeDsn);
    }

    #[test]
    fn pool_checkout_writes_and_reads_events() {
        let cluster = TestCluster::start();
        let pool = PostgresStorePool::builder(cluster.connection_string())
            .max_size(4)
            .build()
            .unwrap();
        assert_eq!(pool.max_size(), 4);

        let mut conn = pool.checkout().unwrap();
        let mut tx = conn.transaction().unwrap();
        append_event_in_tx(
            &mut tx,
            &StoredEvent::new(b"scope-pool".to_vec(), 0, vec![0x77], vec![0x88]),
            None,
        )
        .unwrap();
        tx.commit().unwrap();

        let row = conn
            .query_one(
                "SELECT canonical_event FROM trellis_events WHERE scope = $1",
                &[&b"scope-pool".to_vec()],
            )
            .unwrap();
        let canonical: Vec<u8> = row.get("canonical_event");
        assert_eq!(canonical, vec![0x77]);
    }

    // ----- TestCluster (unchanged from baseline) -------------------------

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
