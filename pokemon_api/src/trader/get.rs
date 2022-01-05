use actix_web::{get, web, HttpResponse};
use itertools::Itertools;
use pokemon_service::DOMAIN_REGISTRY;

use super::query::TraderOrderLimitation;
use super::res;
use crate::api_error::ApiError;
use crate::prelude::*;

#[get("/api/traders/orders")]
pub async fn fetch_latest_orders_by_trader(
  query: web::Query<TraderOrderLimitation>,
  user_id: UserId,
) -> Result<HttpResponse, ApiError> {
  let limit = query.limit;
  let user_id = user_id.0;

  let domain_registry = DOMAIN_REGISTRY.get().unwrap().read().unwrap();

  domain_registry.user_repository().fetch_by_id(user_id)?;

  let orders = domain_registry
    .order_query_commands()
    .latest_orders_by_trader_id(user_id, limit)?
    .into_iter()
    .map(res::Order::from)
    .collect_vec();

  Ok(HttpResponse::Ok().json(orders))
}

#[get("/api/cards/{card_id}/orders")]
pub async fn fetch_latest_orders_by_card(
  path: web::Path<i64>,
  query: web::Query<TraderOrderLimitation>,
) -> Result<HttpResponse, ApiError> {
  let card_id = path.into_inner();
  let limit = query.limit;

  let domain_registry = DOMAIN_REGISTRY.get().unwrap().read().unwrap();

  let orders = domain_registry
    .order_query_commands()
    .latest_orders_by_card_id(card_id, limit)?
    .into_iter()
    .map(res::Order::from)
    .collect_vec();

  Ok(HttpResponse::Ok().json(orders))
}
