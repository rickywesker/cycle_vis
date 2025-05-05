use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 從 Binance 抓取 K 線收盤價
///
/// - `symbol`：交易對，如 "BTCUSDT"
/// - `interval`：K 線週期，如 "1d"
/// - `limit`：最多抓幾根 K 線（最大 1000）
///
/// 回傳一個 Vec<f64>，每個元素對應一根 K 線的收盤價
pub async fn fetch_klines(symbol: &str, interval: &str, limit: usize) -> Result<Vec<f64>> {
    let url = format!(
        "https://api.binance.com/api/v3/klines?symbol={symbol}&interval={interval}&limit={limit}",
        symbol = symbol,
        interval = interval,
        limit = limit,
    );

    let client = Client::new();
    let resp: Vec<Vec<Value>> = client
        .get(&url)
        .send()
        .await?
        .json()
        .await?;

    // K 線格式參考 Binance doc，收盤價在 index=4
    let closes = resp
        .into_iter()
        .map(|k| {
            k[4]
                .as_str()
                .expect("close should be string")
                .parse::<f64>()
                .expect("valid float")
        })
        .collect();

    Ok(closes)
}

/// 對應 Binance /api/v3/ticker/24hr 回傳的部分欄位
#[derive(Deserialize)]
struct Ticker24h {
    symbol: String,
    quoteVolume: String,
}

/// 取得交易量（quoteVolume）最高的前 top_k 個交易對（只看 USDT 交易對，並排除穩定幣）
pub async fn fetch_top_symbols(top_k: usize) -> Result<Vec<String>> {
    // 1) 請求所有 24hr tickers
    let url = "https://api.binance.com/api/v3/ticker/24hr";
    let client = Client::new();
    let resp: Vec<Ticker24h> = client
        .get(url)
        .send()
        .await?
        .json()
        .await?;

    // 2) 過濾：只保留以 USDT 結尾，並排除某些穩定幣
    let excludes = ["DARUSDT", "USDCUSDT", "FDUSDUSDT"];
    let mut filtered: Vec<_> = resp
        .into_iter()
        .filter(|t| {
            t.symbol.ends_with("USDT") && !excludes.contains(&t.symbol.as_str())
        })
        .collect();

    // 3) 依 quoteVolume 排序 (降冪)
    filtered.sort_by(|a, b| {
        let va = a.quoteVolume.parse::<f64>().unwrap_or(0.0);
        let vb = b.quoteVolume.parse::<f64>().unwrap_or(0.0);
        vb.partial_cmp(&va).unwrap()
    });

    // 4) 取前 top_k 的 symbol 回傳
    let top: Vec<String> = filtered
        .into_iter()
        .take(top_k)
        .map(|t| t.symbol)
        .collect();

    Ok(top)
}