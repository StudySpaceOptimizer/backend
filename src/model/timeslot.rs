use super::common::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct TimeSlot {
  pub start_time: i64,
  pub end_time: i64,
}
