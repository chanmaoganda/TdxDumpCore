use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyData {
    pub date: u32,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    // last 4 bytes are padding and should be ignored
}

impl DailyData {
    pub fn new(date: u32, open: f32, high: f32, low: f32, close: f32) -> Self {
        Self {
            date,
            open,
            high,
            low,
            close,
        }
    }
}