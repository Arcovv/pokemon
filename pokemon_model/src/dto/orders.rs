use chrono::{DateTime, Utc};

use super::schema::orders;

#[derive(Queryable, Identifiable, AsChangeset)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name = "orders"]
pub struct Order {
  pub id: i64,
  pub kind: String,
  pub status: String,
  pub buyer_id: Option<i64>,
  pub seller_id: Option<i64>,
  pub buy_order_id: Option<i64>,
  pub sell_order_id: Option<i64>,
  pub card_id: i64,
  pub expected_price: i32,
  pub actual_price: Option<i32>,
  pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "orders"]
pub struct InsertOrder {
  pub kind: String,
  pub status: String,
  pub buyer_id: Option<i64>,
  pub seller_id: Option<i64>,
  pub buy_order_id: Option<i64>,
  pub sell_order_id: Option<i64>,
  pub card_id: i64,
  pub expected_price: i32,
  pub actual_price: Option<i32>,
}
