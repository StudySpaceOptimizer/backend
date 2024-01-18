use super::{common::*, user};

use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use rocket::request::{FromRequest, Outcome, Request};

trait Claim: Sized {
  fn verify_jwt(token: &str) -> Result<Self, Status>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaim {
  pub user_id: i64,
  pub user_role: user::UserRole,
  pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationClaim {
  pub user_id: i64,
  pub expiration: i64,
  pub exp: usize,
}

impl Claim for UserClaim {
  fn verify_jwt(token: &str) -> Result<UserClaim, Status> {
    let key = env::var("SECRET_KEY").expect("Failed to get secret key");

    let token = handle(
      decode::<UserClaim>(
        token,
        &DecodingKey::from_secret(key.as_ref()),
        &Validation::new(Algorithm::HS256),
      ),
      "Decoding JWT",
    )?;

    Ok(token.claims)
  }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserClaim {
  type Error = ();

  async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    let headers = request.headers().get_one("Authorization");
    match headers {
      Some(header) => {
        let token = header.replace("Bearer ", "");

        match Self::verify_jwt(&token) {
          Ok(claims) => Outcome::Success(claims),
          Err(_) => Outcome::Failure((Status::Unauthorized, ())),
        }
      }
      None => Outcome::Failure((Status::BadRequest, ())),
    }
  }
}

impl Claim for VerificationClaim {
  fn verify_jwt(token: &str) -> Result<VerificationClaim, Status> {
    let key = env::var("SECRET_KEY").expect("Failed to get secret key");

    let token = handle(
      decode::<VerificationClaim>(
        token,
        &DecodingKey::from_secret(key.as_ref()),
        &Validation::new(Algorithm::HS256),
      ),
      "Decoding JWT",
    )?;

    Ok(token.claims)
  }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for VerificationClaim {
  type Error = ();

  async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    let headers = request.headers().get_one("Authorization");
    match headers {
      Some(header) => {
        let token = header.replace("Bearer ", "");

        match Self::verify_jwt(&token) {
          Ok(claims) => Outcome::Success(claims),
          Err(_) => Outcome::Failure((Status::Unauthorized, ())),
        }
      }
      None => Outcome::Failure((Status::BadRequest, ())),
    }
  }
}
