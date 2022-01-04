use chrono::{Duration, Utc};
use pokemon_model::Order;
use pokemon_model::OrderKind::{Buy, Sell};
use pokemon_model::OrderStatus::Pending;
use pokemon_service::errors::CrudError;
use pokemon_service::OrderRepository;

pub struct MockOrderRepository;

impl OrderRepository for MockOrderRepository {
  #[rustfmt::skip]
  fn fetch_pending_buy_orders_by(&self, _card_id: i64) -> Result<Vec<Order>, CrudError> {
    let orders = vec![
      Order { id: 1, kind: Buy, status: Pending, buyer_id: Some(1), seller_id: None, buy_order_id: None,
        sell_order_id: None, card_id: 1, expected_price: 110.into(), actual_price: None, created_at: Utc::now() - Duration::hours(3) },

      Order { id: 2, kind: Buy, status: Pending, buyer_id: Some(2), seller_id: None, buy_order_id: None,
        sell_order_id: None, card_id: 1, expected_price: 115.into(), actual_price: None, created_at: Utc::now() - Duration::hours(2) },

      Order { id: 3, kind: Buy, status: Pending, buyer_id: Some(3), seller_id: None, buy_order_id: None,
        sell_order_id: None, card_id: 1, expected_price: 120.into(), actual_price: None, created_at: Utc::now() - Duration::hours(1) },
    ];

    Ok(orders)
  }

  #[rustfmt::skip]
  fn fetch_pending_sell_orders_by(&self, _card_id: i64) -> Result<Vec<Order>, CrudError> {
    let now = Utc::now();

    let orders = vec![
      Order { id: 1, kind: Sell, status: Pending, buyer_id: None, seller_id: Some(1), buy_order_id: None,
        sell_order_id: None, card_id: 1, expected_price: 200.into(), actual_price: None, created_at: now - Duration::hours(3) },

      Order { id: 2, kind: Sell, status: Pending, buyer_id: None, seller_id: Some(2), buy_order_id: None,
        sell_order_id: None, card_id: 1, expected_price: 205.into(), actual_price: None, created_at: now - Duration::hours(2) },

      Order { id: 3, kind: Sell, status: Pending, buyer_id: None, seller_id: Some(3), buy_order_id: None,
        sell_order_id: None, card_id: 1, expected_price: 210.into(), actual_price: None, created_at: now - Duration::hours(1) },
    ];

    Ok(orders)
  }

  fn fetch_by_order_id(&self, _order_id: i64) -> Result<Order, CrudError> {
    let order = Order {
      id: 1,
      kind: Sell,
      status: Pending,
      buyer_id: None,
      seller_id: Some(1),
      buy_order_id: None,
      sell_order_id: None,
      card_id: 1,
      expected_price: 200.into(),
      actual_price: None,
      created_at: Utc::now() - Duration::hours(3),
    };

    Ok(order)
  }

  fn save_order(&self, _order: Order) -> Result<(), CrudError> {
    Ok(())
  }
}
