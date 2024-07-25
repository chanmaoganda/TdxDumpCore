use serde::Serialize;
use std::io::Cursor;

#[cfg(feature = "macd")]
#[derive(Debug, Clone, Serialize)]
pub struct DailyData {
    date: u32,
    open: f32,
    high: f32,
    low: f32,
    close: f32,
    turnover: f32,
    volume: f32,
    diff: f64,
    dea: f64,
    macd: f64,
}

#[cfg(feature = "macd")]
impl DailyData {
    pub fn new(raw_daily: RawDaily, diff: f64, dea: f64, macd: f64) -> Self {
        let RawDaily {
            date,
            open,
            high,
            low,
            close,
            turnover,
            volume,
        } = raw_daily;
        Self {
            date,
            open,
            high,
            low,
            close,
            turnover,
            volume,
            diff,
            dea,
            macd,
        }
    }
}

#[cfg(not(feature = "macd"))]
#[derive(Debug, Clone, Serialize)]
pub struct DailyData {
    date: u32,
    open: f32,
    high: f32,
    low: f32,
    close: f32,
    turnover: f32,
    volume: f32,
}

#[cfg(not(feature = "macd"))]
impl DailyData {
    pub fn new(raw_daily: RawDaily) -> Self {
        let RawDaily {
            date,
            open,
            high,
            low,
            close,
            turnover,
            volume,
        } = raw_daily;
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

#[derive(Debug, Clone)]
pub struct RawDaily {
    date: u32,
    open: f32,
    high: f32,
    low: f32,
    close: f32,
    turnover: f32,
    volume: f32,
}

impl RawDaily {
    pub fn from_stream(stream: Vec<u8>) -> Self {
        let mut stream = Cursor::new(stream);
        let date = parser::take_next_4integer(&mut stream);
        let open = parser::take_next_4integer(&mut stream) as f32 / 100.;
        let high = parser::take_next_4integer(&mut stream) as f32 / 100.;
        let low = parser::take_next_4integer(&mut stream) as f32 / 100.;
        let close = parser::take_next_4integer(&mut stream) as f32 / 100.;
        let turnover = parser::take_next4_float(&mut stream);
        let volume = parser::take_next_4integer(&mut stream) as f32 / 100.;
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

    pub fn get_close_price(&self) -> f64 {
        self.close as f64
    }

    pub fn get_date(&self) -> u32 {
        self.date
    }
}

mod parser {
    use std::io::{Cursor, Read};

    pub fn take_next_4integer(cursor: &mut Cursor<Vec<u8>>) -> u32 {
        let mut bytes = [0u8; 4];
        cursor.read_exact(&mut bytes).unwrap();
        u32::from_le_bytes(bytes)
    }

    pub fn take_next4_float(cursor: &mut Cursor<Vec<u8>>) -> f32 {
        let mut bytes = [0u8; 4];
        cursor.read_exact(&mut bytes).unwrap();
        f32::from_le_bytes(bytes)
    }
}
