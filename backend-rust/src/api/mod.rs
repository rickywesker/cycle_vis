use axum::{Router};
use axum::routing::get;

mod rsi;

pub fn routes() -> Router {
    Router::new()
        .route("/rsi", get(rsi::get_rsi))
}