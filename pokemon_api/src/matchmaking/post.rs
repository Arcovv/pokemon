use actix_web::{post, web, HttpResponse};
use pokemon_service::DOMAIN_REGISTRY;

use crate::api_error::ApiError;

#[post("/api/matchmaking/cards/{card_id}")]
pub async fn run_matchmaking(path: web::Path<i64>) -> Result<HttpResponse, ApiError> {
  let card_id = path.into_inner();

  DOMAIN_REGISTRY
    .get()
    .unwrap()
    .read()
    .unwrap()
    .matchmaking_service()
    .run(card_id)?;

  Ok(HttpResponse::Ok().finish())
}
