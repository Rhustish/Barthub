use axum::response::{IntoResponse, Html};

pub async fn health_check() -> impl IntoResponse{
    println!("--> {:<12} - health_check","Handler");
    Html("HC OK")
}