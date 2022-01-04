use diesel::prelude::*;
use itertools::Itertools;
use pokemon_model::dto::schema;
use pokemon_model::{dto, Trader};
use pokemon_service::errors::CrudError;
use pokemon_service::TraderRepository;

use crate::PgConnectionPool;

pub struct DieselTraderRepository {
  pool: PgConnectionPool,
}

impl DieselTraderRepository {
  pub fn new(pool: PgConnectionPool) -> Self {
    Self { pool }
  }
}

impl TraderRepository for DieselTraderRepository {
  fn fetch_all_traders(&self) -> Result<Vec<Trader>, CrudError> {
    use schema::traders as t;

    let conn = self.pool.get()?;

    let dto_traders = t::table
      .select(t::all_columns)
      .order_by(t::id)
      .get_results::<dto::Trader>(&conn)?;

    let traders = dto_traders.into_iter().map(Trader::from).collect_vec();

    Ok(traders)
  }

  fn fetch_by_id(&self, trader_id: i64) -> Result<Trader, CrudError> {
    use schema::traders as t;

    let conn = self.pool.get()?;

    let dto_trader = t::table
      .select(t::all_columns)
      .filter(t::id.eq(trader_id))
      .first::<dto::Trader>(&conn)
      .optional()?
      .ok_or(CrudError::NotFound)?;

    let trader = Trader::from(dto_trader);

    Ok(trader)
  }

  fn save(&self, trader: Trader) -> Result<(), CrudError> {
    use schema::traders as t;

    let conn = self.pool.get()?;

    let count = if trader.id == Trader::default_id() {
      let dto_trader = dto::InsertTrader::from(trader);

      diesel::insert_into(t::table)
        .values(dto_trader)
        .execute(&conn)?
    } else {
      diesel::update(t::table)
        .filter(t::id.eq(trader.id))
        .set(t::balance.eq(trader.balance))
        .execute(&conn)?
    };

    if count != 1usize {
      Err(CrudError::SavingError)
    } else {
      Ok(())
    }
  }
}
