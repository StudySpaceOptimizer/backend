use super::common::*;
use crate::{
  model::{blacklist::*, timeslot::*},
  repository::blacklist_repo::BlacklistRepository,
};

#[derive(Clone)]
pub struct SqliteBlacklistRepository {
  pool: Pool<Sqlite>,
}

impl SqliteBlacklistRepository {
  pub fn new(pool: Pool<Sqlite>) -> Self {
    SqliteBlacklistRepository { pool }
  }
}

impl BlacklistRepository for SqliteBlacklistRepository {
  async fn insert_user_to_blacklist(
    &self,
    user_name: &str,
    timeslot: TimeSlot,
  ) -> Result<(), Status> {
    let result = handle_sqlx(
      query!(
        "INSERT INTO BlackList
          (user_id, banned_at, unbanned_at)
        VALUES 
          (
            ?, 
            datetime(?, 'unixepoch', '+8 hours'), 
            datetime(?, 'unixepoch', '+8 hours')
          )",
        user_name,
        timeslot.start_time,
        timeslot.end_time
      )
      .execute(&self.pool)
      .await,
      "Inserting user to balck list",
    )?;

    let affected_rows = result.rows_affected();

    if affected_rows == 0 {
      log::warn!("No insert operation was executed");
      return Err(Status::NotFound);
    }

    Ok(())
  }

  async fn delete_user_from_blacklist(&self, user_name: &str) -> Result<(), Status> {
    let result = handle_sqlx(
      query!(
        "DELETE FROM BlackList
        WHERE
          user_id = ? ",
        user_name,
      )
      .execute(&self.pool)
      .await,
      "Deleting user from BlackList",
    )?;

    let affected_rows = result.rows_affected();

    if affected_rows == 0 {
      log::warn!("No delete operation was executed from BlackList");
      return Err(Status::NotFound);
    }

    Ok(())
  }
}

// read
// async fn is_user_in_blacklist( &self, id: i64) -> Result<bool, Status> {
//   let result = handle_sqlx(
//     query_scalar!(
//       "SELECT EXISTS(
//           SELECT 1 FROM BlackList
//           WHERE
//             user_id = ? AND
//             start_time <= datetime(strftime('%s','now'), 'unixepoch', '+8 hours') AND
//             end_time > datetime(strftime('%s','now'), 'unixepoch', '+8 hours')
//         )",
//       id,
//     )
//     .fetch_one(pool)
//     .await,
//     "Checking if the user is currently listed in the blacklist",
//   )?;

//   let is_within_blacklist: bool = result.map_or(false, |count| count != 0);

//   Ok(is_within_blacklist)
// }
