use std::num::NonZeroU16;

use lazy_static::lazy_static;

use super::ApiError;

macro_rules! api_error {
  ($error_code:expr, $status_code:expr, $konst:ident, $reason:expr) => {
    lazy_static! {
      pub static ref $konst: ApiError = unsafe {
        ApiError::new(
          NonZeroU16::new_unchecked($status_code),
          $error_code,
          $reason.into(),
        )
      };
    }
  };
}

api_error!(10000, 500, CONNECTION_ERROR, "Failed to get connection");
api_error!(10001, 500, DATABASE_ERROR, "Encouter database error");
api_error!(10002, 400, SAVING_ERROR, "Failed to save data");
api_error!(10003, 404, NOT_FOUND, "Failed to find data");
api_error!(10004, 400, DELETE_ERROR, "Failed to delete data");
api_error!(10005, 500, HASH_ERROR, "Failed to handle hash method");
api_error!(10006, 400, WRONG_PASSWORD, "This is wrong password");
api_error!(10007, 404, ACCOUNT_NOT_EXIST, "Account not exists");
api_error!(10008, 403, ACCOUNT_EXISTED, "Account existed");
api_error!(10009, 500, JWT_ENCODE_ERROR, "Failed to create token");
api_error!(10009, 400, JWT_DECODE_ERROR, "Failed to decode token");
api_error!(10010, 403, INVALID_TOKEN, "Invalid token");
