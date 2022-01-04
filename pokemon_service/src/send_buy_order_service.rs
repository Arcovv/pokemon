use pokemon_model::CardPrice;

use crate::CrudError;

pub trait SendBuyOrderService {
  fn send(
    &self,
    card_id: i64,
    trader_id: i64,
    expected_price: CardPrice,
  ) -> Result<bool, CrudError>;
}
