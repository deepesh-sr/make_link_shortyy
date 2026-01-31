use std::error::Error;

use axum::routing::{get, post};
use axum::Router;
use axum_prometheus::PrometheusMetricLayer;
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::routes::{health, shorten_link, redirect_to_url};
mod routes;
mod crud;
#[tokio::main]
async fn main()-> Result<(),Box<dyn Error>> {

    //setup dotenvy 
    dotenvy::dotenv()?;

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

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    // Add database connection 
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await?;

    tracing::info!("Database connection established");

    // adding prometheus integration 
    // prometheus helps in understanding how our server is working, memory usage, site visits etc.
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
    
    // creating routes for the link shortener
    let app = Router::new()
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .route("/health", get(health))
        .route("/shorten", post(shorten_link))  // POST /shorten - Create shortened link
        .route("/{code}", get(redirect_to_url))  // GET /{code} - Redirect to original URL
        .layer(TraceLayer::new_for_http())
        .layer(prometheus_layer)
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Could not initialize TCP listener");

    let addr = listener
        .local_addr()
        .expect("Could not get local address");

    // Log server startup
    tracing::info!("🚀 Link shortener server listening on http://{}", addr);
    tracing::info!("📊 Metrics available at http://{}/metrics", addr);
    tracing::info!("❤️  Health check at http://{}/health", addr);

    // Creating the server
    axum::serve(listener, app)
        .await
        .expect("Could not successfully create server");

    Ok(())
}

