use crate::{db_err, session::CurrentUser, Res};
use axum::extract::{Extension, Json, Query};
use entity::{prelude::*, user};
use sea_orm::{prelude::*, QueryOrder};
use sea_orm::{FromQueryResult, JoinType, RelationTrait};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Params {
    pi: Option<usize>,
    ps: Option<usize>,
}

#[derive(FromQueryResult)]
pub struct UserDetail {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub group_id: i32,
    pub group_name: String,
    pub level: i32,
}

pub async fn list(
    Extension(ref conn): Extension<DatabaseConnection>,
    Extension(current_user): Extension<CurrentUser>,
) -> Res<Vec<UserDetail>> {
    log::warn!("{:?}", current_user);
    let res = User::find()
        .find_also_related(Group)
        .order_by_asc(user::Column::Id)
        .all(conn)
        .await
        .map_err(db_err)?;
    let res = res
        .into_iter()
        .map(|item| match item {
            (u, Some(g)) => UserDetail {
                id: u.id,
                name: u.name,
                description: u.description.unwrap_or_else(|| "".to_string()),
                group_id: g.id,
                group_name: g.name,
                level: g.level,
            },
            _ => unreachable!(),
        })
        .collect();
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
