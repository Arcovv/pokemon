use actix_web::dev::Payload;
use actix_web::http::header::Header;
use actix_web::{web, FromRequest, HttpRequest};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use futures_util::future::{err, ok, Ready};

use crate::api_error::{self, ApiError};
use crate::config::Config;
use crate::token::Claims;

pub struct UserId(pub i64);

impl FromRequest for UserId {
  type Error = ApiError;
  type Future = Ready<Result<Self, Self::Error>>;

  fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
    let config = req.app_data::<web::Data<Config>>().unwrap();
    let authorization = Authorization::<Bearer>::parse(req);

    match authorization {
      Ok(bearer) => {
        let token = bearer.into_scheme().token().to_string();
        let claims = Claims::decode_token(token, config.jwt_secret.clone());

        match claims {
          Ok(c) => ok(UserId(c.user_id)),
          Err(e) => err(e),
        }
      }
      Err(_) => err(api_error::INVALID_TOKEN.clone()),
    }
  }
}
