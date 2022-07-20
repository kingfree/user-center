use crate::{db_err, session::CurrentUser, Res};
use axum::extract::{Extension, Json, Path, Query};
use axum::http::StatusCode;
use entity::{group, prelude::*, user};
use sea_orm::{entity::*, prelude::*, query::*, FromQueryResult, QueryOrder, Set};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Params {
    sort: Option<String>,
    asc: Option<bool>,
}

#[derive(FromQueryResult, Deserialize, Serialize)]
pub struct UserDetail {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub group_id: i32,
    pub group_name: String,
    pub level: i32,
}

impl UserDetail {
    pub fn new(u: user::Model, g: group::Model) -> Self {
        Self {
            id: u.id,
            name: u.name,
            description: u.description,
            group_id: g.id,
            group_name: g.name,
            level: g.level,
        }
    }
}

pub async fn list(
    Extension(ref conn): Extension<DatabaseConnection>,
    Extension(current_user): Extension<CurrentUser>,
    Query(query): Query<Params>,
) -> Res<Vec<UserDetail>> {
    log::warn!("{:?}", current_user);

    let order = if query.asc.unwrap_or(true) {
        Order::Asc
    } else {
        Order::Desc
    };
    let sort = if let Some(sort) = query.sort {
        match sort.to_ascii_lowercase().trim() {
            "id" => user::Column::Id,
            "name" => user::Column::Name,
            "description" => user::Column::Description,
            "group_id" => user::Column::GroupId,
            _ => user::Column::Id,
        }
    } else {
        user::Column::Id
    };
    log::info!("sort={sort:?} order={order:?}");

    let sql = User::find()
        .find_also_related(Group)
        .filter(group::Column::Level.lte(current_user.level))
        .order_by(sort, order);
    let res = sql.all(conn).await.map_err(db_err)?;
    let res = res
        .into_iter()
        .map(|item| match item {
            (u, Some(g)) => UserDetail::new(u, g),
            _ => unreachable!(),
        })
        .collect();
    Ok(Json(res))
}

pub async fn create(
    Extension(ref conn): Extension<DatabaseConnection>,
    Json(payload): Json<serde_json::Value>,
) -> Res<()> {
    let mut model = user::ActiveModel::from_json(payload).map_err(db_err)?;
    if let Some(password) = model.password.take() {
        model.password = if password.is_empty() {
            return Err((StatusCode::BAD_REQUEST, "Password is empty"));
        } else {
            Set(util::encode_password(&password))
        };
    }
    model.insert(conn).await.map_err(db_err)?;
    Ok(Json(()))
}

pub async fn detail(
    Extension(ref conn): Extension<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Res<UserDetail> {
    let model = User::find_by_id(id)
        .find_also_related(Group)
        .one(conn)
        .await
        .map_err(db_err)?;
    match model {
        Some((u, Some(g))) => Ok(Json(UserDetail::new(u, g))),
        _ => Err((StatusCode::NOT_FOUND, "User not found")),
    }
}

pub async fn update(
    Extension(ref conn): Extension<DatabaseConnection>,
    Path(id): Path<i32>,
    Json(payload): Json<serde_json::Value>,
) -> Res<()> {
    let mut model = user::ActiveModel::from_json(payload).map_err(db_err)?;
    model.id = Set(id);
    if let Some(password) = model.password.take() {
        log::info!("password=[{}], len={}", password, password.len());
        model.password = if password.is_empty() {
            ActiveValue::NotSet
        } else {
            Set(util::encode_password(&password))
        };
    }
    model.save(conn).await.map_err(db_err)?;
    Ok(Json(()))
}

pub async fn delete(
    Extension(ref conn): Extension<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Res<()> {
    let model = User::find_by_id(id).one(conn).await.map_err(db_err)?;
    match model {
        Some(model) => {
            model.delete(conn).await.map_err(db_err)?;
            Ok(Json(()))
        }
        None => Err((StatusCode::NOT_FOUND, "User not found")),
    }
}
