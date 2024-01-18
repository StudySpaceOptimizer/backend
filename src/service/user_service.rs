use super::common::*;
use crate::{
  model::{token::*, user::*},
  repository::user_repo::*,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService<U: UserRepository> {
  user_repository: U,
}

impl<U: UserRepository> UserService<U> {
  pub fn new(user_repository: U) -> Self {
    UserService { user_repository }
  }

  pub async fn create_user(
    &self,
    email: &str,
    password: &str,
    user_role: UserRole,
  ) -> Result<User, Status> {
    let password_hash = handle(hash(password, DEFAULT_COST), "Hashing password")?;
    let verification_token = Uuid::new_v4().to_string();
    let verified = false;

    self
      .user_repository
      .insert_user(
        email,
        &password_hash,
        user_role,
        verified,
        &verification_token,
      )
      .await?;

    let user = self.user_repository.query_user_by_email(email).await?;

    Ok(user)
  }

  pub async fn login(&self, email: &str, password: &str) -> Result<User, Status> {
    let user = self.user_repository.query_user_by_email(email).await?;

    let is_verify =
      verify(password, &(user.password_hash)).map_err(|_| Status::InternalServerError)?;

    if !is_verify {
      log::warn!("Password is incorrect");
      return Err(Status::Unauthorized);
    }

    if !user.verified {
      log::warn!("The user's email has not been verified");
      return Err(Status::BadRequest);
    }

    // if  self.user_repository.is_in_blacklist(pool.inner()).await? {
    //   log::warn!("User '{}' is currently in the blacklist.", user.user_id);
    //   return Err(Status::Forbidden);
    // }

    Ok(user)
  }

  pub async fn get_user_by_id(&self, user_id: i64) -> Result<User, Status> {
    self.user_repository.query_user_by_id(user_id).await
  }

  pub async fn verify_email(&self, verification_token: &str) -> Result<(), Status> {
    self
      .user_repository
      .update_verified(&verification_token)
      .await
  }

  // fn get_user_list() {}

  pub async fn add_violation_points(&self, user_id: i64, points: i64) -> Result<(), Status> {
    Ok(())
  }

  pub fn create_user_token(&self, user_id: i64, user_role: UserRole) -> Result<String, Status> {
    let duration: Duration = match user_role {
      UserRole::Student => Duration::hours(1), // 1 小時後過期
      UserRole::RegularUser => Duration::hours(1),
      UserRole::Admin => Duration::hours(24), // 1 天後過期
      UserRole::Assistant => Duration::hours(24),
    };

    let exp = Utc::now()
      .checked_add_signed(duration)
      .expect("valid timestamp")
      .timestamp() as usize;

    let claim = UserClaim {
      user_id: user_id,
      user_role: user_role,
      exp: exp,
    };

    let header = Header::new(Algorithm::HS256);
    let key = env::var("SECRET_KEY").expect("Failed to get secret key");

    let encoding_key = EncodingKey::from_secret(key.as_ref());

    let token = handle(encode(&header, &claim, &encoding_key), "Encoding JWT")?;

    Ok(token)
  }

  pub fn create_verification_token(&self, user_id: i64) -> Result<String, Status> {
    let exp = Utc::now()
      .checked_add_signed(Duration::hours(1)) // 1 小時後過期
      .expect("valid timestamp")
      .timestamp() as usize; // or u64

    let expiration = Utc::now()
      .checked_add_signed(Duration::minutes(1))
      .expect("valid timestamp")
      .timestamp() as i64;

    let claim = VerificationClaim {
      user_id: user_id,
      expiration: expiration,
      exp: exp,
    };

    let header = Header::new(Algorithm::HS256);
    let key = env::var("SECRET_KEY").expect("Failed to get secret key");

    let encoding_key = EncodingKey::from_secret(key.as_ref());

    let token = handle(encode(&header, &claim, &encoding_key), "Encoding JWT")?;

    Ok(token)
  }

  pub fn is_resend_allowed(&self, claim: &VerificationClaim) -> bool {
    let expiration = claim.expiration;
    let now: i64 = get_now().timestamp();
    if now <= expiration {
      return false;
    }
    true
  }
}
