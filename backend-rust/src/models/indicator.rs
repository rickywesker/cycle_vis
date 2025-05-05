use serde::Serialize;

/// 通用的技術指標回傳格式
#[derive(Serialize)]
pub struct IndicatorResult {
    pub symbol: String,
    pub value: f64,
    pub category: String,
}