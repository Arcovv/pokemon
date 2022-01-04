mod from;

use std::num::NonZeroU16;

use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
#[error("ApiError [status_code: {status_code}, error_code: {error_code}, reason: {reason} ]")]
pub struct ApiError {
  status_code: NonZeroU16,
  error_code: u32,
  reason: String,
}

impl ApiError {
  pub fn new(status_code: NonZeroU16, error_code: u32, reason: String) -> Self {
    Self {
      status_code,
      error_code,
      reason,
    }
  }
}

impl ResponseError for ApiError {
  fn status_code(&self) -> StatusCode {
    StatusCode::from_u16(self.status_code.into()).unwrap()
  }

  fn error_response(&self) -> HttpResponse {
    HttpResponse::build(self.status_code()).json(self)
  }
}
