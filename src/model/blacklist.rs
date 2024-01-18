use super::{common::*, timeslot::*};

#[derive(Debug, Serialize, Deserialize)]
struct Blacklist {
  blacklisted_users: Vec<BlacklistedUser>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BlacklistedUser {
  user_id: i64,
  email: String,
  timeslot: TimeSlot,
}
