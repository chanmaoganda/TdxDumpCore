use crate::model::data::DailyData;

#[derive(Debug, Clone)]
pub struct DayLine {
    data: Vec<DailyData>,
}

impl DayLine {
    pub fn new(data: Vec<DailyData>) -> Self {
        Self { data }
    }
}