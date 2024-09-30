use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyIndicator {
    dif: f32,
    dea: f32,
    macd: f32,
}