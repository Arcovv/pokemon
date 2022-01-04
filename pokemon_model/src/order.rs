use std::str::FromStr;

use chrono::{DateTime, Utc};

use super::{dto, CardPrice};

#[derive(AsRefStr, ToString, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum OrderKind {
  Buy,
  Sell,
}

#[derive(AsRefStr, ToString, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum OrderStatus {
  Pending,
  Completed,
  Failed,
}

impl OrderStatus {
  /// Returns `true` if the order status is [`Pending`].
  ///
  /// [`Pending`]: OrderStatus::Pending
  pub fn is_pending(&self) -> bool {
    matches!(self, Self::Pending)
  }

  /// Returns `true` if the order status is [`Completed`].
  ///
  /// [`Completed`]: OrderStatus::Completed
  pub fn is_completed(&self) -> bool {
    matches!(self, Self::Completed)
  }

  /// Returns `true` if the order status is [`Failed`].
  ///
  /// [`Failed`]: OrderStatus::Failed
  pub fn is_failed(&self) -> bool {
    matches!(self, Self::Failed)
  }
}

pub struct Order {
  pub id: i64,
  pub kind: OrderKind,
  pub status: OrderStatus,
  pub buyer_id: Option<i64>,
  pub seller_id: Option<i64>,
  pub buy_order_id: Option<i64>,
  pub sell_order_id: Option<i64>,
  pub card_id: i64,
  pub expected_price: CardPrice,
  pub actual_price: Option<CardPrice>,
  pub created_at: DateTime<Utc>,
}

impl Order {
  pub const fn default_id() -> i64 {
    -1
  }

  pub fn create_buy_order(trader_id: i64, card_id: i64, expected_price: CardPrice) -> Self {
    Self {
      id: Self::default_id(),
      kind: OrderKind::Buy,
      status: OrderStatus::Pending,
      buyer_id: Some(trader_id),
      seller_id: None,
      buy_order_id: None,
      sell_order_id: None,
      card_id,
      expected_price,
      actual_price: None,
      created_at: Utc::now(),
    }
  }

  pub fn create_sell_order(trader_id: i64, card_id: i64, expected_price: CardPrice) -> Self {
    Self {
      id: Self::default_id(),
      kind: OrderKind::Sell,
      status: OrderStatus::Pending,
      buyer_id: None,
      seller_id: Some(trader_id),
      buy_order_id: None,
      sell_order_id: None,
      card_id,
      expected_price,
      actual_price: None,
      created_at: Utc::now(),
    }
  }

  pub fn mark_as_completed(&mut self) {
    self.status = OrderStatus::Completed;
  }

  pub fn mark_buy_failed(&mut self) {
    self.status = OrderStatus::Failed;
    self.seller_id = None;
    self.buy_order_id = None;
    self.sell_order_id = None;
    self.actual_price = None;
  }

  pub fn is_completed(&self) -> bool {
    self.status.is_completed()
  }
}

impl From<dto::Order> for Order {
  fn from(v: dto::Order) -> Self {
    let expected_price = CardPrice::new(v.expected_price);

    let actual_price = v.actual_price.map(CardPrice::new);

    Self {
      id: v.id,
      kind: OrderKind::from_str(&v.kind).unwrap(),
      status: OrderStatus::from_str(&v.status).unwrap(),
      buyer_id: v.buyer_id,
      seller_id: v.seller_id,
      buy_order_id: v.buy_order_id,
      sell_order_id: v.sell_order_id,
      card_id: v.card_id,
      expected_price,
      actual_price,
      created_at: v.created_at,
    }
  }
}

impl From<Order> for dto::Order {
  fn from(o: Order) -> Self {
    Self {
      id: o.id,
      kind: o.kind.to_string(),
      status: o.status.to_string(),
      buyer_id: o.buyer_id,
      seller_id: o.seller_id,
      buy_order_id: o.buy_order_id,
      sell_order_id: o.sell_order_id,
      card_id: o.card_id,
      expected_price: o.expected_price.cents,
      actual_price: o.actual_price.map(|p| p.cents),
      created_at: o.created_at,
    }
  }
}

impl From<Order> for dto::InsertOrder {
  fn from(o: Order) -> Self {
    Self {
      kind: o.kind.to_string(),
      status: o.status.to_string(),
      buyer_id: o.buyer_id,
      seller_id: o.seller_id,
      buy_order_id: o.buy_order_id,
      sell_order_id: o.sell_order_id,
      card_id: o.card_id,
      expected_price: o.expected_price.cents,
      actual_price: o.actual_price.map(|p| p.cents),
    }
  }
}
