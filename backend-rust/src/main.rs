use axum::{routing::get, Router};
use tokio::net::TcpListener;
use std::net::SocketAddr;
use anyhow::Result;

mod api;
mod indicators;
mod models;
mod data;
#[tokio::main]
async fn main() -> Result<()> {
    // 1) 組裝你的 router
    let app = Router::new()
        .nest("/api", api::routes());

    // 2) 綁定 TCP listener
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("🚀 Server running at http://{}", addr);

    // 3) 用 axum::serve 啟動
    axum::serve(listener, app).await?;

    Ok(())
}