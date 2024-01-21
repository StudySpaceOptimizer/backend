use super::common::*;
use crate::model::user::*;

/*
Repository 層：
負責直接與數據庫交互
定義 User Repository 的 interface
*/
#[async_trait]
pub trait UserRepository {
  async fn insert_user(
    &self,
    email: &str,
    password_hash: &str,
    user_role: UserRole,
    verified: bool,
    verification_token: &str,
  ) -> Result<(), Status>;
  async fn query_user_by_email(&self, email: &str) -> Result<User, Status>;
  async fn query_user_by_id(&self, user_id: i64) -> Result<User, Status>;
  async fn update_verified(&self, verification_token: &str) -> Result<(), Status>;
}
