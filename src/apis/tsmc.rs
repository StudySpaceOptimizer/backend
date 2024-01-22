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
#[get("/api/statuscode/<status_code>")]
pub fn get_status_code(status_code: u16) -> Result<Json<StatusCodeResponse>, Status> {
  log::info!("Starting to get status code: status code: {}", status_code);

  let status_code = StatusCode::from_u16(status_code).map_err(|_| Status::BadRequest)?;

  log::info!(
    "Completed getting status code successfully: status code: {}",
    status_code.as_u16()
  );
  Ok(Json(StatusCodeResponse {
    code: status_code.as_u16(),
    name: status_code.canonical_reason().unwrap_or("").to_string(),
  }))
}

// 斷開資料庫連接
#[get("/api/disconnectdb")]
pub async fn disconnect_db(pool: &State<sqlx::Pool<Sqlite>>) -> Result<(), Status> {
  // log::info!("Starting to disconnect from database");
  pool.close().await;
  // log::info!("Completed disconnecting from database successfully");
  Ok(())
}

// 處理超時
#[get("/api/timeout/<timeout>")]
pub async fn timeout(timeout: u64) -> Result<(), Status> {
  log::info!("Starting to handle timeout: {}", timeout);

  sleep(Duration::from_millis(timeout)).await;

  log::info!("Completed handling timeout successfully");
  Ok(())
}

// 使用大量記憶體
#[get("/api/big_memory/<size>")]
pub fn big_memory(size: usize) -> Result<(), Status> {
  log::info!("Starting to allocate big memory: Size: {}", size);

  let mut v = Vec::with_capacity(size);
  v.resize(size, 0);

  log::info!("Completed big memory allocation successfully");
  Ok(())
}

// 使用大量 CPU
#[get("/api/big_cpu")]
pub fn big_cpu() -> Result<(), Status> {
  log::info!("Starting to perform big CPU operation");

  let mut v = Vec::with_capacity(100000000);
  v.resize(100000000, 0);
  let mut sum = 0;
  for i in 0..100000000 {
    for j in 0..100000000 {
      sum += i * j % 1000000007;
      v[i] = sum;
    }
  }

  log::info!("Completed big CPU operation successfully");
  Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusCodeResponse {
  pub code: u16,
  pub name: String,
}
