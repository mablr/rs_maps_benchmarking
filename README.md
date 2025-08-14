# Rust HashMap Benchmarks

A performance comparison of some HashMap implementations in Rust.

## Running the Benchmarks

```bash
cargo bench
```

Results will be generated in `target/criterion/` with detailed HTML reports.

## Results for 1M operations

![Benchmark Results](data/violin.svg)

### Single-threaded

| HashMap Implementation | Mean Time |
|---|---|
| hashbrown | **23.705 ms** ⚡ |
| std::HashMap | 37.945 ms |

### Multi-threaded (16 threads parking_lot)

| HashMap Implementation | Mean Time |
|---|---|
| DashMap | **47.665 ms** ⚡ |
| hashbrown + Mutex | 787.28 ms |
| std::HashMap + Mutex | 1.1380 s |

### Multi-threaded (16 threads tokio)

| HashMap Implementation | Mean Time |
|---|---|
| hashbrown + Mutex | 380.36 ms |
| std::HashMap + Mutex | 425.98 ms |
