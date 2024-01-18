use super::common::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Seat {
  pub seat_id: u16,
  pub available: bool,
  pub other_info: Option<String>,
}

impl FromRow<'_, SqliteRow> for Seat {
  fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
    /*
    .try_into().map_err(|_| Error::RowNotFound)?;
     */
    Ok(Seat {
      seat_id: row.try_get("seat_id")?,
      available: row.try_get("available")?,
      other_info: row.try_get("other_info")?,
    })
  }
}

// 座位狀態
#[derive(Serialize)]
pub struct SeatAvailability {
  pub seat_id: u16,
  pub status: SeatAvailabilityStatus,
}

// 全部座位狀態
#[derive(Serialize)]
pub struct SeatsOverview {
  pub seats: Vec<SeatAvailability>,
}

#[derive(Serialize)]
pub enum SeatAvailabilityStatus {
  Available,
  Unavailable,
  Borrowed,
}

impl ToString for SeatAvailabilityStatus {
  fn to_string(&self) -> String {
    match *self {
      SeatAvailabilityStatus::Available => "Available".to_owned(),
      SeatAvailabilityStatus::Unavailable => "Unavailable".to_owned(),
      SeatAvailabilityStatus::Borrowed => "Borrowed".to_owned(),
    }
  }
}

impl FromStr for SeatAvailabilityStatus {
  type Err = std::io::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Available" => Ok(SeatAvailabilityStatus::Available),
      "Unavailable" => Ok(SeatAvailabilityStatus::Unavailable),
      "Borrowed" => Ok(SeatAvailabilityStatus::Borrowed),
      _ => Err(std::io::Error::new(
        ErrorKind::InvalidInput,
        "Provided string does not match any Status variant",
      )),
    }
  }
}
