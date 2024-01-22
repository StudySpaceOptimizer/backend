use rocket::async_trait;

use super::common::*;
use crate::model::timeslot::*;

/*
Repository 層：
負責直接與數據庫交互
定義 timeslot Repository 的 interface
*/
pub trait TimeSlotRepository {
  async fn is_overlapping_with_unavailable_timeslot(
    &self,
    timeslot: TimeSlot,
  ) -> Result<bool, Status>;

  async fn is_within_unavailable_timeslot(&self, time: i64) -> Result<bool, Status>;

  async fn insert_timeslot(&self, timeslot: TimeSlot) -> Result<(), Status>;
}
