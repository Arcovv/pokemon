use chrono::{DateTime, Utc};

use super::schema::cards;

#[derive(Queryable, Identifiable)]
#[table_name = "cards"]
pub struct Card {
  pub id: i64,
  pub name: String,
  pub created_at: DateTime<Utc>,
}
