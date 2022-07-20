use axum::{
    extract::Extension,
    middleware,
    routing::{get, post},
    Router, Server,
};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use std::str::FromStr;
use std::{env, net::SocketAddr};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use user_center::{group, session, user};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{}:{}", host, port);

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    let cors = CorsLayer::very_permissive().allow_credentials(true);

    let users = Router::new()
        .route("/", get(user::list).post(user::create))
        .route(
            "/:id",
            get(user::detail).put(user::update).delete(user::delete),
        )
        .route_layer(middleware::from_fn(session::auth));
    let groups = Router::new()
        .route("/", get(group::list))
        .route_layer(middleware::from_fn(session::auth));
    let app = Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/login", post(session::login))
                .nest("/user", users)
                .nest("/group", groups),
        )
        .layer(ServiceBuilder::new().layer(Extension(conn)))
        .layer(cors);

    let addr = SocketAddr::from_str(&server_url).unwrap();
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}
