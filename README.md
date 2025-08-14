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
| hashbrown | **216.68 µs** ⚡ |
| std::HashMap | 307.45 µs |

### Multi-threaded (16 threads parking_lot)

| HashMap Implementation | Mean Time |
|---|---|
| DashMap | **1.0545 ms** ⚡ |
| hashbrown + Mutex | 11.596 ms |
| std::HashMap + Mutex | 12.460 ms |

