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

impl User {
  pub const fn default_id() -> i64 {
    -1
  }

  pub fn create(nickname: String, email: String, password: String) -> Self {
    Self {
      id: Self::default_id(),
      nickname,
      email,
      password,
      created_at: Utc::now(),
      updated_at: Utc::now(),
    }
  }
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

impl From<User> for dto::InsertUser {
  fn from(v: User) -> Self {
    Self {
      nickname: v.nickname,
      email: v.email,
      password: v.password,
    }
  }
}
