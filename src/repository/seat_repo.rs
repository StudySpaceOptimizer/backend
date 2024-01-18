use super::common::*;
use crate::model::{seat::*, timeslot::*};

/*
Repository 層：
負責直接與數據庫交互
定義 Seat Repository 的 interface
*/
pub trait SeatRepository {
  async fn query_current_seat_statuses(&self) -> Result<Vec<(u16, String)>, Status>;
  async fn query_seat_statuses_in_specific_timeslot(
    &self,
    timeslot: TimeSlot,
  ) -> Result<Vec<(u16, String)>, Status>;
  async fn query_seat(&self, seat_id: u16) -> Result<Seat, Status>;
  async fn update_seat(
    &self,
    seat_id: u16,
    available: bool,
    other_info: Option<String>,
  ) -> Result<(), Status>;
  async fn insert_seat(
    &self,
    seat_id: u16,
    available: bool,
    other_info: Option<String>,
  ) -> Result<(), Status>;
}
