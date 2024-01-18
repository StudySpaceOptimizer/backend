use super::common::*;
use crate::{model::timeslot::*, repository::timeslot_repo::TimeSlotRepository};

#[derive(Clone)]
pub struct SqliteTimeSlotRepository {
  pool: Pool<Sqlite>,
}

impl SqliteTimeSlotRepository {
  pub fn new(pool: Pool<Sqlite>) -> Self {
    SqliteTimeSlotRepository { pool }
  }
}

impl TimeSlotRepository for SqliteTimeSlotRepository {
  async fn is_overlapping_with_unavailable_timeslot(
    &self,
    timeslot: TimeSlot,
  ) -> Result<bool, Status> {
    let result = handle_sqlx(
    query_scalar!(
      "SELECT EXISTS(
        SELECT 1 FROM UnavailableTimeSlots
        WHERE 
          (MAX(datetime(?, 'unixepoch', '+8 hours'), start_time) < MIN(datetime(?, 'unixepoch', '+8 hours'), end_time))
      )",
      timeslot.start_time,
      timeslot.end_time
    )
    .fetch_one(&self.pool)
    .await,
    "Checking if the timeslot is overlapping unavailable timeslots",
  )?;

    let overlapping: bool = result.map_or(false, |count| count != 0);

    Ok(overlapping)
  }

  async fn is_within_unavailable_timeslot(&self, time: i64) -> Result<bool, Status> {
    let result = handle_sqlx(
      query_scalar!(
        "SELECT EXISTS(
        SELECT 1 FROM UnavailableTimeSlots
        WHERE 
          start_time <= datetime(?, 'unixepoch', '+8 hours') AND 
          end_time > datetime(?, 'unixepoch', '+8 hours')
      )",
        time,
        time
      )
      .fetch_one(&self.pool)
      .await,
      "Checking if the time is within any unavailable timeslots",
    )?;

    let is_within_timeslot: bool = result.map_or(false, |count| count != 0);

    Ok(is_within_timeslot)
  }

  async fn insert_timeslot(&self, timeslot: TimeSlot) -> Result<(), Status> {
    let mut tx = handle_sqlx((&self.pool).begin().await, "Starting new transaction")?;

    handle_sqlx(
      query!(
        "INSERT INTO UnavailableTimeSlots 
        (start_time, end_time) 
      VALUES 
        (datetime(?, 'unixepoch', '+8 hours'), datetime(?, 'unixepoch', '+8 hours'))",
        timeslot.start_time,
        timeslot.end_time
      )
      .execute(&mut *tx)
      .await,
      "Inserting timeslot",
    )?;

    handle_sqlx(tx.commit().await, "Committing transaction")?;

    Ok(())
  }
}
