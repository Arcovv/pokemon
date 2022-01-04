use chrono::{DateTime, Utc};

use super::schema::{traders, traders_balance_logs};

#[derive(Queryable, Identifiable)]
#[table_name = "traders"]
pub struct Trader {
  pub id: i64,
  pub user_id: i64,
  pub balance: i32,
  pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "traders"]
pub struct InsertTrader {
  pub user_id: i64,
  pub balance: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "traders_balance_logs"]
pub struct InsertTraderBalanceLog {
  pub previous_value: i32,
  pub current_value: i32,
  pub modify_value: i32,
  pub reason: String,
}

impl InsertTraderBalanceLog {
  pub fn new(previous_value: i32, current_value: i32, modify_value: i32, reason: String) -> Self {
    Self {
      previous_value,
      current_value,
      modify_value,
      reason,
    }
  }
}
