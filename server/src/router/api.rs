//! This module contains all the API version routers.

mod v1;

/// Creates a new router with all the API routes.
pub(super) fn new() -> axum::Router {
    axum::Router::new().nest("/v1", v1::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tower::ServiceExt;

    // --- Tests ---
    crate::router::tests::run_test_invalid_path!(new());
}
