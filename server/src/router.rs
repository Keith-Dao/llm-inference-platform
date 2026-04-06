//! This module contains all the routers.

mod api;

/// Creates a health check router.
fn health_check_router() -> axum::Router {
    axum::Router::new().route(
        "/",
        axum::routing::get(|| async { axum::Json(serde_json::json!({"status": "ok"})) }),
    )
}

/// Creates a new router with the top level routes.
pub(super) fn new() -> axum::Router {
    axum::Router::new()
        .route(
            "/metrics",
            axum::routing::get(|| async {
                // TODO: Add handler for metrics
                "Metrics"
            }),
        )
        .nest("/api", api::new())
        .nest("/health", health_check_router())
}
