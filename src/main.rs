mod apis;
mod logger;
mod model;
mod repository;
mod service;
mod timer;
mod utils;

use apis::{auth::*, reservation::*, seat_status::*, tsmc::*};
use repository::sqlite_repo::*;
use service::*;

use dotenv::dotenv;
use rocket::{
  self, catch, catchers,
  fairing::{Fairing, Info, Kind},
  http::Header,
  options, routes, {Request, Response},
};
use sqlx::{Pool, Sqlite};
use std::env;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
  fn info(&self) -> Info {
    Info {
      name: "Add CORS headers to responses",
      kind: Kind::Response,
    }
  }

  async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
    response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
    response.set_header(Header::new(
      "Access-Control-Allow-Methods",
      "POST, GET, PATCH, OPTIONS, DELETE, PUT",
    ));
    response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
    response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
  }
}

#[options("/<_..>")]
async fn options_route() -> &'static str {
  ""
}

#[catch(422)]
fn handle_unprocessable_entity(_: &Request) -> &'static str {
  // 請求格式錯誤
  "The request contains invalid parameters"
}
#[catch(403)]
fn handle_forbidden(_: &Request) -> &'static str {
  "You don't have permission to access this resource"
}
#[catch(401)]
fn unauthorized(_: &Request) -> &'static str {
  "Unauthorized access"
}
#[catch(404)]
fn handle_not_found(_: &Request) -> &'static str {
  "The resource was not found"
}
#[catch(500)]
fn handle_internal_server_error(_: &Request) -> &'static str {
  "Something went wrong"
}
#[catch(503)]
fn handle_service_unavailable(_: &Request) -> &'static str {
  // 伺服器當前無法處理請求
  "The server is currently unable to handle the request"
}

#[derive(Clone)]
pub struct App {
  /*
  一個 trait 要被用作 trait 物件（例如 dyn SomeTrait），必須滿足特定的物件安全條件。
  解決方案:
  使用具體類型而非 Trait 物件（如 SqliteReservationRepository）


  Service 和 Repository 都不涉及任何內部可變狀態(最終都是操作資料庫)，因此不使用 Arc 來包裹這些組件。

   */
  user_service: user_service::UserService<user_sqlite_repo::SqliteUserRepository>,

  seat_service: seat_service::SeatService<
    seat_sqlite_repo::SqliteSeatRepository,
    timeslot_sqlite_repo::SqliteTimeSlotRepository,
  >,

  reservation_service: reservation_service::ReservationService<
    reservation_sqlite_repo::SqliteReservationRepository,
    timeslot_sqlite_repo::SqliteTimeSlotRepository,
  >,

  timeslot_service:
    timeslot_service::TimeSlotService<timeslot_sqlite_repo::SqliteTimeSlotRepository>,

  blacklist_service:
    blacklist_service::BlacklistService<blacklist_sqlite_repo::SqliteBlacklistRepository>,

  init_service: init_service::InitService<
    seat_sqlite_repo::SqliteSeatRepository,
    timeslot_sqlite_repo::SqliteTimeSlotRepository,
  >,
}

unsafe impl Sync for App {}
unsafe impl Send for App {}

impl App {
  pub async fn new(pool: Pool<Sqlite>) -> Self {
    // repository

    // user
    let user_repository = user_sqlite_repo::SqliteUserRepository::new(pool.clone());

    // seat
    let seat_repository = seat_sqlite_repo::SqliteSeatRepository::new(pool.clone());

    // timeslot
    let timeslot_repository = timeslot_sqlite_repo::SqliteTimeSlotRepository::new(pool.clone());

    // reservation
    let reservation_repository =
      reservation_sqlite_repo::SqliteReservationRepository::new(pool.clone());

    // blacklist
    let blacklist_repository = blacklist_sqlite_repo::SqliteBlacklistRepository::new(pool);

    // Service

    // user
    let user_service = user_service::UserService::new(user_repository);

    // seat
    let seat_service =
      seat_service::SeatService::new(seat_repository.clone(), timeslot_repository.clone());

    // reservation
    let reservation_service = reservation_service::ReservationService::new(
      reservation_repository,
      timeslot_repository.clone(),
    );

    // timeslot
    let timeslot_service = timeslot_service::TimeSlotService::new(timeslot_repository.clone());

    // blacklist
    let blacklist_service = blacklist_service::BlacklistService::new(blacklist_repository);

    // init
    let init_service = init_service::InitService::new(seat_repository, timeslot_repository);

    App {
      user_service: user_service,
      seat_service: seat_service,
      reservation_service: reservation_service,
      timeslot_service: timeslot_service,
      blacklist_service: blacklist_service,
      init_service: init_service,
    }
  }
}

#[tokio::main]
async fn main() {
  dotenv().ok();
  logger::init_logger(log::LevelFilter::Info);

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

  let pool = sqlx::pool::PoolOptions::new()
    .max_lifetime(None)
    .idle_timeout(None)
    .connect(&database_url)
    .await
    .expect("Failed to create pool");

  let app = App::new(pool.clone()).await;
  app.init_service.init_db().await;

  let app_clone = app.clone();
  tokio::spawn(async move {
    timer::start(app_clone).await;
  });

  let catchers = catchers![
    handle_unprocessable_entity,
    handle_forbidden,
    handle_not_found,
    handle_internal_server_error,
    handle_service_unavailable,
    unauthorized
  ];

  let routes = routes![
    options_route,
    register_user,
    login_user,
    reserve_seat,
    delete_reservation,
    display_user_reservations,
    show_current_seats_status,
    show_seats_status_in_specific_timeslots,
    get_status_code,
    disconnect_db,
    timeout,
    big_memory,
    big_cpu,
    concurrent_error
  ];

  let server = rocket::build()
    .register("/", catchers)
    .mount("/", routes)
    .attach(CORS)
    .manage(app)
    .manage(pool)
    .launch();

  tokio::select! {
      _ = server => {},
      _ = tokio::signal::ctrl_c() => {},
  }
}
