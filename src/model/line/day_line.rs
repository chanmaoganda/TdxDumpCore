use serde::{Deserialize, Serialize};

use crate::model::data::DailyData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayLine {
    pub data: Vec<DailyData>,
}

impl DayLine {
    pub fn new(data: Vec<DailyData>) -> Self {
        Self { data }
    }
}

impl From<DayLine> for Vec<DailyData> {
    fn from(val: DayLine) -> Self {
        val.data
    }
}