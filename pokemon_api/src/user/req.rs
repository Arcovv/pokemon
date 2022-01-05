use serde::Deserialize;

#[derive(Deserialize)]
pub struct Login {
  pub email: String,
  pub password: String,
}

#[derive(Deserialize)]
pub struct Register {
  pub nickname: String,
  pub email: String,
  pub password: String,
  pub password_again: String,
}
