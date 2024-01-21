use super::common::*;
use crate::model::{blacklist::*, timeslot::*};

/*
Repository 層：
負責直接與數據庫交互
定義 Blacklist Repository 的 interface
*/
#[async_trait]
pub trait BlacklistRepository {
  async fn insert_user_to_blacklist(
    &self,
    user_name: &str,
    timeslot: TimeSlot,
  ) -> Result<(), Status>;

  async fn delete_user_from_blacklist(&self, user_name: &str) -> Result<(), Status>;
}
