use std::{net::SocketAddr, env};

use axum::{Router, Server, routing::get};
use dotenv::dotenv;
use routes::routes_ticket;
use schemas::AppState;
use utils::create_postgres_pool;

const PORT: u16 = 3000;

mod routes;
mod utils;
mod handlers;
mod schemas;

#[tokio::main]
async fn main() -> Result<(), ()> {
    // Load our env variables
    dotenv().ok();

    // Create local server address
    let addr = SocketAddr::from(([0, 0, 0, 0], PORT));
    
    // Load the 'DATABASE_URL' env variable
    let postgres_database_url = env::var("DATABASE_URL").expect("Failed to load 'DATABASE_URL' env variable");

    // Get the postgres pool
    let pool =  create_postgres_pool(postgres_database_url).await.expect("Failed to Initialize postgres database");

    // Initialize the app state
    let app_state = AppState {
        pool
    };

    // Create the hello world route
    let hello_world_route = Router::new().route("/", get(|| async { "Hello World"}));
    let test_route = Router::new().route("/test", get(|| async { "Docker image deployment from github actions Successful!"}));

    let routes = Router::new()
        .merge(hello_world_route)
        .merge(test_route)
        .nest("/api", routes_ticket(app_state));

    Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .expect("Failed to bind to server");

    Ok(())

}