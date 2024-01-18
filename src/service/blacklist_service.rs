use super::common::*;
use crate::{model::blacklist::*, repository::blacklist_repo::*};

#[derive(Clone)]
pub struct BlacklistService<B: BlacklistRepository> {
  blacklist_repository: B,
}

impl<B: BlacklistRepository> BlacklistService<B> {
  pub fn new(blacklist_repository: B) -> Self {
    BlacklistService {
      blacklist_repository,
    }
  }

  pub async fn remove_user_from_blacklist(&self, user_id_to_unban: i64) -> Result<(), Status> {
    Ok(())
  }
}
