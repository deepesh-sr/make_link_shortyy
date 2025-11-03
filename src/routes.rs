// this will contain all the routes endpoints.

// this allow to return anything that axum can return into response. 
use axum::{http::StatusCode, response::IntoResponse};

// first is the simple health route.
pub async fn health()-> impl IntoResponse {
    // here we are only storing the status code to check whether the application is still up and running. 
    (StatusCode::OK,"Service is healthy")
}