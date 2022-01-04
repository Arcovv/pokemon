use diesel::prelude::*;
use itertools::Itertools;
use pokemon_model::dto::schema;
use pokemon_model::{dto, Order};
use pokemon_service::errors::CrudError;
use pokemon_service::OrderQueryCommands;

use crate::PgConnectionPool;

pub struct DieselOrderQueryCommands {
  pool: PgConnectionPool,
}

impl DieselOrderQueryCommands {
  pub fn new(pool: PgConnectionPool) -> Self {
    Self { pool }
  }
}

impl OrderQueryCommands for DieselOrderQueryCommands {
  fn latest_orders_by_trader_id(
    &self,
    trader_id: i64,
    limit: u32,
  ) -> Result<Vec<Order>, CrudError> {
    use schema::orders as o;

    let conn = self.pool.get()?;

    let dto_orders = o::table
      .select(o::all_columns)
      .filter(o::buyer_id.eq(trader_id).or(o::seller_id.eq(trader_id)))
      .limit(limit as i64)
      .order_by(o::created_at.desc())
      .get_results::<dto::Order>(&conn)?;

    let orders = dto_orders.into_iter().map(Order::from).collect_vec();

    Ok(orders)
  }

  fn latest_orders_by_card_id(&self, card_id: i64, limit: u32) -> Result<Vec<Order>, CrudError> {
    use schema::orders as o;

    let conn = self.pool.get()?;

    let dto_orders = o::table
      .select(o::all_columns)
      .filter(o::card_id.eq(card_id))
      .limit(limit as i64)
      .order_by(o::created_at.desc())
      .get_results::<dto::Order>(&conn)?;

    let orders = dto_orders.into_iter().map(Order::from).collect_vec();

    Ok(orders)
  }
}
