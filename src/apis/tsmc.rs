use std::{env, fs};

use reqwest::StatusCode;
use tokio::time::{sleep, Duration};

/**
 * This file is part of the TSMC hackathon demo use, so it is not included in the original project.
 *
 * @file   tsmc.rs
 * @since  2024-01-20 ~ 2024-01-27
 */
use super::{common::*, validate_utils::*};

#[get("/api/statuscode/<status_code>")]
pub fn get_status_code(status_code: u16) -> Result<Json<StatusCodeResponse>, Status> {
  let status_code = StatusCode::from_u16(status_code).map_err(|_| Status::BadRequest)?;
  Ok(Json(StatusCodeResponse {
    code: status_code.as_u16(),
    name: status_code.canonical_reason().unwrap_or("").to_string(),
  }))
}

#[get("/api/disconnectdb")]
pub async fn disconnect_db() -> Result<(), Status> {
  let path = env::var("DATABASE_URL").map_err(|_| Status::InternalServerError)?;
  fs::remove_file(path).map_err(|_| Status::InternalServerError)?;
  Ok(())
}

#[get("/api/timeout/<timeout>")]
pub async fn timeout(timeout: u64) -> Result<(), Status> {
  sleep(Duration::from_millis(timeout)).await;
  Ok(())
}

#[get("/api/bigMemory/<size>")]
pub fn big_memory(size: usize) -> Result<(), Status> {
  let mut v = Vec::with_capacity(size);
  v.resize(size, 0);
  Ok(())
}

#[get("/api/bigCPU")]
pub fn big_cpu() -> Result<(), Status> {
  let mut v = Vec::with_capacity(100000000);
  v.resize(100000000, 0);
  let mut sum = 0;
  for i in 0..100000000 {
    for j in 0..100000000 {
      sum += i * j % 1000000007;
      v[i] = sum;
    }
  }
  Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct StatusCodeResponse {
  code: u16,
  name: String,
}
