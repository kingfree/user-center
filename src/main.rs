use axum::{
    extract::{Extension, Query},
    response::Json,
    routing::get,
    Router, Server,
};
use entity::{prelude::*, user};
use migration::{Migrator, MigratorTrait};
use sea_orm::{prelude::*, Database, QueryOrder};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::{env, net::SocketAddr};
use tower::ServiceBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{}:{}", host, port);

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    let app = Router::new()
        .route("/", get(list_users))
        .layer(ServiceBuilder::new().layer(Extension(conn)));

    let addr = SocketAddr::from_str(&server_url).unwrap();
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

#[derive(Deserialize, Serialize)]
pub struct Params {
    pi: Option<usize>,
    ps: Option<usize>,
}

pub async fn list_users(
    Extension(ref conn): Extension<DatabaseConnection>,
    Query(params): Query<Params>,
) -> Result<Json<Vec<user::Model>>, ()> {
    let pi = params.pi.unwrap_or(1);
    let ps = params.ps.unwrap_or(10);

    let sql = User::find().order_by_desc(user::Column::Id);
    let pager = sql.paginate(conn, ps);
    let res: Vec<_> = pager.fetch_page(pi - 1).await.unwrap();
    Ok(Json(res))
}
