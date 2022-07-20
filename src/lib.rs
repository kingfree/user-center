use axum::extract::Json;
use axum::http::StatusCode;
use sea_orm::prelude::*;

pub mod group;
pub mod session;
pub mod user;

pub type ResErr = (StatusCode, &'static str);
pub type Res<T> = core::result::Result<Json<T>, ResErr>;

pub fn db_err(err: DbErr) -> ResErr {
    log::error!("db_err: {:?}", err);
    (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
}
