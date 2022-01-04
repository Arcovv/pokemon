use actix_web::{post, web, HttpResponse};
use pokemon_service::DOMAIN_REGISTRY;

use super::{req, res};
use crate::api_error::ApiError;

#[post("/api/traders/{trader_id}/orders/sell")]
pub async fn send_sell_order(
  path: web::Path<i64>,
  payload: web::Json<req::SellOrder>,
) -> Result<HttpResponse, ApiError> {
  let payload = payload.into_inner();
  let trader_id = path.into_inner();
  let card_id = payload.card_id;
  let expected_price = payload.expected_price.into();

  let domain_registry = DOMAIN_REGISTRY.get().unwrap().read().unwrap();
  let send_sell_order_service = domain_registry.send_sell_order_service();
  let card_repository = domain_registry.card_repository();

  // Make sure card is existed
  card_repository.fetch_by_id(payload.card_id)?;

  let is_success = send_sell_order_service.send(card_id, trader_id, expected_price)?;

  let result = if is_success {
    res::SendSellOrder::success()
  } else {
    res::SendSellOrder::failure("Sell order is waiting for new buy order")
  };

  Ok(HttpResponse::Ok().json(result))
}

#[post("/api/traders/{trader_id}/orders/buy")]
pub async fn send_buy_order(
  path: web::Path<i64>,
  payload: web::Json<req::BuyOrder>,
) -> Result<HttpResponse, ApiError> {
  let trader_id = path.into_inner();
  let payload = payload.into_inner();
  let card_id = payload.card_id;
  let expected_price = payload.expected_price.into();

  let domain_registry = DOMAIN_REGISTRY.get().unwrap().read().unwrap();
  let send_buy_order_service = domain_registry.send_buy_order_service();
  let card_repository = domain_registry.card_repository();

  // Make sure card is existed
  card_repository.fetch_by_id(payload.card_id)?;

  let is_success = send_buy_order_service.send(card_id, trader_id, expected_price)?;

  let result = if is_success {
    res::SendSellOrder::success()
  } else {
    res::SendSellOrder::failure("Buy order is waiting for new sell order")
  };

  Ok(HttpResponse::Ok().json(result))
}
