Core library for dumping TDX .day files, with optional feature to calculate dif, dea, macd and so on.

## Usage
#### Default is raw data only
```rust
let path = "/path/to/tdx/xxx.day";
let daily = DayLine::new(path);
```

#### With "macd" `feature`, you can calculate dif, dea, macd. with default parameters.
```toml
tdx_dump_core = { version = "0.1", features = ["macd"] }
```

```rust
let path = "/path/to/tdx/xxx.day";
let daily = DayLine::new(path).with_macd();
```