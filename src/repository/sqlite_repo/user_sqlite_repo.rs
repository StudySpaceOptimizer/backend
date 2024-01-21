use super::common::*;
use crate::{model::user::*, repository::user_repo::UserRepository};

#[derive(Clone)]
pub struct SqliteUserRepository {
  pool: Pool<Sqlite>,
}

impl SqliteUserRepository {
  pub fn new(pool: Pool<Sqlite>) -> Self {
    SqliteUserRepository { pool }
  }
}

impl UserRepository for SqliteUserRepository {
  async fn insert_user(
    &self,
    email: &str,
    password_hash: &str,
    user_role: UserRole,
    verified: bool,
    verification_token: &str,
  ) -> Result<(), Status> {
    handle_sqlx(
    query!("INSERT INTO Users (email, password_hash, user_role, verified, verification_token, points) VALUES (?, ?, ?, ?, ?, ?)", email, password_hash, user_role, verified, verification_token, 0)
      .execute(&self.pool)
      .await,
    "Inserting user",
  )?;

    Ok(())
  }

  async fn query_user_by_email(&self, email: &str) -> Result<User, Status> {
    let user = handle_sqlx(
      sqlx::query_as::<_, User>(
        "SELECT *
         FROM Users
         WHERE email = ?",
      )
      .bind(email)
      .fetch_one(&self.pool)
      .await,
      "Querying user by email",
    )?;

    Ok(user)
  }

  async fn query_user_by_id(&self, user_id: i64) -> Result<User, Status> {
    let user = handle_sqlx(
      sqlx::query_as::<_, User>(
        "SELECT *
         FROM Users
         WHERE user_id = ?",
      )
      .bind(user_id)
      .fetch_one(&self.pool)
      .await,
      "Querying user by id",
    )?;

    Ok(user)
  }

  async fn update_verified(&self, verification_token: &str) -> Result<(), Status> {
    let result = handle_sqlx(
      sqlx::query!(
        "UPDATE Users SET verified = true WHERE verification_token = ?",
        verification_token
      )
      .execute(&self.pool)
      .await,
      "Updating user verified by verification_token",
    )?;

    let affected_rows = result.rows_affected();

    if affected_rows == 0 {
      log::warn!("No Users found for updation");
      return Err(Status::NotFound);
    }

    Ok(())
  }
}
