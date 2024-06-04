use std::{env};
use axum::{response::Html, routing::get, Router};
// use dotenv::dotenv;

#[tokio::main]
async fn main(){

    //loading the environment
    dotenv::dotenv().ok();
    
    let hello_route = Router::new().route("/hw",get(|| async {Html("Hello")}));



    //start server
    let portno = env::var("PORT").expect("4000");
    let s = format!("0.0.0.0:{}",portno);
    let listener = tokio::net::TcpListener::bind(&s).await.unwrap();
    println!("Server running on {}",s);
    axum::serve(listener,hello_route).await.unwrap();
}