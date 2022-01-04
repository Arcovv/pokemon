use diesel::prelude::*;
use itertools::Itertools;
use pokemon_model::dto::schema;
use pokemon_model::{dto, Card};
use pokemon_service::errors::CrudError;
use pokemon_service::CardRepository;

use crate::PgConnectionPool;

pub struct DieselCardRepository {
  pool: PgConnectionPool,
}

impl DieselCardRepository {
  pub fn new(pool: PgConnectionPool) -> Self {
    Self { pool }
  }
}

impl CardRepository for DieselCardRepository {
  fn fetch_all_cards(&self) -> Result<Vec<Card>, CrudError> {
    use schema::cards as c;

    let conn = self.pool.get()?;

    let dto_cards = c::table
      .select(c::all_columns)
      .order_by(c::id)
      .get_results::<dto::Card>(&conn)?;

    let cards = dto_cards.into_iter().map(Card::from).collect_vec();

    Ok(cards)
  }

  fn fetch_by_id(&self, card_id: i64) -> Result<Card, CrudError> {
    use schema::cards as c;

    let conn = self.pool.get()?;

    let dto_card = c::table
      .select(c::all_columns)
      .filter(c::id.eq(card_id))
      .first::<dto::Card>(&conn)
      .optional()?
      .ok_or(CrudError::NotFound)?;

    let card = Card::from(dto_card);

    Ok(card)
  }
}
