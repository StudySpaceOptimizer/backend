use super::common::*;
use crate::model::{reservation::*, timeslot::*};

/*
Repository 層：
負責直接與數據庫交互
定義 reservation Repository 的 interface
*/

pub trait ReservationRepository {
  async fn query_seat_reservations(
    &self,
    seat_id: u16,
    timeslot: TimeSlot,
  ) -> Result<Vec<Reservation>, Status>;

  async fn query_user_reservations(&self, user_id: i64) -> Result<Vec<Reservation>, Status>;

  async fn insert_reservation(
    &self,
    user_id: i64,
    seat_id: u16,
    timeslot: TimeSlot,
  ) -> Result<(), Status>;

  async fn delete_reservation(&self, reservation_id: i64) -> Result<(), Status>;
  async fn query_reservation_by_id(&self, reservation_id: i64) -> Result<Reservation, Status>;
}
