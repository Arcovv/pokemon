use pokemon_service::errors::{CrudError, UserServiceError};

use crate::api_error::{self, ApiError};

impl From<CrudError> for ApiError {
  fn from(err: CrudError) -> Self {
    match err {
      CrudError::ConnectionError(err) => {
        error!("ConnectionError: {:?}", err);
        api_error::CONNECTION_ERROR.clone()
      }

      CrudError::DatabaseError(err) => {
        error!("DatabaseError: {:?}", err);
        api_error::DATABASE_ERROR.clone()
      }

      CrudError::SavingError => {
        error!("SavingError");
        api_error::SAVING_ERROR.clone()
      }

      CrudError::NotFound => {
        info!("NotFound: {:?}", err);
        api_error::NOT_FOUND.clone()
      }

      CrudError::DeleteError => {
        warn!("DeleteError");
        api_error::DELETE_ERROR.clone()
      }
    }
  }
}

impl From<UserServiceError> for ApiError {
  fn from(err: UserServiceError) -> Self {
    use UserServiceError::*;

    error!("UserServiceError: {:?}", err);

    match err {
      CrudError(err) => ApiError::from(err),
      WrongPassword => api_error::WRONG_PASSWORD.clone(),
      AccountExisted => api_error::ACCOUNT_EXISTED.clone(),
      AccountNotExist => api_error::ACCOUNT_NOT_EXIST.clone(),
      HashError(_) => api_error::HASH_ERROR.clone(),
    }
  }
}
