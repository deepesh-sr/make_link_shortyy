// this will contain all the routes endpoints.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json, Redirect},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::crud::{create_link, get_link_by_code, increment_clicks, short_code_exists};

// Health check route
pub async fn health() -> impl IntoResponse {
    // here we are only storing the status code to check whether the application is still up and running.
    (StatusCode::OK, "Service is healthy")
}

// Request body for creating a shortened link
#[derive(Deserialize)]
pub struct CreateLinkRequest {
    pub url: String,
    pub custom_code: Option<String>, // Optional custom short code
}

// Response for created link
#[derive(Serialize)]
pub struct CreateLinkResponse {
    pub short_code: String,
    pub short_url: String,
    pub original_url: String,
}

// Error response
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// Create a shortened link
/// POST /shorten
/// Body: { "url": "https://example.com", "custom_code": "optional" }
pub async fn shorten_link(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateLinkRequest>,
) -> Result<Json<CreateLinkResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Validate URL format
    if !payload.url.starts_with("http://") && !payload.url.starts_with("https://") {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "URL must start with http:// or https://".to_string(),
            }),
        ));
    }

    // Validate URL is not empty
    if payload.url.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "URL cannot be empty".to_string(),
            }),
        ));
    }

    // Generate or use custom short code
    let short_code = if let Some(custom) = payload.custom_code {
        // Validate custom code
        if custom.len() < 3 || custom.len() > 10 {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Custom code must be between 3 and 10 characters".to_string(),
                }),
            ));
        }

        // Check if code contains only alphanumeric characters
        if !custom.chars().all(|c| c.is_alphanumeric()) {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Custom code must contain only alphanumeric characters".to_string(),
                }),
            ));
        }

        // Check if code already exists
        match short_code_exists(&pool, &custom).await {
            Ok(exists) => {
                if exists {
                    return Err((
                        StatusCode::CONFLICT,
                        Json(ErrorResponse {
                            error: "This custom code is already taken".to_string(),
                        }),
                    ));
                }
            }
            Err(_) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "Failed to check if code exists".to_string(),
                    }),
                ));
            }
        }

        custom
    } else {
        // Generate random short code
        generate_short_code(&pool).await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to generate short code".to_string(),
                }),
            )
        })?
    };

    // Insert into database
    match create_link(&pool, &short_code, &payload.url).await {
        Ok(_) => Ok(Json(CreateLinkResponse {
            short_code: short_code.clone(),
            short_url: format!("http://localhost:3000/{}", short_code),
            original_url: payload.url,
        })),
        Err(e) => {
            tracing::error!("Failed to create link: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to create shortened link".to_string(),
                }),
            ))
        }
    }
}

/// Redirect to original URL
/// GET /:code
pub async fn redirect_to_url(
    State(pool): State<PgPool>,
    Path(short_code): Path<String>,
) -> Result<Redirect, StatusCode> {
    // Get link from database
    let link = match get_link_by_code(&pool, &short_code).await {
        Ok(Some(link)) => link,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to get link: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Increment click count asynchronously (don't wait for it)
    let pool_clone = pool.clone();
    let code_clone = short_code.clone();
    tokio::spawn(async move {
        if let Err(e) = increment_clicks(&pool_clone, &code_clone).await {
            tracing::error!("Failed to increment clicks: {:?}", e);
        }
    });

    // Redirect to original URL
    Ok(Redirect::permanent(&link.original_url))
}

/// Generate a unique random short code
async fn generate_short_code(pool: &PgPool) -> Result<String, sqlx::Error> {
    use rand::{Rng, distr::Alphanumeric};

    // Try up to 10 times to generate a unique code
    for _ in 0..10 {
        let short_code: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect();

        // Check if this code already exists
        let exists = short_code_exists(pool, &short_code).await?;
        if !exists {
            return Ok(short_code);
        }
    }

    // If we couldn't generate a unique code after 10 tries, use a longer code
    let short_code: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    Ok(short_code)
}