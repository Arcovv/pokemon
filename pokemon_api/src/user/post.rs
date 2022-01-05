use actix_web::{post, web, HttpResponse};
use pokemon_service::DOMAIN_REGISTRY;

use super::{req, res};
use crate::api_error::ApiError;
use crate::token::Claims;

#[post("/api/users/login")]
pub async fn login(payload: web::Json<req::Login>) -> Result<HttpResponse, ApiError> {
  let domain_registry = DOMAIN_REGISTRY.get().unwrap().read().unwrap();

  let login = payload.into_inner();

  let user_id = domain_registry
    .user_service()
    .login(login.email, login.password)?;

  let token = Claims::create_token(user_id, "secret_key".into())?;
  let res = res::Token::new(token);

  Ok(HttpResponse::Ok().json(res))
}

#[post("/api/users/register")]
pub async fn register(payload: web::Json<req::Register>) -> Result<HttpResponse, ApiError> {
  let domain_registry = DOMAIN_REGISTRY.get().unwrap().read().unwrap();

  let register = payload.into_inner();

  domain_registry
    .user_service()
    .register(register.nickname, register.email, register.password)?;

  Ok(HttpResponse::Created().finish())
}
