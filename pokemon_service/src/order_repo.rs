use pokemon_model::Order;

use crate::CrudError;

pub trait OrderRepository {
  fn fetch_pending_buy_orders_by(&self, card_id: i64) -> Result<Vec<Order>, CrudError>;
  fn fetch_pending_sell_orders_by(&self, card_id: i64) -> Result<Vec<Order>, CrudError>;
  fn fetch_by_order_id(&self, order_id: i64) -> Result<Order, CrudError>;
  fn save_order(&self, order: Order) -> Result<(), CrudError>;
}
