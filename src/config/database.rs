use crate::errors::AppError;

use sqlx::{
    PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use std::time::Duration;
use std::str::FromStr;
use tracing::{info, warn, error};
use anyhow::{Context, anyhow};
use tokio::time::sleep;

#[derive(Clone, Debug)]
pub struct DbConfig {
    pub database_url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout: Duration,
    pub acquire_timeout: Duration,
    pub idle_timeout: Option<Duration>,
    pub max_lifetime: Option<Duration>,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            // FIX: Added .expect() to get the String value.
            // Panics if DATABASE_URL is not set in the environment.
            database_url: std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            // FIX: Added missing commas
            max_connections: 10,
            min_connections: 1,
            connection_timeout: Duration::from_secs(5),
            acquire_timeout: Duration::from_secs(5),
            idle_timeout: Some(Duration::from_secs(300)), // 5 mins
            max_lifetime: Some(Duration::from_secs(1800)), // 30 mins
        }
    }
}

/// Create a PgPool using the supplied config.
/// Returns `Ok(PgPool)` or `Err(AppError::Internal)` on failure.
pub async fn create_db_pool(cfg: &DbConfig) -> Result<PgPool, AppError> {
    // load env in dev if present
    let _ = dotenv::dotenv(); // This is syntactically valid, though .ok() is common

    info!(
        "initializing db pool; min={}, max={}, url={}",
        cfg.min_connections,
        cfg.max_connections,
        cfg.database_url
    );

    let connect_opts = PgConnectOptions::from_str(&cfg.database_url)
        .context("parsing DATABASE_URL into PgConnectOptions")
        .map_err(|e| AppError::Internal(anyhow!(e)))?;

    // simple retry loop (useful if DB isn't ready at app start)
    let mut last_err: Option<AppError> = None;
    let max_retries = 3u32;

    for attempt in 0..=max_retries {
        if attempt > 0 {
            let backoff_ms = 100u64.saturating_mul(1u64 << (attempt - 1));
            warn!("db connect retry {}/{} - sleeping {}ms before retry", attempt, max_retries, backoff_ms);
            sleep(Duration::from_millis(backoff_ms)).await;
        }

        let mut builder = PgPoolOptions::new()
            .max_connections(cfg.max_connections)
            .min_connections(cfg.min_connections)
            .connect_timeout(cfg.connection_timeout)
            .acquire_timeout(cfg.acquire_timeout)
            .test_before_acquire(true);

        if let Some(idle) = cfg.idle_timeout {
            builder = builder.idle_timeout(idle);
        }
        if let Some(max_l) = cfg.max_lifetime {
            builder = builder.max_lifetime(max_l);
        }

        match builder.connect_with(connect_opts.clone()).await {
            Ok(pool) => {
                info!("db pool connected successfully (attempt {})", attempt);

                // quick validation to ensure queries can be executed
                match sqlx::query_scalar::<_, i32>("SELECT 1").fetch_one(&pool).await {
                    Ok(1) => return Ok(pool),
                    Ok(v) => {
                        let err = AppError::Internal(anyhow!("validation query returned unexpected value: {}", v));
                        error!("{:#?}", err);
                        last_err = Some(err);
                        // fallthrough to retry
                    }
                    Err(e) => {
                        let err = AppError::Internal(anyhow!(e));
                        error!("validation query failed: {:#?}", err);
                        last_err = Some(err);
                        // fallthrough to retry
                    }
                }
            }
            Err(e) => {
                let err = AppError::Internal(anyhow!(e));
                error!("failed to create db pool on attempt {}: {:#?}", attempt, err);
                last_err = Some(err);
            }
        }
    }

    Err(last_err.unwrap_or_else(|| AppError::Internal(anyhow!("exhausted pool connect retries"))))
}