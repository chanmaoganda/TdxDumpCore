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
        let max_days = std::fs::metadata(path.as_ref())?.len();

        Ok(Self { file, max_days })
    }

    pub fn query_days(mut self, days: u64) -> anyhow::Result<Self> {
        let pos_offset = (days * DAY_SIZE as u64) as i64;
        self.file.seek( SeekFrom::End(0 - pos_offset) )?;
        if self.max_days > days {
            self.max_days = days;
        }
        Ok(self)
    }

    pub fn build(mut self) -> anyhow::Result<DayLine> {
        let mut buffer = [0u8; DAY_SIZE];
        let mut day_line = Vec::with_capacity(self.max_days as usize);

        for _ in 0..self.max_days {
            self.file.read_exact(buffer.as_mut())?;
            let date = byteorder::LE::read_u32(&buffer[0..4]);
            let open = byteorder::LE::read_u32(&buffer[4..8]) as f32 / 100f32;
            let high = byteorder::LE::read_u32(&buffer[8..12]) as f32 / 100f32;
            let low = byteorder::LE::read_u32(&buffer[12..16]) as f32 / 100f32;
            let close = byteorder::LE::read_u32(&buffer[16..20]) as f32 / 100f32;
            let daily_data = DailyData::new(date, open, high, low, close);
            day_line.push(daily_data);
        }

        Ok(DayLine::new(day_line))
    }

}


#[test]
fn builder_test() -> anyhow::Result<()> {
    let builder = DayLineBuilder::from_path("../shlday/sh000001.day")?.query_days(10)?;
    let day_line = builder.build()?;
    dbg!(day_line);
    Ok(())
}