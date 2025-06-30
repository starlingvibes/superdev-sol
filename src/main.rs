use axum::{
    response::Json as ResponseJson,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod handlers;
mod types;

use handlers::*;
use types::*;

#[tokio::main]
async fn main() {
    // Build the router with all endpoints
    let app = Router::new()
        .route("/", get(health_check))
        .route("/keypair", post(generate_keypair))
        .route("/token/create", post(create_token))
        .route("/token/mint", post(mint_token))
        .route("/message/sign", post(sign_message))
        .route("/message/verify", post(verify_message))
        .route("/send/sol", post(send_sol))
        .route("/send/token", post(send_token));

    // Start the server
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "10000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");
    
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("🚀 Solana HTTP Server running at http://{}", addr);
    
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> ResponseJson<ApiResponse<String>> {
    ResponseJson(ApiResponse::success("Solana HTTP Server is running!".to_string()))
}
