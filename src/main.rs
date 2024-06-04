use std::env;
pub mod rtrz;
pub mod ctrlz;

use crate::rtrz::api_router::app_router;

#[tokio::main]
async fn main(){
    //loading the environment
    dotenv::dotenv().ok();
    
    //start server
    let portno = env::var("PORT").expect("4000");
    let s = format!("0.0.0.0:{}",portno);
    let listener = tokio::net::TcpListener::bind(&s).await.unwrap();
    println!("Server running on {}",s);
    axum::serve(listener,app_router()).await.unwrap();
}

