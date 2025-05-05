use axum::{
    extract::Query,
    response::IntoResponse,
    Json,
};
use anyhow::Result;
use redis::AsyncCommands;
use crate::data::loader::{fetch_klines, fetch_top_symbols};
use crate::indicators::rsi::{calculate_rsi, categorize_rsi};
use crate::models::indicator::IndicatorResult;
use futures::future::join_all;
/// 解析 URI 查詢參數
/// - symbols: 以逗號分隔的交易對列表 (e.g. "BTCUSDT,ETHUSDT")
/// - interval: K 線週期 (預設 "1d")
/// - limit: 最多抓幾根 K 線 (預設 500)
/// - period: RSI 週期 (預設 14)
#[derive(serde::Deserialize)]
pub struct RsiParams {
    symbols: Option<String>,
    interval: Option<String>,
    limit: Option<usize>,
    period: Option<usize>,
}


// ---------------- 帮助函数：拿一个异步连接 ----------------
async fn redis_conn() -> redis::RedisResult<redis::aio::Connection> {
    let client = redis::Client::open("redis://:foobared@127.0.0.1/")?;
    client.get_async_connection().await
}


// pub async fn get_rsi(Query(params): Query<RsiParams>) -> impl IntoResponse {
//     // ===== 1. symbols 出自 query，否則自動抓 top 200 =====
//     let symbols: Vec<String> = if let Some(s) = params.symbols {
//         s.split(',')
//          .map(str::trim)
//          .map(String::from)
//          .collect()
//     } else {
//         // 預設 top 200
//         fetch_top_symbols(200).await.unwrap_or_default()
//     };

//     let interval = params.interval.unwrap_or_else(|| "1d".into());
//     let limit = params.limit.unwrap_or(500);
//     let period = params.period.unwrap_or(14);

    

//     // 1. 为每个 symbol 构建一个异步任务
//     let tasks: Vec<_> = symbols.into_iter().map(|symbol| {
//         let interval = interval.clone();
//         async move {
//             // 抓 K 线
//             match fetch_klines(&symbol, &interval, limit).await {
//                 Ok(prices) => {
//                     let rsi_series = calculate_rsi(&prices, period);
//                     let last = *rsi_series.last().unwrap_or(&f64::NAN);
//                     IndicatorResult {
//                         symbol: symbol.clone(),
//                         value: last,
//                         category: categorize_rsi(last),
//                     }
//                 }
//                 Err(err) => IndicatorResult {
//                     symbol: symbol.clone(),
//                     value: f64::NAN,
//                     category: format!("error: {}", err),
//                 },
//             }
//         }
//     }).collect();

//     // 2. 并行执行所有任务
//     let results: Vec<IndicatorResult> = join_all(tasks).await;

//     // 3. 返回 JSON
//     Json(results)
//     }

pub async fn get_rsi(Query(params): Query<RsiParams>) -> impl IntoResponse {
    // ---------- 0. 生成缓存 key ----------
    let interval = params.interval.clone().unwrap_or_else(|| "1d".into());
    let limit    = params.limit.unwrap_or(500);
    let period   = params.period.unwrap_or(14);

    let raw_syms = params.symbols.clone().unwrap_or_else(|| "AUTO_TOP200".into());
    let cache_key = format!("rsi:{}:{}:{}:{}", raw_syms, interval, limit, period);

    // ---------- 1. 先查 Redis ----------
    if let Ok(mut conn) = redis_conn().await {
        if let Ok(cached_json) = conn.get::<String, String>(cache_key.clone()).await {
            if let Ok(data) = serde_json::from_str::<Vec<IndicatorResult>>(&cached_json) {
                return Json(data);
            }
        }
    }

    // ---------- 2. 没有缓存 → 真正计算 ----------
    let symbols: Vec<String> = if let Some(s) = params.symbols {
        s.split(',').map(str::trim).map(String::from).collect()
    } else {
        fetch_top_symbols(200).await.unwrap_or_default()
    };

    let tasks = symbols.into_iter().map(|symbol| {
        let interval = interval.clone();
        async move {
            match fetch_klines(&symbol, &interval, limit).await {
                Ok(prices) => {
                    let rsi = calculate_rsi(&prices, period);
                    let last = *rsi.last().unwrap_or(&f64::NAN);
                    IndicatorResult{ symbol, value: last, category: categorize_rsi(last) }
                }
                Err(e) => IndicatorResult{ symbol, value: f64::NAN, category: format!("error: {e}") },
            }
        }
    });

    let results: Vec<IndicatorResult> = join_all(tasks).await;

    // ---------- 3. 写回 Redis，TTL 30 秒 ----------
    if let Ok(mut conn) = redis_conn().await {
        let _ : redis::RedisResult<()> =
            conn.set_ex::<String, String, ()>(
                cache_key,
                serde_json::to_string(&results).unwrap(),
                30).await;
            }
    Json(results)
}

