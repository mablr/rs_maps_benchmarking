# Rust HashMap Benchmarks

A performance comparison of some HashMap implementations in Rust.

## Running the Benchmarks

```bash
cargo bench
```

Results will be generated in `target/criterion/` with detailed HTML reports.

## Results

![Benchmark Results](data/violin.svg)

### Single-threaded

| HashMap Implementation | Mean Time |
|---|---|
| hashbrown | **259.88 µs** ⚡ |
| std::HashMap | 346.34 µs |

### Multi-threaded (16 threads parking_lot)

| HashMap Implementation | Mean Time |
|---|---|
| DashMap | **1.0749 µs** ⚡ |
| hashbrown + Mutex | 12.142 µs |
| std::HashMap + Mutex | 12.463 µs |

### Multi-threaded (16 threads tokio)

| HashMap Implementation | Mean Time |
|---|---|
| hashbrown + Mutex | 4.1194 µs |
| std::HashMap + Mutex | 4.5277 µs |
