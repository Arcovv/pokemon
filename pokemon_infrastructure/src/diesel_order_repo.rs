use diesel::prelude::*;
use itertools::Itertools;
use pokemon_model::dto::{self, schema};
use pokemon_model::{Order, OrderKind, OrderStatus};
use pokemon_service::errors::CrudError;
use pokemon_service::OrderRepository;
use OrderKind::{Buy, Sell};
use OrderStatus::Pending;

use crate::PgConnectionPool;

#[derive(Clone)]
pub struct DieselOrderRepository {
  pool: PgConnectionPool,
}

impl DieselOrderRepository {
  pub fn new(pool: PgConnectionPool) -> Self {
    Self { pool }
  }
}

impl OrderRepository for DieselOrderRepository {
  fn fetch_pending_buy_orders_by(&self, card_id: i64) -> Result<Vec<Order>, CrudError> {
    use schema::orders as o;

    let conn = self.pool.get()?;

    let dto_orders = o::table
      .select(o::all_columns)
      .filter(o::kind.eq(Buy.to_string()))
      .filter(o::status.eq(Pending.to_string()))
      .filter(o::card_id.eq(card_id))
      .order_by(o::created_at)
      .get_results::<dto::Order>(&conn)?;

    let orders = dto_orders.into_iter().map(Order::from).collect_vec();

    Ok(orders)
  }

  fn fetch_pending_sell_orders_by(&self, card_id: i64) -> Result<Vec<Order>, CrudError> {
    use schema::orders as o;

    let conn = self.pool.get()?;

    let dto_orders = o::table
      .select(o::all_columns)
      .filter(o::kind.eq(Sell.to_string()))
      .filter(o::status.eq(Pending.to_string()))
      .filter(o::card_id.eq(card_id))
      .order_by(o::created_at)
      .get_results::<dto::Order>(&conn)?;

    let orders = dto_orders.into_iter().map(Order::from).collect_vec();

    Ok(orders)
  }

  fn fetch_by_order_id(&self, order_id: i64) -> Result<Order, CrudError> {
    use schema::orders as o;

    let conn = self.pool.get()?;

    let dto_order = o::table
      .select(o::all_columns)
      .filter(o::id.eq(order_id))
      .first::<dto::Order>(&conn)
      .optional()?
      .ok_or(CrudError::NotFound)?;

    let order = Order::from(dto_order);

    Ok(order)
  }

  fn save_order(&self, order: Order) -> Result<(), CrudError> {
    use schema::orders as o;

    let conn = self.pool.get()?;

    let count = if order.id == Order::default_id() {
      let dto_order = dto::InsertOrder::from(order);

      diesel::insert_into(o::table)
        .values(dto_order)
        .execute(&conn)?
    } else {
      let dto_order = dto::Order::from(order);

      diesel::update(o::table)
        .filter(o::id.eq(dto_order.id))
        .set(&dto_order)
        .execute(&conn)?
    };

    if count != 1usize {
      Err(CrudError::SavingError)
    } else {
      Ok(())
    }
  }
}
