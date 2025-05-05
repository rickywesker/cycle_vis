/// 計算 RSI 序列（Wilder EMA 版）
/// prices: 收盤價序列；period: 週期（通常 14）
/// 回傳與 prices 等長的 Vec<f64>，前 period 個值會是 NaN

pub fn calculate_rsi(prices: &[f64], period: usize) -> Vec<f64> {

    let n: usize = prices.len();
    // === 新增：如果数据点不够，就全部返回 NaN ===
    if n <= period {
        return vec![f64::NAN; n];
    }
    let mut rsi: Vec<f64> = Vec::with_capacity(n);

    // 差分：第一個放 None
    let mut deltas: Vec<f64> = vec![0.0; n];
    for i in 1..n {
        deltas[i] = prices[i] - prices[i - 1];
    }

    // 初始平均漲跌幅 (simple average)
    let mut avg_gain: f64 = deltas[1..=period]
        .iter()
        .filter(|&&d| d > 0.0)
        .sum::<f64>()
        / period as f64;
    let mut avg_loss = deltas[1..=period]
        .iter()
        .filter(|&&d| d < 0.0)
        .map(|&d| -d)
        .sum::<f64>()
        / period as f64;

    // 前 period 個值放 NaN
    for _ in 0..=period {
        rsi.push(f64::NAN);
    }

    // Wilder EMA: α = 1/period
    let alpha = 1.0 / period as f64;
    for i in (period + 1)..n {
        let delta = deltas[i];
        let gain = if delta > 0.0 { delta } else { 0.0 };
        let loss = if delta < 0.0 { -delta } else { 0.0 };

        avg_gain = (avg_gain * (period as f64 - 1.0) + gain) * alpha;
        avg_loss = (avg_loss * (period as f64 - 1.0) + loss) * alpha;

        let rs = if avg_loss.abs() < std::f64::EPSILON {
            std::f64::INFINITY
        } else {
            avg_gain / avg_loss
        };
        rsi.push(100.0 - (100.0 / (1.0 + rs)));
    }

    rsi
}

/// 根據 RSI 值回傳分類
pub fn categorize_rsi(rsi: f64) -> String {
    if rsi.is_nan() {
        "na".into()
    } else if rsi > 70.0 {
        "overbought".into()
    } else if rsi < 30.0 {
        "oversold".into()
    } else {
        "neutral".into()
    }
}