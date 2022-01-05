use pokemon_model::User;

use crate::errors::CrudError;

pub trait UserRepository {
  fn fetch_by_email(&self, email: String) -> Result<User, CrudError>;
  fn fetch_by_id(&self, user_id: i64) -> Result<User, CrudError>;
  fn save(&self, user: User) -> Result<(), CrudError>;
}
