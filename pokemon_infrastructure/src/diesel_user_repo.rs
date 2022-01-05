use chrono::Utc;
use diesel::prelude::*;
use pokemon_model::dto::schema;
use pokemon_model::{dto, User};
use pokemon_service::errors::CrudError;
use pokemon_service::UserRepository;

use crate::PgConnectionPool;

pub struct DieselUserRepository {
  pool: PgConnectionPool,
}

impl DieselUserRepository {
  pub fn new(pool: PgConnectionPool) -> Self {
    Self { pool }
  }
}

impl UserRepository for DieselUserRepository {
  fn fetch_by_email(&self, email: String) -> Result<User, CrudError> {
    use schema::users as u;

    let conn = self.pool.get()?;

    u::table
      .select(u::all_columns)
      .filter(u::email.eq(email))
      .first::<dto::User>(&conn)
      .optional()?
      .map(User::from)
      .ok_or(CrudError::NotFound)
  }

  fn fetch_by_id(&self, user_id: i64) -> Result<User, CrudError> {
    use schema::users as u;

    let conn = self.pool.get()?;

    u::table
      .select(u::all_columns)
      .filter(u::id.eq(user_id))
      .first::<dto::User>(&conn)
      .optional()?
      .map(User::from)
      .ok_or(CrudError::NotFound)
  }

  fn save(&self, user: User) -> Result<(), CrudError> {
    use schema::users as u;

    let conn = self.pool.get()?;

    let count = if user.id == User::default_id() {
      let dto_user = dto::InsertUser::from(user);

      diesel::insert_into(u::table)
        .values(dto_user)
        .execute(&conn)?
    } else {
      diesel::update(u::table)
        .filter(u::id.eq(user.id))
        .set((
          u::nickname.eq(user.nickname),
          u::password.eq(user.password),
          u::updated_at.eq(Utc::now()),
        ))
        .execute(&conn)?
    };

    if count != 1usize {
      Err(CrudError::SavingError)
    } else {
      Ok(())
    }
  }
}
