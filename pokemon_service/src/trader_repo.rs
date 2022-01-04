use pokemon_model::Trader;

use crate::CrudError;

pub trait TraderRepository {
  fn fetch_all_traders(&self) -> Result<Vec<Trader>, CrudError>;
  fn fetch_by_id(&self, trader_id: i64) -> Result<Trader, CrudError>;
  fn save(&self, trader: Trader) -> Result<(), CrudError>;
}
