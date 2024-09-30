use std::{fs::File, io::{Read, Seek, SeekFrom}, path::Path};

use byteorder::ByteOrder;

use crate::model::{data::DailyData, DayLine};

const DAY_SIZE: usize = 32;

#[derive(Debug)]
pub struct DayLineBuilder {
    file: File,
    max_days: u64,
}

impl DayLineBuilder {
    pub fn from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let file = File::open(path.as_ref())?;
        let max_days = std::fs::metadata(path.as_ref())?.len() / (DAY_SIZE as u64);

        Ok(Self { file, max_days })
    }

    pub fn query_days(mut self, days: u64) -> Self {
        if self.max_days > days {
            // take the smaller one between the two values
            self.max_days = days;
        }

        let pos_offset = (self.max_days * DAY_SIZE as u64) as i64;
        self.file.seek( SeekFrom::End(0 - pos_offset) ).unwrap();
        
        self
    }

    pub fn build(mut self) -> DayLine {
        let mut buffer = [0u8; DAY_SIZE];
        let mut day_line = Vec::with_capacity(self.max_days as usize);

        for _ in 0..self.max_days {
            self.file.read_exact(buffer.as_mut()).unwrap();
            let date = byteorder::LE::read_u32(&buffer[0..4]);
            let open = byteorder::LE::read_u32(&buffer[4..8]) as f32 / 100f32;
            let high = byteorder::LE::read_u32(&buffer[8..12]) as f32 / 100f32;
            let low = byteorder::LE::read_u32(&buffer[12..16]) as f32 / 100f32;
            let close = byteorder::LE::read_u32(&buffer[16..20]) as f32 / 100f32;
            // last 12 bytes consists of 4 bytes of turnover, 4 bytes of volume, and last 4 bytes reserved
            // those bytes are not needed now, so we drop these bytes
            let daily_data = DailyData::new(date, open, high, low, close);
            day_line.push(daily_data);
        }

        DayLine::new(day_line)
    }

}


#[test]
fn builder_test() -> anyhow::Result<()> {
    let builder = DayLineBuilder::from_path("../shlday/sh000001.day")?
        .query_days(10000);
    let day_line = builder.build();
    dbg!(day_line.data.len());
    Ok(())
}