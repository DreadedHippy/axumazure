use std::{net::SocketAddr, env};

use axum::{Router, routing::get, Server};
use dotenv::dotenv;
use routes::routes_ticket;
use schemas::AppState;
use utils::create_postgres_pool;

mod routes; // Holds all routes
mod schemas; // Holds all app schemas 
mod handlers; // Holds all handlers
mod utils; // Holds all utility functions

// Declare port for code to run on (3000 in this case)
const PORT: u16 = 3000;

#[tokio::main]
async fn main() -> Result<(), ()> {
    // Load in our environment variables...
    dotenv().ok();

    // Create the localhost address
    let address = SocketAddr::from(([0, 0, 0, 0], PORT));

    // Create a "hello world" route
    let hello_world_route = Router::new().route("/", get(|| async { "Hello, World!" }));

    // Load the 'DATABASE_URL' env variable
    let postgres_database_url = env::var("DATABASE_URL").expect("Failed to load 'DATABASE_URL' env variable");

    // Get the postgres pool
    let pool =  create_postgres_pool(postgres_database_url).await.expect("Failed to Initialize postgres database");

    // Initialize the app state
    let app_state = AppState {
        pool
    };

    // Create the Universal router
    let routes = Router::new()
        .merge(hello_world_route)
        .nest("/api", routes_ticket(app_state));

    // Bind to localhost
    Server::bind(&address)
        .serve(routes.into_make_service())
        .await
        .expect("Failed to bind to server");


    Ok(())
}