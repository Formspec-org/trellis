//! Async migration runner for the Trellis Postgres schema.

use sqlx::PgPool;
use stack_common_postgres::{MigrationSet, run_sqlx_migrations};
use trellis_store_postgres_shared::migrations::{ADVISORY_LOCK_KEY, MIGRATIONS};

const LEDGER_TABLE: &str = "trellis_schema_migrations";

/// Error returned by async migration setup.
#[derive(Debug, thiserror::Error)]
pub enum MigrationError {
    /// Database schema has a migration version this binary does not know.
    #[error(
        "schema ahead of binary: applied version {applied_version}, declared max {declared_max}"
    )]
    SchemaAhead {
        /// Highest version recorded in the database.
        applied_version: i32,
        /// Highest version declared by this binary.
        declared_max: i32,
    },
    /// SQL execution failed while applying or recording migrations.
    #[error("sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),
    /// Shared migration runner rejected the request.
    #[error("shared migration runner: {0}")]
    Shared(String),
}

impl From<stack_common_postgres::MigrationError> for MigrationError {
    fn from(error: stack_common_postgres::MigrationError) -> Self {
        match error {
            stack_common_postgres::MigrationError::SchemaAhead {
                applied_version,
                declared_max,
            } => Self::SchemaAhead {
                applied_version,
                declared_max,
            },
            stack_common_postgres::MigrationError::Sqlx(error) => Self::Sqlx(error),
            other => Self::Shared(other.to_string()),
        }
    }
}

/// Applies pending Trellis Postgres migrations.
///
/// The migration list and advisory-lock key come from the shared Postgres contract.
/// Rows are append-only: old migration SQL must not be edited after landing.
///
/// # Errors
///
/// Returns [`MigrationError::SchemaAhead`] when the database records a newer
/// migration than this binary declares, or [`MigrationError::Sqlx`] for SQL
/// execution failures.
pub async fn run_migrations(pool: &PgPool) -> Result<(), MigrationError> {
    run_sqlx_migrations(
        pool,
        MigrationSet::new(LEDGER_TABLE, ADVISORY_LOCK_KEY, MIGRATIONS),
    )
    .await?;
    Ok(())
}
