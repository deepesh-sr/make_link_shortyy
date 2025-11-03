use std::error::Error;

use axum::routing::{get, Route};
use axum::Router;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::routes::health;
mod routes;

#[tokio::main]
async fn main()-> Result<(),Box<dyn Error>> {
    
    // Setup logging system for our application (tracing subscriber)
    // This helps us see what's happening in our code when it runs
    
    tracing_subscriber::registry()  // Creates a registry to collect all log messages (events/spans)
    .with(
        // Sets up filtering - decides which messages to show based on importance level
        tracing_subscriber::EnvFilter::try_from_default_env()  // Checks environment variable for log level
        .unwrap_or_else(|_| "link_shortner=debug".into())  // If no env var found, use "debug" level (shows detailed info)
    )
    .with(tracing_subscriber::fmt::layer())  // Formats log messages nicely for console output (human-readable format)
    .init();  // Activates the logging system (makes it the global logger)

    //creating a new route
    let app = Router::new()
            .route("/health", get(health));

    let listener = tokio::net::TcpListener::bind("0.0.0.3000")
            .await
            .expect("Couldnot initialise TCPlisnter");

    // log print the local address ( IP address + PORT )
    tracing::debug!(
        "listening on {}",listener
        .local_addr()
        .expect("could not convert listner address to local address")
    );

    //creating the server
    axum::serve(listener, app)
        .await
        .expect("Could not succesfully create server");

    Ok(())
}

