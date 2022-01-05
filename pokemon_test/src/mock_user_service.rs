use pokemon_service::errors::UserServiceError;
use pokemon_service::UserService;

pub struct MockUserService;

impl UserService for MockUserService {
  fn login(&self, email: String, password: String) -> Result<i64, UserServiceError> {
    if email == "arcofz@gmail.com" && password == "111" {
      Ok(1)
    } else {
      Err(UserServiceError::WrongPassword)
    }
  }

  fn register(
    &self,
    _nickname: String,
    _email: String,
    _password: String,
  ) -> Result<i64, UserServiceError> {
    Ok(1)
  }
}
