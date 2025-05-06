use axum::{extract::Query, response::IntoResponse, Json};
use futures::future::join_all;
use redis::AsyncCommands;

use crate::{
    data::loader::{fetch_klines, fetch_top_symbols},
    indicators::rsi::{calculate_rsi, categorize_rsi},
    models::indicator::IndicatorResult,
};

/// URI 查询参数
#[derive(serde::Deserialize)]
pub struct RsiParams {
    symbols:  Option<String>,
    interval: Option<String>,
    limit:    Option<usize>,
    period:   Option<usize>,
}

/* ------------ Redis 连接工具 -------------- */
async fn redis_conn() -> redis::RedisResult<redis::aio::Connection> {
    // 记得把密码 foobared 换成你真实的
    redis::Client::open("redis://:foobared@127.0.0.1/")?
        .get_async_connection()
        .await
}

/* ------------ 业务 handler -------------- */
pub async fn get_rsi(Query(params): Query<RsiParams>) -> impl IntoResponse {
    // 0. 组合缓存 key
    let interval = params.interval.clone().unwrap_or_else(|| "1d".into());
    let limit    = params.limit.unwrap_or(500);
    let period   = params.period.unwrap_or(14);
    let raw_syms = params.symbols.clone().unwrap_or_else(|| "AUTO_TOP200".into());
    let cache_key = format!("rsi:{}:{}:{}:{}", raw_syms, &interval, limit, period);

    /* ---------- 1. 先看 Redis ---------- */
    if let Ok(mut conn) = redis_conn().await {
        // 👇 使用 Option<String>，key 不存在时得到 Ok(None)
        if let Ok(Some(raw)) = conn.get::<_, Option<String>>(&cache_key).await {
            if let Ok(data) = serde_json::from_str::<Vec<IndicatorResult>>(&raw) {
                tracing::info!("cache hit {}", cache_key);
                return Json(data);          // ★ 命中立即返回，不再往下算
            }
        }
    }

    /* ---------- 2. 真正计算 ---------- */
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
                    let rsi_series = calculate_rsi(&prices, period);
                    let last = *rsi_series.last().unwrap_or(&f64::NAN);
                    IndicatorResult {
                        symbol,
                        value: if last.is_nan() { None } else { Some(last) },
                        category: categorize_rsi(last),
                    }
                }
                Err(err) => IndicatorResult {
                    symbol,
                    value: None,
                    category: format!("error: {err}"),
                },
            }
        }
    });

    let results: Vec<IndicatorResult> = join_all(tasks).await;

    /* ---------- 3. 回写 Redis (TTL 300 s) ---------- */
    if let Ok(mut conn) = redis_conn().await {
        // 忽略错误：即使写失败也照常返回
        let _ : redis::RedisResult<()> =
            conn.set_ex(&cache_key, serde_json::to_string(&results).unwrap(), 43200).await;
    }

    Json(results)
}