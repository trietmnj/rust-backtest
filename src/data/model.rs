use chrono::{self, DateTime, Utc};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OHLCV {
    date: DateTime<Utc>,
    symbol: String,
    close: f32,
    open: f32,
    high: f32,
    low: f32,
    volume: f32,
}
