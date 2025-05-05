// src/models/indicator.rs
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct IndicatorResult {
    pub symbol: String,
    pub value: f64,
    pub category: String,
}