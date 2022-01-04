use std::num::NonZeroU16;

use pokemon_service::errors::CrudError;

use super::ApiError;

impl From<CrudError> for ApiError {
  fn from(err: CrudError) -> Self {
    match err {
      CrudError::ConnectionError(err) => {
        error!("ConnectionError: {:?}", err);

        unsafe {
          ApiError::new(
            NonZeroU16::new_unchecked(500),
            10000,
            "Failed to get connection".into(),
          )
        }
      }

      CrudError::DatabaseError(err) => {
        error!("DatabaseError: {:?}", err);

        unsafe {
          ApiError::new(
            NonZeroU16::new_unchecked(500),
            10001,
            "Encouter database error".into(),
          )
        }
      }

      CrudError::SavingError => {
        error!("SavingError");

        unsafe {
          ApiError::new(
            NonZeroU16::new_unchecked(400),
            10002,
            "Failed to save data".into(),
          )
        }
      }

      CrudError::NotFound => {
        info!("NotFound: {:?}", err);

        unsafe {
          ApiError::new(
            NonZeroU16::new_unchecked(404),
            10003,
            "Failed to find data".into(),
          )
        }
      }

      CrudError::DeleteError => {
        warn!("DeleteError");

        unsafe {
          ApiError::new(
            NonZeroU16::new_unchecked(400),
            10004,
            "Failed to delete data".into(),
          )
        }
      }
    }
  }
}
