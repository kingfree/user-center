use crate::{db_err, Res};
use axum::extract::Json;
use axum::{
    extract::Extension,
    http::{self, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use entity::{prelude::*, user};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use jwt::VerifyWithKey;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::BTreeMap;
use util::verify_password;

#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub id: i32,
    pub name: String,
    pub level: i32,
}

pub async fn auth<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .unwrap_or("");

    let split = auth_header.split_once(' ');
    let auth_header = match split {
        Some((name, contents)) if name == "Bearer" => contents,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    if let Ok(current_user) = authorize_current_user(auth_header).await {
        req.extensions_mut().insert(current_user);
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn authorize_current_user(auth_token: &str) -> Result<CurrentUser, ()> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").map_err(|_| ())?;
    let claims: BTreeMap<String, String> = auth_token.verify_with_key(&key).map_err(|_| ())?;
    log::info!("{:?}", claims);
    Ok(CurrentUser {
        id: claims["id"].parse().map_err(|_| ())?,
        name: claims["name"].parse().map_err(|_| ())?,
        level: claims["level"].parse().map_err(|_| ())?,
    })
}

#[derive(Deserialize, Serialize)]
pub struct LoginForm {
    name: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserInfo {
    id: i32,
    name: String,
    token: String,
}

pub async fn login(
    Extension(ref conn): Extension<DatabaseConnection>,
    Json(payload): Json<LoginForm>,
) -> Res<UserInfo> {
    let model = User::find()
        .find_also_related(Group)
        .filter(user::Column::Name.eq(payload.name))
        .one(conn)
        .await
        .map_err(db_err)?;
    match model {
        Some((u, Some(g))) => {
            if !verify_password(&u.password, &payload.password) {
                return Err((StatusCode::UNAUTHORIZED, "Password is incorrect"));
            }

            let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
            let mut claims = BTreeMap::new();
            claims.insert("id", u.id.to_string());
            claims.insert("name", u.name.to_string());
            claims.insert("level", g.level.to_string());
            let token = claims
                .sign_with_key(&key)
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Sign error"))?;

            Ok(Json(UserInfo {
                id: u.id,
                name: u.name,
                token,
            }))
        }
        _ => Err((StatusCode::UNAUTHORIZED, "User not found")),
    }
}
