use chrono::{Duration, Utc};
use pokemon_model::Order;
use pokemon_model::OrderKind::Sell;
use pokemon_model::OrderStatus::Pending;
use pokemon_service::errors::CrudError;
use pokemon_service::OrderQueryCommands;

pub struct MockOrdersQueryCommands;

impl OrderQueryCommands for MockOrdersQueryCommands {
  #[rustfmt::skip]
  fn latest_orders_by_trader_id(
    &self,
    _trader_id: i64,
    _limit: u32,
  ) -> Result<Vec<Order>, CrudError> {
    let now = Utc::now();

    let orders = vec![
      Order { id: 1, kind: Sell, status: Pending, buyer_id: None, seller_id: Some(1), buy_order_id: None,
        sell_order_id: None, card_id: 1, expected_price: 200.into(), actual_price: None, created_at: now - Duration::hours(3) },

      Order { id: 2, kind: Sell, status: Pending, buyer_id: None, seller_id: Some(1), buy_order_id: None,
        sell_order_id: None, card_id: 1, expected_price: 205.into(), actual_price: None, created_at: now - Duration::hours(2) },

      Order { id: 3, kind: Sell, status: Pending, buyer_id: None, seller_id: Some(1), buy_order_id: None,
        sell_order_id: None, card_id: 1, expected_price: 210.into(), actual_price: None, created_at: now - Duration::hours(1) },
    ];

    Ok(orders)
  }

  #[rustfmt::skip]
  fn latest_orders_by_card_id(&self, _card_id: i64, _limit: u32) -> Result<Vec<Order>, CrudError> {
    let now = Utc::now();

    let orders = vec![
      Order { id: 4, kind: Sell, status: Pending, buyer_id: None, seller_id: Some(1), buy_order_id: None,
        sell_order_id: None, card_id: 1, expected_price: 200.into(), actual_price: None, created_at: now - Duration::hours(3) },

      Order { id: 5, kind: Sell, status: Pending, buyer_id: None, seller_id: Some(2), buy_order_id: None,
        sell_order_id: None, card_id: 1, expected_price: 205.into(), actual_price: None, created_at: now - Duration::hours(2) },

      Order { id: 6, kind: Sell, status: Pending, buyer_id: None, seller_id: Some(3), buy_order_id: None,
        sell_order_id: None, card_id: 1, expected_price: 210.into(), actual_price: None, created_at: now - Duration::hours(1) },
    ];

    Ok(orders)
  }
}
