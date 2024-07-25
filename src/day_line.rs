use std::path::Path;

use crate::daily::{DailyData, RawDaily};

#[cfg(feature = "macd")]
use ta::{indicators::MovingAverageConvergenceDivergence as MACD, Next};

#[derive(Debug)]
pub struct DayLine {
    pub data: Vec<DailyData>,
    pub total_days: usize,
}

#[cfg(feature = "macd")]
impl DayLine {

    pub fn new(path: impl AsRef<Path>) -> Self {
        let bytes = std::fs::read(path).unwrap();
        assert!(bytes.len() % 32 == 0);
        let total_days = bytes.len() / 32;
        let mut raw_data = Vec::with_capacity(total_days);
        let _ = (0..total_days).into_iter().for_each(|iteration| {
            let start_pos = iteration * 32;
            let end_pos = start_pos + 32;

            let daily = RawDaily::from_stream(bytes[start_pos..end_pos].to_vec());
            raw_data.push(daily);
        });

        Self {
            data: Self::additional_info(total_days, raw_data),
            total_days,
        }
    }

    fn additional_info(total_days: usize, raw_data: Vec<RawDaily>) -> Vec<DailyData> {
        let mut macd = MACD::default();
        let mut daily_data = Vec::with_capacity(total_days);
        raw_data.into_iter().for_each(|daily| {
            let (diff, dea, raw_macd) = macd.next(daily.get_close_price()).into();
            let macd = raw_macd * 2 as f64;
            daily_data.push(DailyData::new(daily, diff, dea, macd));
        });
        daily_data
    }
}

#[cfg(not(feature = "macd"))]
impl DayLine {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let bytes = std::fs::read(path).unwrap();
        assert!(bytes.len() % 32 == 0);
        let total_days = bytes.len() / 32;
        let mut data = Vec::with_capacity(total_days);
        let _ = (0..total_days).into_iter().for_each(|iteration| {
            let start_pos = iteration * 32;
            let end_pos = start_pos + 32;

            let raw_daily = RawDaily::from_stream(bytes[start_pos..end_pos].to_vec());
            let daily = DailyData::new(raw_daily);
            data.push(daily);
        });

        Self {
            data,
            total_days,
        }
    }
}