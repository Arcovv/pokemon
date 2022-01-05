use chrono::Utc;
use pokemon_model::User;
use pokemon_service::errors::CrudError;
use pokemon_service::UserRepository;

pub struct MockUserRepository;

impl UserRepository for MockUserRepository {
  fn fetch_by_email(&self, email: String) -> Result<User, CrudError> {
    let user = User {
      id: 1,
      nickname: "my_nickname".into(),
      email,
      password: "password".into(),
      created_at: Utc::now(),
      updated_at: Utc::now(),
    };

    Ok(user)
  }

  fn fetch_by_id(&self, user_id: i64) -> Result<User, CrudError> {
    let user = User {
      id: user_id,
      nickname: "my_nickname".into(),
      email: "email@gmail.com".into(),
      password: "password".into(),
      created_at: Utc::now(),
      updated_at: Utc::now(),
    };

    Ok(user)
  }

  fn save(&self, _user: User) -> Result<(), CrudError> {
    Ok(())
  }
}
