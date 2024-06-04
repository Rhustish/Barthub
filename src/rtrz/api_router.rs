use axum::{Router , routing::{*}};
use crate::ctrlz::health_check::health_check;

pub fn app_router() ->Router {
    Router::new()
    .route("/hc",get(health_check))
}