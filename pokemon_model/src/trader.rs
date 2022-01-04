use chrono::{DateTime, Utc};

use super::{dto, CardPrice};
use crate::dto::InsertTraderBalanceLog;

pub struct Trader {
  pub id: i64,
  pub user_id: i64,
  pub balance: i32,
  pub created_at: DateTime<Utc>,
}

impl Trader {
  pub const fn default_id() -> i64 {
    -1
  }

  pub fn is_enough_balance(&self, price: CardPrice) -> bool {
    self.balance >= price.cents
  }

  pub fn balance_increase(&mut self, value: CardPrice) -> InsertTraderBalanceLog {
    let previous_value = self.balance;

    self.balance += value.cents;

    let current_value = self.balance;
    let modify_value = value.cents;

    InsertTraderBalanceLog::new(
      previous_value,
      current_value,
      modify_value,
      "Balance increase".into(),
    )
  }

  pub fn balance_decrease(&mut self, value: CardPrice) -> InsertTraderBalanceLog {
    let previous_value = self.balance;

    self.balance -= value.cents;

    let current_value = self.balance;
    let modify_value = value.cents;

    InsertTraderBalanceLog::new(
      previous_value,
      current_value,
      -modify_value,
      "Balance decrease".into(),
    )
  }
}

impl From<dto::Trader> for Trader {
  fn from(v: dto::Trader) -> Self {
    Self {
      id: v.id,
      user_id: v.user_id,
      balance: v.balance,
      created_at: v.created_at,
    }
  }
}

impl From<Trader> for dto::InsertTrader {
  fn from(v: Trader) -> Self {
    Self {
      user_id: v.user_id,
      balance: v.balance,
    }
  }
}
