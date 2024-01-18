use super::{common::*, validate_utils::*};
use crate::model::{reservation, timeslot, token};

// 預約座位
#[post("/api/reservations", format = "json", data = "<request>")]
pub async fn reserve_seat(
  app: &State<App>,
  claims: token::UserClaim,
  request: Json<InsertReservationRequest>,
) -> Result<(), Status> {
  handle_validator(request.validate())?;

  log::info!(
    "User {} attempting to reserve seat {}",
    claims.user_id,
    request.seat_id
  );
  app
    .reservation_service
    .reserve_seat(claims.user_id, request.seat_id, request.timeslot)
    .await?;

  log::info!(
    "Seat {} reserved successfully for user {}",
    request.seat_id,
    claims.user_id
  );
  Ok(())
}

// 刪除預約時段
#[delete("/api/reservations/<reservation_id>")]
pub async fn delete_reservation(
  app: &State<App>,
  claims: token::UserClaim,
  reservation_id: i64,
) -> Result<(), Status> {
  log::info!(
    "User {} deleting reservation id: {}",
    claims.user_id,
    reservation_id
  );
  app
    .reservation_service
    .delete_reservation(claims.user_id, reservation_id)
    .await?;

  log::info!(
    "Reservation deleted successfully for user {}",
    claims.user_id
  );
  Ok(())
}

// 顯示使用者預約時段
#[get("/api/users/reservations")]
pub async fn display_user_reservations(
  app: &State<App>,
  claims: token::UserClaim,
) -> Result<Json<Vec<reservation::Reservation>>, Status> {
  log::info!("Displaying reservations for user: {}", claims.user_id);

  let reservations: Vec<reservation::Reservation> = app
    .reservation_service
    .get_user_reservations(claims.user_id)
    .await?;

  log::info!(
    "Displayed reservations successfully for user {}",
    claims.user_id
  );
  Ok(Json(reservations))
}

// 查詢當前特定位置預約狀態
#[get("/api/seats/<seat_id>/reservations/<start_time>/<end_time>")]
pub async fn show_seat_reservations(
  app: &State<App>,
  seat_id: u16,
  start_time: i64,
  end_time: i64,
) -> Result<String, Status> {
  handle_validator(
    validate_seat_id(seat_id).map_err(|e| convert_to_validation_errors(e, "seat_id")),
  )?;

  log::info!("Show reservations of seat: {}", seat_id);

  let timeslots: Vec<reservation::Reservation> = app
    .reservation_service
    .get_seat_reservations(
      seat_id,
      timeslot::TimeSlot {
        start_time,
        end_time,
      },
    )
    .await?;

  let json = handle(
    serde_json::to_string(&timeslots),
    "Serialize the data as a String of JSON",
  )?;

  log::info!("Show reservations successfully for seat {}", seat_id);

  Ok(json)
}

// 新增 Reservation
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct InsertReservationRequest {
  #[validate(custom = "validate_seat_id")]
  pub seat_id: u16,
  #[validate(custom = "validate_reservation_timeslot")]
  pub timeslot: timeslot::TimeSlot,
}
