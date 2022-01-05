use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::api_error::{self, ApiError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub user_id: i64,

  #[serde(with = "jwt_numeric_date")]
  exp: DateTime<Utc>,
}

impl Claims {
  pub fn create_token(user_id: i64, secret_key: String) -> Result<String, ApiError> {
    let now = Utc::now();
    let claims = Claims {
      user_id,
      exp: now + Duration::days(30),
    };

    encode(
      &Header::default(),
      &claims,
      &EncodingKey::from_secret(secret_key.as_ref()),
    )
    .map_err(|err| {
      error!("JwtError: {:?}", err);
      api_error::JWT_ENCODE_ERROR.clone()
    })
  }

  pub fn decode_token(token: String, secret_key: String) -> Result<Claims, ApiError> {
    decode::<Claims>(
      &token,
      &DecodingKey::from_secret(secret_key.as_ref()),
      &Validation::default(),
    )
    .map_err(|err| {
      error!("JwtError: {:?}", err);
      api_error::JWT_DECODE_ERROR.clone()
    })
    .map(|t| t.claims)
  }
}

/// Info:
/// https://github.com/Keats/jsonwebtoken/blob/master/examples/custom_chrono.rs#L29
mod jwt_numeric_date {
  //! Custom serialization of DateTime<Utc> to conform with the JWT spec (RFC
  //! 7519 section 2, "Numeric Date")
  use chrono::{DateTime, TimeZone, Utc};
  use serde::{self, Deserialize, Deserializer, Serializer};

  /// Serializes a DateTime<Utc> to a Unix timestamp (milliseconds since
  /// 1970/1/1T00:00:00T)
  pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let timestamp = date.timestamp();
    serializer.serialize_i64(timestamp)
  }

  /// Attempts to deserialize an i64 and use as a Unix timestamp
  pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
  where
    D: Deserializer<'de>,
  {
    Utc
      .timestamp_opt(i64::deserialize(deserializer)?, 0)
      .single() // If there are multiple or no valid DateTimes from timestamp, return None
      .ok_or_else(|| serde::de::Error::custom("invalid Unix timestamp value"))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_decode_token() {
    let token = Claims::create_token(1, "secret_key".into()).unwrap();
    println!("token: {}", token);

    let claims = Claims::decode_token(token, "secret_key".into()).unwrap();

    assert_eq!(claims.user_id, 1);
  }
}
