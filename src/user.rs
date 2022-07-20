use crate::{db_err, Res};
use axum::extract::{Extension, Json, Query};
use entity::{prelude::*, user};
use sea_orm::{prelude::*, QueryOrder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Params {
    pi: Option<usize>,
    ps: Option<usize>,
}

pub async fn list(
    Extension(ref conn): Extension<DatabaseConnection>,
    Query(params): Query<Params>,
) -> Res<Vec<user::Model>> {
    let pi = params.pi.unwrap_or(1);
    let ps = params.ps.unwrap_or(10);

    let sql = User::find().order_by_asc(user::Column::Id);
    let pager = sql.paginate(conn, ps);
    let res: Vec<_> = pager.fetch_page(pi - 1).await.map_err(db_err)?;
    Ok(Json(res))
}

pub async fn create(
    Extension(ref conn): Extension<DatabaseConnection>,
    Json(payload): Json<serde_json::Value>,
) -> Res<()> {
    let model = user::ActiveModel::from_json(payload).map_err(db_err)?;
    model.save(conn).await.map_err(db_err)?;
    Ok(Json(()))
}
