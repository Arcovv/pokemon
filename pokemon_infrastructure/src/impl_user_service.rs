use pokemon_model::User;
use pokemon_service::errors::UserServiceError;
use pokemon_service::{UserService, DOMAIN_REGISTRY};

pub struct ImplUserService;

impl UserService for ImplUserService {
  fn login(&self, email: String, password: String) -> Result<i64, UserServiceError> {
    let domain_registry = DOMAIN_REGISTRY.get().unwrap().read().unwrap();
    let user_repository = domain_registry.user_repository();

    let user = user_repository.fetch_by_email(email)?;
    let is_verified = bcrypt::verify(password, &user.password)?;

    if is_verified {
      Ok(user.id)
    } else {
      Err(UserServiceError::WrongPassword)
    }
  }

  fn register(
    &self,
    nickname: String,
    email: String,
    password: String,
  ) -> Result<i64, UserServiceError> {
    use bcrypt::DEFAULT_COST;

    let domain_registry = DOMAIN_REGISTRY.get().unwrap().read().unwrap();
    let user_repository = domain_registry.user_repository();

    let user_found = user_repository.fetch_by_email(email.clone());

    if user_found.is_ok() {
      return Err(UserServiceError::AccountExisted);
    }

    let password = bcrypt::hash(password, DEFAULT_COST)?;
    let user = User::create(nickname, email, password);
    let user_id = user.id;

    user_repository.save(user)?;

    Ok(user_id)
  }
}
