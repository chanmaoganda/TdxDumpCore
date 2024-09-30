use serde::{Deserialize, Serialize};

use crate::model::data::DailyData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayLine {
    data: Vec<DailyData>,
}

impl DayLine {
    pub fn new(data: Vec<DailyData>) -> Self {
        Self { data }
    }
}