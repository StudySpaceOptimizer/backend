use super::common::*;
use crate::{
  model::{seat::*, timeslot::*},
  repository::seat_repo::*,
};

#[derive(Clone)]
pub struct SqliteSeatRepository {
  pool: Pool<Sqlite>,
}

impl SqliteSeatRepository {
  pub fn new(pool: Pool<Sqlite>) -> Self {
    SqliteSeatRepository { pool }
  }
}

#[async_trait]
impl SeatRepository for SqliteSeatRepository {
  // 查詢所有位置目前狀態
  async fn query_current_seat_statuses(&self) -> Result<Vec<(u16, String)>, Status> {
    let sql = "
    SELECT 
      Seats.seat_id,
      CASE
        WHEN Seats.available = 0 THEN 'Unavailable'
        WHEN Reservations.seat_id IS NULL THEN 'Available'
        ELSE 'Borrowed'
      END as status
    FROM 
      Seats
    LEFT JOIN Reservations ON 
      Seats.seat_id = Reservations.seat_id AND
      Reservations.start_time <= datetime(strftime('%s','now'), 'unixepoch', '+8 hours') AND
      Reservations.end_time > datetime(strftime('%s','now'), 'unixepoch', '+8 hours')";

    // 取得每個座位的狀態，回傳為vector包含(座位號碼, 狀態)
    let result: Vec<(u16, String)> = handle_sqlx(
      sqlx::query_as::<_, (u16, String)>(sql)
        .fetch_all(&self.pool)
        .await,
      "Querying current seat statuses",
    )?;

    Ok(result)
  }

  // 查詢所有位置在特定時間段狀態
  async fn query_seat_statuses_in_specific_timeslot(
    &self,
    timeslot: TimeSlot,
  ) -> Result<Vec<(u16, String)>, Status> {
    let sql = "
    SELECT DISTINCT 
      Seats.seat_id,
      CASE
        WHEN Seats.available = 0 THEN 'Unavailable'
        WHEN Reservations.seat_id IS NULL THEN 'Available'
        ELSE 'Borrowed'
      END as status
    FROM 
      Seats
    LEFT JOIN Reservations ON 
      Seats.seat_id = Reservations.seat_id AND
      (MAX(datetime(?, 'unixepoch', '+8 hours'), start_time) < MIN(datetime(?, 'unixepoch', '+8 hours'), end_time))";

    let result: Vec<(u16, String)> = handle_sqlx(
      sqlx::query_as::<_, (u16, String)>(sql)
        .bind(timeslot.start_time)
        .bind(timeslot.end_time)
        .fetch_all(&self.pool)
        .await,
      "Querying seat statuses in specific timeslot",
    )?;

    Ok(result)
  }

  async fn query_seat(&self, seat_id: u16) -> Result<Seat, Status> {
    let sql = "SELECT * FROM Seats WHERE seat_id = ?";

    let seat: Seat = handle_sqlx(
      sqlx::query_as::<_, Seat>(sql)
        .bind(seat_id)
        .fetch_one(&self.pool)
        .await,
      "Selecting seat",
    )?;

    Ok(seat)
  }

  async fn update_seat(
    &self,
    seat_id: u16,
    available: bool,
    other_info: Option<String>,
  ) -> Result<(), Status> {
    let affected_rows = handle_sqlx(
      query!(
        "UPDATE Seats
      SET
        available = ?,
        other_info = ?
      WHERE 
        seat_id = ? ",
        available,
        other_info,
        seat_id,
      )
      .execute(&self.pool)
      .await,
      "Updating seat",
    )?
    .rows_affected();

    // 檢查是否有成功更新
    // affected_rows == 0，此次操作無作用到任何資料
    if affected_rows == 0 {
      log::warn!("No seat found for updation");

      return Err(Status::NotFound);
    }

    Ok(())
  }

  async fn insert_seat(
    &self,
    seat_id: u16,
    available: bool,
    other_info: Option<String>,
  ) -> Result<(), Status> {
    handle_sqlx(
      query!(
        "INSERT OR IGNORE INTO Seats (seat_id, available, other_info) VALUES (?1, ?2, ?3)",
        seat_id,
        available,
        other_info
      )
      .execute(&self.pool)
      .await,
      "Inserting seat",
    )?;

    Ok(())
  }
}
