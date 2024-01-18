use super::common::*;
use crate::{model::timeslot::*, repository::timeslot_repo::*};

#[derive(Clone)]
pub struct TimeSlotService<T: TimeSlotRepository> {
  timeslot_repository: T,
}

impl<T: TimeSlotRepository> TimeSlotService<T> {
  pub fn new(timeslot_repository: T) -> Self {
    TimeSlotService {
      timeslot_repository,
    }
  }

  pub async fn set_unavailable_timeslots(&self, timeslot: TimeSlot) -> Result<(), Status> {
    self.timeslot_repository.insert_timeslot(timeslot).await
  }


}
