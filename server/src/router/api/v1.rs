//! This module contains the V1 API router.

/// Creates a new router with V1 routes.
pub(super) fn new() -> axum::Router {
    // TODO: Implement inference endpoint
    axum::Router::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tower::ServiceExt;

    // --- Tests ---
    crate::router::tests::run_test_invalid_path!(new());
}
