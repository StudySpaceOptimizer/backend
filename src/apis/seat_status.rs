use super::{common::*, validate_utils::*};
use crate::model::{seat, timeslot};

// 查詢當前所有位置狀態
/*
如果座位(Seats)不可用，則該座位的狀態為Unavailable
如果特定時間被包含在某筆預約中，則則該座位的狀態為Borrowed
否則為Available
*/
#[get("/api/seats/status")]
pub async fn show_current_seats_status(
  app: &State<App>,
) -> Result<Json<seat::SeatsOverview>, Status> {
  log::info!("Retrieving current seats status");

  let seat_overview = app.seat_service.get_seats_overview().await?;

  log::info!("Current seats status retrieved successfully");
  Ok(Json(seat_overview))
}

// 查詢當前所有位置狀態 + filter
/*
如果給定的時間段中有重疊到被預約的時段則回傳
Borrowed
否則回傳
Available
*/
#[get("/api/seats/status/<start_time>/<end_time>")]
pub async fn show_seats_status_in_specific_timeslots(
  app: &State<App>,
  start_time: i64,
  end_time: i64,
) -> Result<Json<seat::SeatsOverview>, Status> {
  let timeslot = timeslot::TimeSlot {
    start_time,
    end_time,
  };
  handle_validator(
    validate_timeslot(&timeslot).map_err(|e| convert_to_validation_errors(e, "timeslot")),
  )?;

  log::info!(
    "Retrieving seats status for time range {} to {}",
    start_time,
    end_time
  );

  let seat_reserving_statuses = app
    .seat_service
    .get_seats_overview_in_timeslot(timeslot)
    .await?;

  log::info!("Seats status for specified time range retrieved successfully");
  Ok(Json(seat_reserving_statuses))
}
