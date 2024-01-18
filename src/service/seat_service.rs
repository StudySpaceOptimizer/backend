use std::str::FromStr;

use super::common::*;
use crate::{
  model::{constant::*, seat::*, timeslot::*},
  repository::{seat_repo::*, timeslot_repo::TimeSlotRepository},
};

#[derive(Clone)]
pub struct SeatService<S: SeatRepository, T: TimeSlotRepository> {
  seat_repository: S,
  timeslot_repository: T,
}

impl<R: SeatRepository, T: TimeSlotRepository> SeatService<R, T> {
  pub fn new(seat_repository: R, timeslot_repository: T) -> Self {
    SeatService {
      seat_repository,
      timeslot_repository,
    }
  }

  pub async fn get_seat_info(&self, seat_id: u16) -> Result<Seat, Status> {
    self.seat_repository.query_seat(seat_id).await
  }

  pub async fn set_seat_info(
    &self,
    seat_id: u16,
    available: bool,
    other_info: Option<String>,
  ) -> Result<(), Status> {
    self
      .seat_repository
      .update_seat(seat_id, available, other_info)
      .await
  }

  pub async fn create_seat(
    &self,
    seat_id: u16,
    available: bool,
    other_info: Option<String>,
  ) -> Result<(), Status> {
    self
      .seat_repository
      .insert_seat(seat_id, available, other_info)
      .await
  }

  pub async fn get_seats_overview(&self) -> Result<SeatsOverview, Status> {
    let now: i64 = get_now().timestamp();
    let seat_overview: SeatsOverview;
    let mut seats_vec = Vec::<SeatAvailability>::new();

    if self
      .timeslot_repository
      .is_within_unavailable_timeslot(now)
      .await?
    {
      for seat in 1..NUMBER_OF_SEATS {
        seats_vec.push(SeatAvailability {
          seat_id: seat,
          status: SeatAvailabilityStatus::Unavailable,
        })
      }

      seat_overview = SeatsOverview { seats: seats_vec };
    } else {
      let result = self.seat_repository.query_current_seat_statuses().await?;

      for (seat_id, status_str) in result {
        let status = match status_str.as_str() {
          "Available" => SeatAvailabilityStatus::Available,
          "Borrowed" => SeatAvailabilityStatus::Borrowed,
          "Unavailable" => SeatAvailabilityStatus::Unavailable,
          _ => return Err(Status::InternalServerError),
        };
        seats_vec.push(SeatAvailability { seat_id, status });
      }

      seat_overview = SeatsOverview { seats: seats_vec };
    }

    Ok(seat_overview)
  }

  pub async fn get_seats_overview_in_timeslot(
    &self,
    timeslot: TimeSlot,
  ) -> Result<SeatsOverview, Status> {
    let mut seats_vec = Vec::<SeatAvailability>::new();

    let result = self
      .seat_repository
      .query_seat_statuses_in_specific_timeslot(timeslot)
      .await?;

    for (seat_id, status_str) in result {
      let status = handle(
        SeatAvailabilityStatus::from_str(status_str.as_str()),
        "Parsing SeatAvailabilityStatus from str",
      )?;

      seats_vec.push(SeatAvailability { seat_id, status });
    }

    let seat_overview = SeatsOverview { seats: seats_vec };

    Ok(seat_overview)
  }
}
