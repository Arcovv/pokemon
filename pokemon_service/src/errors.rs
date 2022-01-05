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

#[derive(Debug, Error)]
pub enum UserServiceError {
  #[error("Encounter CrudError: {0:?}")]
  CrudError(#[from] CrudError),

  #[error("Wrong password")]
  WrongPassword,

  #[error("Account existed")]
  AccountExisted,

  #[error("Account not exist")]
  AccountNotExist,

  #[error("Hash password error: {0:?}")]
  HashError(#[from] bcrypt::BcryptError),
}
