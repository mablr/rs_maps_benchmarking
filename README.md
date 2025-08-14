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
| hashbrown | **213.45 µs** ⚡ |
| std::HashMap | 287.48 µs |

### Multi-threaded (16 threads parking_lot)

| HashMap Implementation | Mean Time |
|---|---|
| DashMap | **412.72 µs** ⚡ |
| hashbrown + Mutex | 6.4446 ms |
| std::HashMap + Mutex | 6.9570 ms |

### Multi-threaded (16 threads tokio)

| HashMap Implementation | Mean Time |
|---|---|
| hashbrown + Mutex | 3.5018 ms |
| std::HashMap + Mutex | 3.5073 ms |
