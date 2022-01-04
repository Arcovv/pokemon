use thiserror::Error;

#[derive(Debug, Error)]
pub enum CrudError {
  #[error("Failed to get connection cause: {0:?}")]
  ConnectionError(#[from] r2d2::Error),

  #[error("Encounter database error: {0:?}")]
  DatabaseError(#[from] diesel::result::Error),

  #[error("Failed to save data")]
  SavingError,

  #[error("Failed to get data")]
  NotFound,

  #[error("Failed to delete data")]
  DeleteError,
}
