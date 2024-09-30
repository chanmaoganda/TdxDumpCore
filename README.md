Core library for dumping TDX .day files, with optional feature to calculate dif, dea, macd and so on.

## Usage

```rust
let builder = DayLineBuilder::from_path("../shlday/sh000001.day")?
    .query_days(10);
let day_line = builder.build();
println!("{:?}", day_line);
```