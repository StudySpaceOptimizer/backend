use super::common::*;
use crate::{
  model::{reservation::*, timeslot::*},
  repository::reservation_repo::ReservationRepository,
};

#[derive(Clone)]
pub struct SqliteReservationRepository {
  pool: Pool<Sqlite>,
}

impl SqliteReservationRepository {
  pub fn new(pool: Pool<Sqlite>) -> Self {
    SqliteReservationRepository { pool }
  }
}

impl ReservationRepository for SqliteReservationRepository {
  // 查詢特定位置狀態
  async fn query_seat_reservations(
    &self,
    seat_id: u16,
    timeslot: TimeSlot,
  ) -> Result<Vec<Reservation>, Status> {
    let sql = "
    SELECT
      reservation_id,
      user_id,
      CAST(strftime('%s', start_time, '-8 hours') AS INTEGER) as start_time, 
      CAST(strftime('%s', end_time, '-8 hours') AS INTEGER) as end_time
      seat_id,
      check_in_time,
      check_out_time
    FROM 
      Reservations
    WHERE
      seat_id = ? AND 
      start_time >= datetime(?, 'unixepoch', '+8 hours') AND 
      end_time <= datetime(?, 'unixepoch', '+8 hours')";

    let timeslots: Vec<Reservation> = handle_sqlx(
      sqlx::query_as::<_, Reservation>(sql)
        .bind(seat_id)
        .bind(timeslot.start_time)
        .bind(timeslot.end_time)
        .fetch_all(&self.pool)
        .await,
      "Querying seat reservations",
    )?;

    Ok(timeslots)
  }

  async fn query_user_reservations(&self, user_id: i64) -> Result<Vec<Reservation>, Status> {
    /*
    獲取使用者的預約紀錄
     */
    let sql = "
    SELECT 
      reservation_id,
      user_id,
      CAST(strftime('%s', start_time, '-8 hours') AS INTEGER) as start_time, 
      CAST(strftime('%s', end_time, '-8 hours') AS INTEGER) as end_time
      seat_id,
      check_in_time,
      check_out_time
    FROM 
      Reservations 
    WHERE 
      user_id = ? AND 
      end_time > datetime(strftime('%s','now'), 'unixepoch', '+8 hours')";

    // 搜尋使用者今天之後的預約紀錄
    let reservations = handle_sqlx(
      query_as::<_, Reservation>(sql)
        .bind(user_id)
        .fetch_all(&self.pool)
        .await,
      "Querying user reservations",
    )?;

    Ok(reservations)
  }

  // 預約座位
  async fn insert_reservation(
    &self,
    user_id: i64,
    seat_id: u16,
    timeslot: TimeSlot,
  ) -> Result<(), Status> {
    // 使用transaction
    let mut tx = handle_sqlx((&self.pool).begin().await, "Starting new transaction")?;

    // 查詢時間段是否重疊
    let result: Option<i32> = handle_sqlx(
    query_scalar!(
      "SELECT EXISTS(
        SELECT 1 FROM Reservations
        WHERE 
          seat_id = ? AND 
          (MAX(datetime(?, 'unixepoch', '+8 hours'), start_time) < MIN(datetime(?, 'unixepoch', '+8 hours'), end_time))
    )",
      seat_id,
      timeslot.start_time,
      timeslot.end_time
    )
    .fetch_one(&mut *tx)
    .await,
    "Querying overlapping reservations",
  )?;
    let overlapping: bool = result.map_or(false, |count| count != 0);

    // 如果重疊
    if overlapping {
      log::warn!(
        "The start_time: {} end_time: {} is overlapping with user's reservation",
        timeslot.start_time,
        timeslot.end_time
      );

      // rollback
      handle_sqlx(tx.rollback().await, "Rolling back")?;
      return Err(Status::Conflict);
    }

    // 新增一筆預約
    handle_sqlx(
      query!(
        "INSERT INTO Reservations 
        (user_id, seat_id, start_time, end_time, check_in_time, check_out_time) 
      VALUES 
        (
          ?, 
          ?, 
          datetime(?, 'unixepoch', '+8 hours'), 
          datetime(?, 'unixepoch', '+8 hours'), 
          NULL, 
          NULL
        )",
        user_id,
        seat_id,
        timeslot.start_time,
        timeslot.end_time
      )
      .execute(&mut *tx)
      .await,
      "Inserting reservation",
    )?;

    // 完成整筆transaction
    handle_sqlx(tx.commit().await, "Committing transaction")?;

    Ok(())
  }

  async fn delete_reservation(&self, reservation_id: i64) -> Result<(), Status> {
    // 執行刪除
    let affected_rows = handle_sqlx(
      query!(
        "DELETE FROM Reservations WHERE reservation_id = ?",
        reservation_id
      )
      .execute(&self.pool)
      .await,
      "Deleting reservation",
    )?
    .rows_affected();

    if affected_rows == 0 {
      log::warn!("No reservation found for deletion");
      return Err(Status::NotFound);
    }

    Ok(())
  }

  async fn query_reservation_by_id(&self, reservation_id: i64) -> Result<Reservation, Status> {
    let sql = "
    SELECT 
      reservation_id,
      user_id,
      CAST(strftime('%s', start_time, '-8 hours') AS INTEGER) as start_time, 
      CAST(strftime('%s', end_time, '-8 hours') AS INTEGER) as end_time
      seat_id,
      check_in_time,
      check_out_time
    FROM 
      Reservations 
    WHERE 
      reservation_id = ?";

    // 搜尋使用者今天之後的預約紀錄
    let reservation = handle_sqlx(
      query_as::<_, Reservation>(sql)
        .bind(reservation_id)
        .fetch_one(&self.pool)
        .await,
      "Querying user reservations",
    )?;

    Ok(reservation)
  }

  // // 修改預約紀錄
  // async fn update_reservation_time(
  //    &self,
  //   user_name: &str,
  //   start_time: i64,
  //   end_time: i64,
  //   new_start_time: i64,
  //   new_end_time: i64,
  // ) -> Result<(), Status> {
  //   // 使用transaction
  //   let mut tx = handle_sqlx(pool.begin().await, "Starting new transaction")?;

  //   // 查詢時間段是否重疊
  //   let result: Option<i32> = handle_sqlx(
  //     query_scalar!(
  //       "SELECT EXISTS(
  //           SELECT 1 FROM Reservations
  //           WHERE
  //             user_name != ? AND
  //             (MAX(datetime(?, 'unixepoch', '+8 hours'), start_time) < MIN(datetime(?, 'unixepoch', '+8 hours'), end_time))
  //       )",
  //       user_name,
  //       new_start_time,
  //       new_end_time
  //     )
  //     .fetch_one(&mut *tx)
  //     .await,
  //     "Checking for overlapping reservations",
  //   )?;
  //   let overlapping: bool = result.map_or(false, |count| count != 0);

  //   // 如果重疊
  //   if overlapping {
  //     log::warn!("Found overlapping reservation");

  //     // rollback
  //     handle_sqlx(tx.rollback().await, "Rolling back")?;
  //     return Err(Status::Conflict);
  //   }

  //   // 執行更新
  //   let affected_rows = handle_sqlx(
  //     query!(
  //       "UPDATE Reservations
  //       SET
  //         start_time = datetime(?, 'unixepoch', '+8 hours'),
  //         end_time = datetime(?, 'unixepoch', '+8 hours')
  //       WHERE
  //         user_name = ? AND
  //         start_time = datetime(?, 'unixepoch', '+8 hours') AND
  //         end_time = datetime(?, 'unixepoch', '+8 hours')",
  //       new_start_time,
  //       new_end_time,
  //       user_name,
  //       start_time,
  //       end_time,
  //     )
  //     .execute(&mut *tx)
  //     .await,
  //     "Updating reservation",
  //   )?
  //   .rows_affected();

  //   // 檢查是否有成功更新
  //   // affected_rows == 0，此次操作無作用到任何資料
  //   if affected_rows == 0 {
  //     log::warn!("No reservation found for updation");

  //     // rollback
  //     handle_sqlx(tx.rollback().await, "Rolling back")?;
  //     return Err(Status::NotFound);
  //   } else {
  //     // 完成整筆transaction
  //     handle_sqlx(tx.commit().await, "Committing transaction")?;
  //   }

  //   Ok(())
  // }
}
