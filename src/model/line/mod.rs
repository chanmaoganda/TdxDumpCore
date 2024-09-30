mod day_line;

pub use day_line::DayLine;

pub enum Line {
    DailyLine(DayLine),
}
