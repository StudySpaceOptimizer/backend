pub use crate::utils::*;
pub use async_trait::async_trait;
pub use sqlx::{query, query_as, query_scalar, Error, Pool, Sqlite};

pub fn handle_sqlx<T>(result: Result<T, Error>, prefix: &str) -> Result<T, Status> {
  result.map_err(|err| {
    log::error!("{} failed with error: {:?}", prefix, err);

    match &err {
      Error::RowNotFound => Status::NotFound,
      Error::ColumnNotFound(_) => Status::BadRequest,
      Error::ColumnIndexOutOfBounds { .. } => Status::BadRequest,
      _ => Status::InternalServerError,
    }
  })
}
