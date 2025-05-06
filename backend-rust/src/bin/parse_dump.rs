//! cargo run --bin parse_dump      # 运行命令
use std::fs;
use anyhow::Result;

use serde_json;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct IndicatorResult {
    pub symbol: String,
    pub value: Option<f64>,   // ← 改成 Option
    pub category: String,
}
fn main() -> Result<()> {
    // 1. 读文件
    let raw = fs::read_to_string("dump.json")?;

    // 2. 反序列化
    let data: Vec<IndicatorResult> = serde_json::from_str(&raw)?;

    // 3. 打印
    for item in &data {
        println!("{item:?}");
    }

    println!("共 {} 条", data.len());
    Ok(())
}