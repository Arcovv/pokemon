use pokemon_model::Order;

use crate::errors::CrudError;

pub trait OrderQueryCommands {
  fn latest_orders_by_trader_id(&self, trader_id: i64, limit: u32)
    -> Result<Vec<Order>, CrudError>;

  fn latest_orders_by_card_id(&self, card_id: i64, limit: u32) -> Result<Vec<Order>, CrudError>;
}
