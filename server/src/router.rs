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

#[cfg(test)]
mod tests {
    use super::*;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    // --- Helper macros ---

    /// Provides a generic invalid path test.
    macro_rules! run_test_invalid_path {
        ($app: expr) => {
            #[tokio::test]
            async fn test_invalid_path() {
                let response = $app
                    .oneshot(
                        axum::http::Request::get("/invalid-path")
                            .body(axum::body::Body::empty())
                            .unwrap(),
                    )
                    .await
                    .unwrap();

                assert_eq!(response.status(), axum::http::StatusCode::NOT_FOUND);
            }
        };
    }
    pub(super) use run_test_invalid_path;

    // --- Tests ---
    run_test_invalid_path!(new());

    #[tokio::test]
    async fn test_health_endpoint() {
        let app = new();

        let response = app
            .oneshot(
                axum::http::Request::get("/health")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), axum::http::StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value =
            serde_json::from_slice(&body).expect("Should be a JSON response body.");
        assert_eq!(body, serde_json::json!({"status": "ok"}));
    }

    #[tokio::test]
    async fn test_metrics() {
        let app = new();

        let response = app
            .oneshot(
                axum::http::Request::get("/metrics")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), axum::http::StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Metrics");
    }
}
