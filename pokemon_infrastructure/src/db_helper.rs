use std::env;
use std::time::Duration;

use diesel::prelude::Connection;
use once_cell::sync::Lazy;

use crate::{PgConnection, PgConnectionManager, PgConnectionPool};

/// Lazy load `DATABASE_URL` from .env
pub static DATABASE_URL: Lazy<String> =
  Lazy::new(|| env::var("DATABASE_URL").expect("Missing database url"));

/// Lazy config `CONNECTION_POOL` with `CONNECTION_COUNT` from .env
pub static CONNECTION_POOL: Lazy<PgConnectionPool> = Lazy::new(|| {
  let max_size = env::var("CONNECTION_COUNT")
    .ok()
    .and_then(|s| s.parse().ok())
    .unwrap_or(1u32);

  let database_url = DATABASE_URL.clone();

  establish_connection_pool(&database_url, max_size)
});

/// Create a db connection by `database_url`
pub fn establish_connection(database_url: &str) -> PgConnection {
  match PgConnection::establish(database_url) {
    Ok(conn) => conn,
    Err(err) => {
      println!("conn err: {:?}", err);
      panic!("Can't establish database {}", database_url)
    }
  }
}

/// Create a db connection pool
pub fn establish_connection_pool(database_url: &str, max_size: u32) -> PgConnectionPool {
  const MIN_IDLE_COUNT: u32 = 2;

  let max_size = if max_size < MIN_IDLE_COUNT {
    MIN_IDLE_COUNT
  } else {
    max_size
  };

  r2d2::Pool::builder()
    .max_size(max_size)
    .min_idle(Some(MIN_IDLE_COUNT))
    .max_lifetime(Some(Duration::from_secs(60 * 3)))
    .idle_timeout(Some(Duration::from_secs(60)))
    .connection_timeout(Duration::from_secs(1))
    .build(PgConnectionManager::new(database_url))
    .expect("Failed to establish connection pool")
}
