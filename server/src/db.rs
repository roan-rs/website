use std::time::Duration;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool;
use crate::var::required_var;
use anyhow::Result;
use deadpool_diesel::Runtime;
use diesel_async::AsyncPgConnection;

pub const MAX_DB_CONNECTIONS: usize = 4;
pub const WAIT_TIMEOUT: Duration = Duration::from_secs(30);

pub fn create_db_connection() -> Result<Pool<AsyncPgConnection>> {
    let db_url = required_var("DATABASE_URL")?;

    let manager = AsyncDieselConnectionManager::new(db_url);

    Ok(
        Pool::builder(manager)
            .wait_timeout(Some(WAIT_TIMEOUT))
            .max_size(MAX_DB_CONNECTIONS)
            .runtime(Runtime::Tokio1)
            .build()?
    )
}