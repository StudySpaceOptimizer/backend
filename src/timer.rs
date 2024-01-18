use crate::{utils::*, App};
use chrono::Duration;
use tokio::time::sleep;

pub async fn start(app: App) {
  log::info!("Starting timmer!");
  loop {
    let today = get_today().and_hms_opt(0, 0, 0).unwrap();
    let tomorrow_midnight = today + Duration::days(1);
    let now = get_now();

    let duration = tomorrow_midnight - now;
    let std_duration = duration.to_std().unwrap();
    // let std_duration = std::time::Duration::from_secs(3);

    sleep(std_duration).await;
    log::info!("Deleting logfile");
    app.init_service.delete_logfile();

    log::info!("Initing unavailable timeslot");
    app.init_service.init_unavailable_timeslot(3).await;
  }
}
