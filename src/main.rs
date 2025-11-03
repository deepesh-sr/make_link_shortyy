use std::error::Error;

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
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


    Ok(())
}

