use std::error::Error;

use axum::routing::get;
use axum::Router;
use axum_prometheus::PrometheusMetricLayer;
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::routes::health;
mod routes;
// mod utils;
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

    let database_url = std::env::var("DATABASE_URL").expect("Provide URL");
    // Add database connection 
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url).await?;
    // "CREATE TABLE books(title VARCHAR(50) owner VARCHAR(50)), INSERT INTO books(title,owner) VALUE("Dp","Hey"), SELECT * FROM books

    // let books = sqlx::query!(
    //    "
    //    CREATE TABLE books(title VARCHAR(50) owner VARCHAR(50))
    //    INSERT INTO books(title,owner) VALUE(value1,value2)
    //     SELECT * FROM books",
    // )

    // sqlx::query("CREATE TABLE testbook(title VARCHAR(50), owner VARCHAR(50));").execute(&pool).await?;
    // sqlx::query("INSERT INTO testbook(title,owner) VALUES($1,$2)").bind("Hello Rust").bind("VXV").execute(&pool).await?;
    let row = sqlx::query("SELECT * FROM testbook").fetch_all(&pool).await?;

    println!("{row:?}");

    // adding prometheus integraion 
        // prometheus helps in understanding how our server is working , memory usage, site visits etc.
    let (prometheus_layer , metric_handle) = PrometheusMetricLayer::pair();
    //creating a new route
    let app = Router::new()
            .route("/metric", get(|| async move {metric_handle.render()}))
            .route("/health", get(health))
            .layer(TraceLayer::new_for_http())
            .layer(prometheus_layer)
            .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
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

