use axum::{routing::get, Router};
use tokio::net::TcpListener;
use std::net::SocketAddr;
use anyhow::Result;

// 👇 新增
use tower_http::cors::{CorsLayer, Any};
use http::{HeaderValue, Method};

mod api;
mod indicators;
mod models;
mod data;

#[tokio::main]
async fn main() -> Result<()> {
    // ── 1. CORS 层 ───────────────────────────────────────────────
    // 生产：只允许正式前端域名
    // 开发 (cargo run) 时也想本地访问，可加 cfg(debug_assertions)
    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_headers(Any)
        .allow_origin(
            "https://www.cyclestudy.org"
                .parse::<HeaderValue>()
                .unwrap(),
        );

    // ── 2. 组装 Router，并套上 CORS ─────────────────────────────
    let app = Router::new()
        .nest("/api", api::routes())
        .layer(cors);                     // 👈 挂 CORS

    // ── 3. 绑定监听地址 ──────────────────────────────────────────
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("🚀 Server running at http://{}", addr);

    // ── 4. 启动服务 ─────────────────────────────────────────────
    axum::serve(listener, app).await?;
    Ok(())
}