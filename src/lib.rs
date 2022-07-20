use axum::extract::Json;
use axum::http::StatusCode;
use sea_orm::prelude::*;

pub mod user;
pub mod session;

pub type ResErr = (StatusCode, &'static str);
pub type Res<T> = core::result::Result<Json<T>, ResErr>;

pub fn db_err(_: DbErr) -> ResErr {
    (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
}
