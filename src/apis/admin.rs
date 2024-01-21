use super::{common::*, validate_utils::*};
use crate::model::{timeslot, token, user};

// 設定不可預約時間
#[post("/api/timeslots/unavailable", format = "json", data = "<request>")]
pub async fn set_unavailable_timeslots(
  app: &State<App>,
  claim: token::UserClaim,
  request: Json<InsertTimeSlotRequest>,
) -> Result<(), Status> {
  handle_validator(request.validate())?;
  log::info!(
    "Starting to set unavailable timeslots: User: {}, start_time: {}, end_time: {}",
    claim.user_id,
    request.timeslot.start_time,
    request.timeslot.end_time
  );

  if claim.user_role != user::UserRole::Admin {
    log::warn!(
      "Unauthorized access attempt: User {} tried to set unavailable timeslots",
      claim.user_id
    );
    return Err(Status::Unauthorized);
  }

  app
    .timeslot_service
    .set_unavailable_timeslots(request.timeslot)
    .await?;

  log::info!(
    "Completed setting unavailable timeslots successfully:Start_time: {}, end_time: {}",
    request.timeslot.start_time,
    request.timeslot.end_time
  );
  Ok(())
}

// 設定不可使用座位
#[post("/api/seats/info", format = "json", data = "<request>")]
pub async fn set_seat_info(
  app: &State<App>,
  claim: token::UserClaim,
  request: Json<UpdateSeatRequest>,
) -> Result<(), Status> {
  handle_validator(request.validate())?;
  log::info!(
    "Starting to set seat info: User: {} seat: {}",
    claim.user_id,
    request.seat_id
  );

  if claim.user_role != user::UserRole::Admin {
    log::warn!(
      "Unauthorized access attempt: User {} tried to set seat info for seat {}",
      claim.user_id,
      request.seat_id
    );
    return Err(Status::Unauthorized);
  }

  app
    .seat_service
    .set_seat_info(
      request.seat_id,
      request.available,
      request.other_info.clone(),
    )
    .await?;

  log::info!(
    "Completed setting seat info successfully: Seat: {}",
    request.seat_id
  );
  Ok(())
}

// 加點
#[post("/api/users/violation_points", format = "json", data = "<request>")]
pub async fn increase_violation_points(
  app: &State<App>,
  claim: token::UserClaim,
  request: Json<AddPointsRequest>,
) -> Result<(), Status> {
  handle_validator(request.validate())?;

  log::info!(
    "Starting to increase violation points: User:{}, user to be incremented: {}",
    claim.user_id,
    request.user_id
  );

  if claim.user_role != user::UserRole::Admin {
    log::warn!(
      "Unauthorized access attempt: User {} tried to increase violation points",
      claim.user_id,
    );
    return Err(Status::Unauthorized);
  }

  app
    .user_service
    .add_violation_points(request.user_id, request.points)
    .await?;

  log::info!(
    "Completed increasing violation points successfully: User to be incremented: {}",
    request.user_id
  );
  Ok(())
}

#[delete("/api/users/<user_id_to_unban>/blacklist")]
pub async fn remove_user_from_blacklist(
  app: &State<App>,
  claim: token::UserClaim,
  user_id_to_unban: i64,
) -> Result<(), Status> {
  log::info!(
    "Starting to remove user from blacklist: User: {}, user to be removed: {}",
    claim.user_id,
    user_id_to_unban
  );

  if claim.user_role != user::UserRole::Admin {
    log::warn!(
      "Unauthorized access attempt: User {} tried to remove user {} from blacklist",
      claim.user_id,
      user_id_to_unban
    );
    return Err(Status::Unauthorized);
  }

  app
    .blacklist_service
    .remove_user_from_blacklist(user_id_to_unban)
    .await?;

  log::info!(
    "Completed removing user from blacklist successfully: User to be removed: {}",
    user_id_to_unban
  );
  Ok(())
}

// 修改座位資訊
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateSeatRequest {
  #[validate(custom = "validate_seat_id")]
  pub seat_id: u16,
  pub available: bool,
  pub other_info: Option<String>,
}

// 新增 TimeSlot
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct InsertTimeSlotRequest {
  #[validate(custom = "validate_timeslot")]
  pub timeslot: timeslot::TimeSlot,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct AddPointsRequest {
  pub user_id: i64,
  #[validate(custom = "validate_points")]
  pub points: i64,
}
