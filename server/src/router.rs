//! This module contains all the routers.

mod api;

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
}
