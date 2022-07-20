use crate::{db_err, session::CurrentUser, Res};
use axum::extract::{Extension, Json};
use entity::{group, prelude::*};
use sea_orm::{prelude::*, QueryOrder};
pub async fn list(
    Extension(ref conn): Extension<DatabaseConnection>,
    Extension(current_user): Extension<CurrentUser>,
) -> Res<Vec<group::Model>> {
    log::warn!("{:?}", current_user);
    let res = Group::find()
        .order_by_desc(group::Column::Level)
        .all(conn)
        .await
        .map_err(db_err)?;
    Ok(Json(res))
}
