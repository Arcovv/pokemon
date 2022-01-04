use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct Order {
  pub id: i64,
  pub kind: String,
  pub status: String,
  pub card_id: i64,
  pub price: i32,
  pub created_at: DateTime<Utc>,
}

impl From<pokemon_model::Order> for Order {
  fn from(v: pokemon_model::Order) -> Self {
    let price = if v.is_completed() {
      v.actual_price.unwrap()
    } else {
      v.expected_price
    };

    Self {
      id: v.id,
      kind: v.kind.to_string(),
      status: v.status.to_string(),
      card_id: v.card_id,
      price: price.cents,
      created_at: v.created_at,
    }
  }
}

#[derive(Serialize)]
pub struct SendSellOrder {
  matched: bool,
  reason: Option<String>,
}

impl SendSellOrder {
  pub fn success() -> Self {
    Self {
      matched: true,
      reason: None,
    }
  }

  pub fn failure(reason: &str) -> Self {
    Self {
      matched: false,
      reason: Some(reason.to_string()),
    }
  }
}
