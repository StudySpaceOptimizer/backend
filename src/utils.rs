use chrono::{FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use lettre::{
  message::Mailbox, transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport,
};
use std::{
  env,
  io::{Error as IoError, ErrorKind},
};
use validator::ValidationErrorsKind;

pub use rocket::http::Status;

pub fn handle<T, E>(result: Result<T, E>, prefix: &str) -> Result<T, Status>
where
  E: std::error::Error + 'static,
{
  result.map_err(|err| {
    log::error!("{} failed with error: {:?}", prefix, err);

    let dyn_error: &dyn std::error::Error = &err;

    if let Some(e) = dyn_error.downcast_ref::<IoError>() {
      match e.kind() {
        ErrorKind::NotFound => Status::NotFound,
        ErrorKind::PermissionDenied => Status::Forbidden,
        ErrorKind::ConnectionRefused => Status::ServiceUnavailable,
        _ => Status::InternalServerError,
      }
    } else {
      Status::InternalServerError
    }
  })
}

pub fn handle_validator(result: Result<(), validator::ValidationErrors>) -> Result<(), Status> {
  result.map_err(|validation_errors| {
    let error_message: Vec<String> = validation_errors
      .errors()
      .into_iter()
      .map(|(field, validation_error_kind)| {
        let msg: Vec<String> =
          if let ValidationErrorsKind::Field(validation_error) = validation_error_kind {
            validation_error
              .into_iter()
              .map(|error| format!("Invalid {} Error: {}", field, error.code))
              .collect()
          } else {
            Vec::new()
          };
        return msg.join(", ");
      })
      .collect();

    log::error!("{}", error_message.join(", "));

    Status::UnprocessableEntity
  })
}

pub fn get_today() -> NaiveDate {
  let taipei_offset = FixedOffset::east_opt(8 * 3600).expect("Invalid offset");
  Utc::now().with_timezone(&taipei_offset).date_naive()
}

pub fn get_now() -> NaiveDateTime {
  let taipei_offset = FixedOffset::east_opt(8 * 3600).expect("Invalid offset");
  Utc::now().with_timezone(&taipei_offset).naive_utc()
}

// pub fn time_to_string(timestamp: i64) -> Result<String, Status> {
//   let naive_datetime = NaiveDateTime::from_timestamp_opt(timestamp, 0).ok_or_else(|| {
//     log::error!("Invalid timestamp");
//     Status::InternalServerError
//   })?;

//   Ok(naive_datetime.format("%Y-%m-%d %H:%M:%S").to_string())
// }

pub fn naive_date_to_timestamp(
  date: NaiveDate,
  hour: u32,
  min: u32,
  sec: u32,
) -> Result<i64, Status> {
  let time = NaiveTime::from_hms_opt(hour, min, sec).ok_or_else(|| {
    log::error!("Invalid NaiveTime");
    Status::InternalServerError
  })?;

  // 本地日期時間 GMT+8
  let datetime_local = NaiveDateTime::new(date, time);
  // GNT 0 日期時間
  let datetime = datetime_local - chrono::Duration::hours(8);

  let timestamp = datetime.timestamp();

  Ok(timestamp)
}

/// -> get now timestamp
pub fn naive_datetime_to_timestamp(datetime_local: NaiveDateTime) -> Result<i64, Status> {
  let datetime = datetime_local - chrono::Duration::hours(8);
  let timestamp = datetime.timestamp();

  Ok(timestamp)
}

pub fn timestamp_to_naive_datetime(timestamp: i64) -> Result<NaiveDateTime, Status> {
  // GMT+8
  let offset = FixedOffset::east_opt(8 * 3600).ok_or_else(|| {
    log::error!("Invalid offset");
    Status::InternalServerError
  })?;

  // GMT 0 日期時間
  let datetime = Utc.timestamp_opt(timestamp, 0).single().ok_or_else(|| {
    log::error!("Invalid timestamp");
    Status::InternalServerError
  })?;

  // 本地日期時間
  let datetime_local = datetime.with_timezone(&offset).naive_local();

  Ok(datetime_local)
}

pub fn get_root() -> String {
  env::var("ROOT").expect("Failed to get root path")
}

pub fn get_base_url() -> String {
  env::var("BASE_URL").expect("Failed to get base url")
}

pub fn send_verification_email(user_email: &str, verification_token: &str) -> Result<(), Status> {
  let email_address_str = env::var("EMAIL_ADDRESS").expect("Failed to get email address");
  let email_password = env::var("EMAIL_PASSWORD").expect("Failed to get email password");
  let email_domain = env::var("EMAIL_DOMAIN").expect("Failed to get email domain");

  let email_address = handle(
    email_address_str.parse::<Mailbox>(),
    "Parsing email address",
  )?;

  let user_email = handle(user_email.parse::<Mailbox>(), "Parsing user email")?;
  let url = format!(
    "{}/api/verify?verification_token={}",
    get_base_url(),
    verification_token
  );

  let email = handle(
    Message::builder()
      .to(user_email)
      .from(email_address)
      .subject("Verify your email")
      .body(format!(
        "Please click on the link to verify your email: {}",
        url
      )),
    "Building email",
  )?;

  let creds = Credentials::new(email_address_str, email_password);

  let mailer = handle(
    SmtpTransport::relay(&format!("smtp.{}.com", email_domain)),
    "Setting SMTP server address",
  )?
  .credentials(creds)
  .build();

  handle(mailer.send(&email), "Sending email")?;

  Ok(())
}
