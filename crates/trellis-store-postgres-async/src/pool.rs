//! Async Postgres pool construction.

use std::time::Duration;

use sqlx::PgPool;
use stack_common_postgres::{PoolConfig, build_sqlx_pool_with_acquire_timeout};

/// Error returned when async Postgres pool setup fails.
#[derive(Debug, thiserror::Error)]
pub enum PoolError {
    /// SQLx could not establish the pool.
    #[error("sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),
}

/// Builds a SQLx Postgres pool.
///
/// The caller chooses the connection string, including TLS mode. Production
/// callers should use a TLS-enforcing `PgConnectOptions` when they need
/// stricter configuration than URL parameters provide.
///
/// # Errors
///
/// Returns [`PoolError::Sqlx`] when SQLx cannot connect or initialize the pool.
pub async fn build_pool(connection_url: &str, max_connections: u32) -> Result<PgPool, PoolError> {
    let config = PoolConfig::programmatic(connection_url, max_connections);
    build_sqlx_pool_with_acquire_timeout(&config, Duration::from_secs(10))
        .await
        .map_err(PoolError::Sqlx)
}
