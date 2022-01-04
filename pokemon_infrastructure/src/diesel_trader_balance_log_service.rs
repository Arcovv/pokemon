use diesel::prelude::*;
use pokemon_model::dto;
use pokemon_model::dto::schema;
use pokemon_service::TraderBalanceLogService;

use crate::PgConnectionPool;

pub struct DieselTraderBalanceLogService {
  pool: PgConnectionPool,
}

impl DieselTraderBalanceLogService {
  pub fn new(pool: PgConnectionPool) -> Self {
    Self { pool }
  }
}

impl TraderBalanceLogService for DieselTraderBalanceLogService {
  fn add_log(&self, log: dto::InsertTraderBalanceLog) {
    self.add_logs(vec![log])
  }

  fn add_logs(&self, logs: Vec<dto::InsertTraderBalanceLog>) {
    use schema::traders_balance_logs as tbl;

    let conn = if let Ok(conn) = self.pool.get() {
      conn
    } else {
      return;
    };

    let count = diesel::insert_into(tbl::table)
      .values(&logs)
      .execute(&conn)
      .unwrap();

    assert_eq!(logs.len(), count);
  }
}
