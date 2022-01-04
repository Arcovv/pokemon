use chrono::{DateTime, Utc};

use super::schema::users;

#[derive(Queryable, Identifiable)]
#[table_name = "users"]
pub struct User {
  pub id: i64,
  pub nickname: String,
  pub email: String,
  pub password: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}
