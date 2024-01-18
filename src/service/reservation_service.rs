use super::common::*;
use crate::{
  model::{reservation::*, timeslot::*},
  repository::{reservation_repo::*, timeslot_repo::*},
};
use chrono::Timelike;

#[derive(Clone)]
pub struct ReservationService<R: ReservationRepository, T: TimeSlotRepository> {
  reservation_repository: R,
  timeslot_repository: T,
}

impl<R: ReservationRepository, T: TimeSlotRepository> ReservationService<R, T> {
  pub fn new(reservation_repository: R, timeslot_repository: T) -> Self {
    ReservationService {
      reservation_repository,
      timeslot_repository,
    }
  }

  pub async fn reserve_seat(
    &self,
    user_id: i64,
    seat_id: u16,
    timeslot: TimeSlot,
  ) -> Result<(), Status> {
    if self
      .timeslot_repository
      .is_overlapping_with_unavailable_timeslot(timeslot.clone())
      .await?
    {
      log::warn!(
        "The start_time: {} end_time: {} is overlapping unavailable timeslot",
        timeslot.start_time,
        timeslot.end_time
      );
      return Err(Status::BadRequest);
    }

    let user_reservations = self
      .reservation_repository
      .query_user_reservations(user_id)
      .await?;


    
    // 在預約的最後一個時間段單位(30分鐘)中，使用者可以進行下一次預約
    for reservation in user_reservations {
      let now = get_now();
      let next_half_hour = if now.minute() >= 30 {
        naive_date_to_timestamp(now.date(), now.hour() + 1, 0, 0)?
      } else {
        naive_date_to_timestamp(now.date(), now.hour(), 30, 0)?
      };
      let end_of_day = naive_date_to_timestamp(now.date(), 23, 59, 59)?;

      // 預約結束時間 <= 下一個時間段單位: 可進行預約
      if next_half_hour < reservation.timeslot.end_time
        && reservation.timeslot.end_time <= end_of_day
      {
        log::warn!("Found ongoing reservation in {}", now.date());
        return Err(Status::BadRequest);
      }
    }

    self
      .reservation_repository
      .insert_reservation(user_id, seat_id, timeslot)
      .await?;

    Ok(())
  }

  pub async fn delete_reservation(&self, user_id: i64, reservation_id: i64) -> Result<(), Status> {
    let reservation = self
      .reservation_repository
      .query_reservation_by_id(reservation_id)
      .await
      .map_err(|_| Status::NotFound)?;

    if reservation.user_id != user_id {
      log::warn!(
        "User {} attempted to delete reservation {} which they do not own",
        user_id,
        reservation_id
      );
      return Err(Status::Forbidden);
    }

    self
      .reservation_repository
      .delete_reservation(reservation_id)
      .await?;

    Ok(())
  }

  pub async fn get_user_reservations(&self, user_id: i64) -> Result<Vec<Reservation>, Status> {
    self
      .reservation_repository
      .query_user_reservations(user_id)
      .await
  }

  pub async fn get_seat_reservations(
    &self,
    seat_id: u16,
    timeslot: TimeSlot,
  ) -> Result<Vec<Reservation>, Status> {
    self
      .reservation_repository
      .query_seat_reservations(seat_id, timeslot)
      .await
  }
}
