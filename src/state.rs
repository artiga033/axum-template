use axum::extract::FromRef;
#[allow(unused)]
use snafu::ResultExt as _;
use snafu::Snafu;

#[derive(Clone, FromRef)]
pub struct AppState {
    #[cfg(feature = "example")]
    pub sqlite_pool: sqlx::SqlitePool,
}

impl AppState {
    pub async fn from_config(config: &crate::config::Config) -> Result<Self> {
        #[cfg(feature = "example")]
        {
            let sqlite_pool = sqlx::SqlitePool::connect(&config.bootstrap.database_url)
                .await
                .context(ConnectDatabaseSnafu)?;
            Ok(AppState { sqlite_pool })
        }
        #[cfg(not(feature = "example"))]
        {
            _ = config;
            Ok(AppState {})
        }
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[cfg(feature = "example")]
    #[snafu(display("connect to database: {}", source))]
    ConnectDatabase { source: sqlx::Error },
}

type Result<T> = std::result::Result<T, Error>;
