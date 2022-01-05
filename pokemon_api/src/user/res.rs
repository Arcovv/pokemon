use serde::Serialize;

#[derive(Serialize)]
pub struct Token {
  pub token: String,
}

impl Token {
  pub fn new(token: String) -> Self {
    Self { token }
  }
}
