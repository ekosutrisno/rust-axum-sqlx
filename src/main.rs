use axum::{http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
}, Router, middleware};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;

use crate::{health::{health_router, main_response_mapper}, router::{todo_router, new_todo_router}};

mod handler;
mod router;
mod model;
mod schema;
mod health;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = Router::new()
        .merge(health_router())
        .merge(todo_router(&pool))
        .nest("/v1", new_todo_router(&pool))
        .layer(middleware::map_response(main_response_mapper))
        .layer(cors);

    println!("🚀 Server started successfully on port {}", 8081);
    axum::Server::bind(&"0.0.0.0:8081".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}
