use super::common::*;
use crate::model::{token, user};

// 註冊
#[post("/api/users/register", format = "json", data = "<request>")]
pub async fn register_user(
  app: &State<App>,
  request: Json<RegisterRequest>,
) -> Result<String, Status> {
  handle_validator(request.validate())?;

  let email = &request.email;
  let password = &request.password;
  let user_role = request.user_role;

  log::info!("Registering new user with email: {}", email);
  let user = app
    .user_service
    .create_user(email, password, user_role)
    .await?;

  send_verification_email(&user.email, &user.verification_token)?;

  let token = app.user_service.create_verification_token(user.user_id)?;

  log::info!("User registration successful: {}", user.user_id);
  Ok(token)
}

#[get("/api/users/resend_verification")]
pub async fn resend_verification_email(
  app: &State<App>,
  claim: token::VerificationClaim,
) -> Result<String, Status> {
  log::info!(
    "Processing resend verification email request for user: {}",
    claim.user_id
  );

  if !app.user_service.is_resend_allowed(&claim) {
    log::warn!(
      "Resend verification email not allowed yet for user: {}",
      claim.user_id
    );
    return Err(Status::BadRequest);
  }

  let user = app.user_service.get_user_by_id(claim.user_id).await?;

  send_verification_email(&user.email, &user.verification_token)?;

  let token = app.user_service.create_verification_token(claim.user_id)?;

  log::info!("Verification email resent to user: {}", claim.user_id);
  Ok(token)
}

#[get("/api/users/verify?<verification_token>")]
pub async fn verify_email(app: &State<App>, verification_token: String) -> Result<String, Status> {
  log::info!("Verifying email with token: {}", verification_token);

  app.user_service.verify_email(&verification_token).await?;

  log::info!(
    "Email verified successfully for token: {}",
    verification_token
  );
  Ok("Your email has been successfully verified.".to_string())
}

// 登入
#[post("/api/users/login", data = "<request>")]
pub async fn login_user(app: &State<App>, request: Json<LoginRequest>) -> Result<String, Status> {
  handle_validator(request.validate())?;

  let email = &request.email;
  let password = &request.password;

  log::info!("Processing login request for user: {}", email);
  let user = app.user_service.login(email, password).await?;

  let token = app
    .user_service
    .create_user_token(user.user_id, user.user_role)?;

  log::info!("User login successful: {}", email);
  Ok(token)
}

// 註冊
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterRequest {
  #[validate(email)]
  pub email: String,
  #[validate(length(min = 8, max = 20))]
  pub password: String,
  pub user_role: user::UserRole,
}

// 登入
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginRequest {
  #[validate(email)]
  pub email: String,
  #[validate(length(min = 8, max = 20))]
  pub password: String,
}
