use crate::CrudError;

pub trait MatchmakingService {
  fn run(&self, card_id: i64) -> Result<(), CrudError>;
}
