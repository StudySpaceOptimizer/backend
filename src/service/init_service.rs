use super::common::*;
use crate::{
  model::{constant::*, timeslot::*},
  repository::{seat_repo::*, timeslot_repo::*},
};
use chrono::{Datelike, Duration, NaiveDate};
use std::fs;

#[derive(Clone)]
pub struct InitService<S: SeatRepository, T: TimeSlotRepository> {
  seat_repository: S,
  timeslot_repository: T,
}

impl<S: SeatRepository, T: TimeSlotRepository> InitService<S, T> {
  pub fn new(seat_repository: S, timeslot_repository: T) -> Self {
    InitService {
      seat_repository,
      timeslot_repository,
    }
  }

  pub async fn init_db(&self) {
    self.init_seats().await;
    for date in 0..=3 {
      self.init_unavailable_timeslot(date).await;
    }
    self.delete_logfile();
  }

  async fn init_seats(&self) {
    for seat_id in 1..=NUMBER_OF_SEATS {
      self
        .seat_repository
        .insert_seat(seat_id, true, None)
        .await
        .unwrap_or_else(|e| {
          panic!("Failed to initialize Seats table: {}", e);
        });
    }
  }

  pub async fn init_unavailable_timeslot(&self, date: i64) {
    let today = get_today();
    let future_date = today + chrono::Duration::days(date);
    let weekday = future_date.weekday();
    let is_holiday = weekday == chrono::Weekday::Sat || weekday == chrono::Weekday::Sun;

    let mut time_slots: Vec<TimeSlot> = Vec::new();

    if is_holiday {
      let start_time: i64 =
        naive_date_to_timestamp(future_date, 0, 0, 0).expect("Invalid timestamp");
      let end_time: i64 = naive_date_to_timestamp(future_date, 9, 0, 0).expect("Invalid timestamp");

      time_slots.push(TimeSlot {
        start_time,
        end_time,
      });

      let start_time: i64 =
        naive_date_to_timestamp(future_date, 17, 0, 0).expect("Invalid timestamp");
      let end_time: i64 =
        naive_date_to_timestamp(future_date, 23, 59, 59).expect("Invalid timestamp");

      time_slots.push(TimeSlot {
        start_time,
        end_time,
      });
    } else {
      let start_time: i64 =
        naive_date_to_timestamp(future_date, 0, 0, 0).expect("Invalid timestamp");
      let end_time: i64 = naive_date_to_timestamp(future_date, 8, 0, 0).expect("Invalid timestamp");

      time_slots.push(TimeSlot {
        start_time,
        end_time,
      });

      let start_time: i64 =
        naive_date_to_timestamp(future_date, 22, 0, 0).expect("Invalid timestamp");
      let end_time: i64 =
        naive_date_to_timestamp(future_date, 23, 59, 59).expect("Invalid timestamp");

      time_slots.push(TimeSlot {
        start_time,
        end_time,
      });
    }

    for timeslot in time_slots.into_iter() {
      let overlapping = self
        .timeslot_repository
        .is_overlapping_with_unavailable_timeslot(timeslot.clone())
        .await
        .unwrap_or_else(|e| {
          panic!(
            "Failed to check overlapping with unavailable timeslot: {}",
            e
          );
        });

      if !overlapping {
        self
          .timeslot_repository
          .insert_timeslot(timeslot)
          .await
          .unwrap_or_else(|e| {
            panic!("Failed to insert unavailable timeslots: {}", e);
          });
      }
    }
  }

  pub fn delete_logfile(&self) {
    let root = get_root();
    let path = root + "/logfiles";
    let last_week = get_today() - Duration::days(7);

    let entries = fs::read_dir(&path).unwrap_or_else(|e| {
      log::error!("Reading directory '{}' failed with err: {:?}", path, e);
      panic!("Reading directory '{}' failed with err: {:?}", path, e);
    });

    for entry in entries {
      match entry {
        Ok(entry) => {
          let file_name = entry.file_name().to_string_lossy().to_string();

          if let Some(date_str) = file_name.split('.').next() {
            if let Ok(date) = date_from_string(date_str) {
              // 刪除7天前的logfile
              if date <= last_week {
                fs::remove_file(entry.path()).unwrap_or_else(|e| {
                  log::warn!(
                    "Removing file '{:?}' failed with err: {:?}",
                    entry.path(),
                    e
                  );
                });
              }
            }
          }
        }
        Err(e) => {
          log::warn!("Failed to read directory entry: {:?}", e);
        }
      }
    }
  }
}

fn date_from_string(date: &str) -> Result<NaiveDate, Status> {
  handle(
    NaiveDate::parse_from_str(date, "%Y-%m-%d"),
    &format!("Parsing date from str '{}'", date),
  )
}
