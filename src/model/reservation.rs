use super::{common::*, timeslot::*};

#[derive(Debug, Deserialize, Serialize)]
pub struct Reservation {
  pub reservation_id: i64,
  pub user_id: i64,
  pub timeslot: TimeSlot,
  pub seat_id: u16,
  pub check_in_time: Option<i64>,
  pub check_out_time: Option<i64>,
}

impl FromRow<'_, SqliteRow> for Reservation {
  fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
    let reservation_id = row.try_get("reservation_id")?;
    let user_id = row.try_get("user_id")?;
    let start_time = row.try_get("start_time")?;
    let end_time = row.try_get("end_time")?;
    let seat_id = row.try_get("seat_id")?;
    let check_in_time = row.try_get("check_in_time")?;
    let check_out_time = row.try_get("check_out_time")?;
    Ok(Reservation {
      reservation_id: reservation_id,
      user_id: user_id,
      timeslot: TimeSlot {
        start_time,
        end_time,
      },
      seat_id: seat_id,
      check_in_time: check_in_time,
      check_out_time: check_out_time,
    })
  }
}

// 修改reservation

// fn validate_update_reservation_request(
//   request: &UpdateReservationRequest,
// ) -> Result<(), ValidationError> {
//   let start_time = request.start_time;
//   let end_time = request.end_time;
//   let new_start_time = request.new_start_time;
//   let new_end_time = request.new_end_time;

//   validate_datetime(start_time, end_time)?;
//   validate_datetime(new_start_time, new_end_time)?;
//   on_the_same_day(start_time, new_start_time)
// }

// #[derive(Debug, Deserialize, Serialize, Validate)]
// #[validate(schema(
//   function = "validate_update_reservation_request",
//   skip_on_field_errors = false
// ))]
// pub struct UpdateReservationRequest {
//   pub start_time: i64,
//   pub end_time: i64,
//   pub new_start_time: i64,
//   pub new_end_time: i64,
// }
