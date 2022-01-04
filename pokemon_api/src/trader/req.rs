use serde::Deserialize;

#[derive(Deserialize)]
pub struct SellOrder {
  pub card_id: i64,
  pub expected_price: i32,
}

pub type BuyOrder = SellOrder;
