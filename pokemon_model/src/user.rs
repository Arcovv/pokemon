use chrono::{DateTime, Utc};

use super::dto;

pub struct User {
  pub id: i64,
  pub nickname: String,
  pub email: String,
  pub password: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<dto::User> for User {
  fn from(v: dto::User) -> Self {
    Self {
      id: v.id,
      nickname: v.nickname,
      email: v.email,
      password: v.password,
      created_at: v.created_at,
      updated_at: v.updated_at,
    }
  }
}
