// #![allow(unused_imports)]

use anyhow::anyhow;
use axum::{
    http::{HeaderValue, Method},
    routing::{get, post},
    Router,
};
use shuttle_secrets::SecretStore;
use std::{env, net::SocketAddr};
use tower_http::cors::CorsLayer;

mod web;

use web::registration;

#[shuttle_runtime::main]
async fn axum(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    // dotenv().ok();

    // let db_url = env::var("DATABASE_URL").expect("DATABASE_URL env not found.");

    let db_url = if let Some(db_url) = secret_store.get("DATABASE_URL") {
        db_url
    } else {
        return Err(anyhow!("DATABASE_URL not found.").into());
    };

    let pool = sqlx::postgres::PgPool::connect(&db_url)
        .await
        .expect("Can't connect to database.");

    println!("\nNow listening to Postgres...");

    let app = Router::new()
        .route("/", get(hello_world))
        .route("/register", post(registration::register))
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST]),
        )
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    println!("Server has started, listening on: {}", addr);

    Ok(app.into())
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}
