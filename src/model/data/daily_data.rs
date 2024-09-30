#[derive(Debug, Clone)]
pub struct DailyData {
    pub date: u32,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub turnover: f32,
    pub volume: f32,
    // last 4 bytes are padding and should be ignored
}

impl DailyData {
    pub fn new(date: u32, open: f32, high: f32, low: f32, close: f32, turnover: f32, volume: f32) -> Self {
        Self {
            date,
            open,
            high,
            low,
            close,
            turnover,
            volume,
        }
    }
}