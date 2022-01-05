use crate::errors::UserServiceError;

pub trait UserService {
  fn login(&self, email: String, password: String) -> Result<i64, UserServiceError>;

  fn register(
    &self,
    nickname: String,
    email: String,
    password: String,
  ) -> Result<i64, UserServiceError>;
}
