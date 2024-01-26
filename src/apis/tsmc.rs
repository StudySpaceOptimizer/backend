use super::{common::*, validate_utils::*};
use reqwest::StatusCode;
use sqlx::Sqlite;
use std::{env, fs};
use tokio::time::{sleep, Duration};

/**
 * This file is part of the TSMC hackathon demo use, so it is not included in the original project.
 *
 * @file   tsmc.rs
 * @since  2024-01-20 ~ 2024-01-27
 */

// 獲取狀態碼
#[get("/api/tsmc/reservations/<reservation_id>")]
pub fn get_status_code(reservation_id: u16) -> Result<(), Status> {
  log::info!(
    "Starting to delete reservation: Reservation: {}",
    reservation_id
  );

  let status = match reservation_id {
    200 => Ok(()),
    404 => Err(Status::NotFound),
    400 => Err(Status::BadRequest),
    503 => Err(Status::ServiceUnavailable),
    _ => Err(Status::InternalServerError),
  };

  log::info!(
    "Completed deleting reservation successfully: Reservation ID: {}",
    reservation_id
  );

  return status;
}

// 斷開資料庫連接
#[get("/api/tsmc/seats/status")]
pub async fn disconnect_db(pool: &State<sqlx::Pool<Sqlite>>) -> Result<(), Status> {
  log::info!("Starting to show current seats status");
  pool.close().await;
  log::info!("Completed showing current seats status successfully");
  Ok(())
}

// 處理超時
#[get("/api/tsmc/timeslots/<time>")]
pub async fn timeout(time: u64) -> Result<(), Status> {
  log::info!("Starting to set unavailable timeslots",);

  sleep(Duration::from_millis(time)).await;

  log::info!("Completed setting unavailable timeslots successfully");
  Ok(())
}

// 使用大量記憶體
#[get("/api/tsmc/seats/info/<seat_id>")]
pub fn big_memory(seat_id: usize) -> Result<(), Status> {
  log::info!("Starting to set seat info: Seat: {}", seat_id);

  let mut v = Vec::with_capacity(seat_id);
  v.resize(seat_id, 0);

  log::info!(
    "Completed setting seat info successfully: Seat: {}",
    seat_id
  );
  Ok(())
}

// 使用大量 CPU
#[get("/api/tsmc/users/reservations")]
pub fn big_cpu() -> Result<(), Status> {
  log::info!("Starting to display reservations");

  let mut v = Vec::with_capacity(100000000);
  v.resize(100000000, 0);
  let mut sum = 0;
  for i in 0..100000000 {
    for j in 0..100000000 {
      sum += i * j % 1000000007;
      v[i] = sum;
    }
  }

  log::info!("Completed displaying reservations successfully");
  Ok(())
}

// 併發錯誤
#[get("/api/tsmc/reservations/reserve")]
pub fn concurrent_error() -> Result<(), Status> {
  log::info!("Starting to reserve seat: User: 123, Seat: 5");
  log::info!("Starting to reserve seat: User: 456, Seat: 5");
  log::info!("Starting to reserve seat: User: 789, Seat: 5");
  log::error!("Inserting reservation failed with error: Seat 5 allocation conflict");
  log::error!("Inserting reservation failed with error: Seat 5 allocation conflict");
  log::error!("Inserting reservation failed with error: Seat 5 allocation conflict");
  log::error!("Failed to resolve seat allocation conflict automatically");
  Err(Status::InternalServerError)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusCodeResponse {
  pub code: u16,
  pub name: String,
}
