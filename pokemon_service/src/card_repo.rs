use pokemon_model::Card;

use crate::CrudError;

pub trait CardRepository {
  fn fetch_all_cards(&self) -> Result<Vec<Card>, CrudError>;
  fn fetch_by_id(&self, card_id: i64) -> Result<Card, CrudError>;
}
