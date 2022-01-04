use chrono::{DateTime, Utc};

use super::dto;

pub struct Card {
  pub id: i64,
  pub name: String,
  pub created_at: DateTime<Utc>,
}

impl From<dto::Card> for Card {
  fn from(v: dto::Card) -> Self {
    Self {
      id: v.id,
      name: v.name,
      created_at: v.created_at,
    }
  }
}
