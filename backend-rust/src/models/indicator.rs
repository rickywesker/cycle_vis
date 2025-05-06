// src/models/indicator.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IndicatorResult {
    pub symbol: String,
    pub value: Option<f64>,   // ← 改成 Option
    pub category: String,
}