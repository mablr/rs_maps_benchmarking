use criterion::{Criterion, criterion_group, criterion_main};
use dashmap::DashMap;
use hashbrown::HashMap as HashbrownHashMap;
use parking_lot::Mutex as ParkingLotMutex;
use rand::{Rng, thread_rng};
use std::collections::HashMap as StdHashMap;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::Mutex as TokioMutex;

const N: usize = 1_000_000; // Number of ops per benchmark

fn bench_maps(c: &mut Criterion) {
    let mut group = c.benchmark_group("maps");

    group.bench_function("hashbrown_single_thread", |b| {
        b.iter(|| {
            let mut map: HashbrownHashMap<u64, u64> = HashbrownHashMap::with_capacity(N);
            for _ in 0..N {
                let k = thread_rng().gen_range(0..N as u64);
                map.insert(k, k + 1);
            }
        })
    });

    group.bench_function("std_hashmap_single_thread", |b| {
        b.iter(|| {
            let mut map: StdHashMap<u64, u64> = StdHashMap::with_capacity(N);
            for _ in 0..N {
                let k = thread_rng().gen_range(0..N as u64);
                map.insert(k, k + 1);
            }
        })
    });

    group.bench_function("hashbrown_multi_thread_parking_lot", |b| {
        b.to_async(Runtime::new().unwrap())
            .iter(|| async {
                let map = Arc::new(ParkingLotMutex::new(
                    HashbrownHashMap::<u64, u64>::with_capacity(N),
                ));
                let mut handles = Vec::new();
                for _ in 0..16 {
                    // 16 threads
                    let m = map.clone();
                    handles.push(tokio::spawn(async move {
                        for _ in 0..(N / 16) {
                            let k = thread_rng().gen_range(0..N as u64);
                            m.lock().insert(k, k + 1);
                        }
                    }));
                }
                for h in handles {
                    h.await.unwrap();
                }
            })
    });

    group.bench_function("std_hashmap_multi_thread_parking_lot", |b| {
        b.to_async(Runtime::new().unwrap())
            .iter(|| async {
                let map = Arc::new(ParkingLotMutex::new(StdHashMap::<u64, u64>::with_capacity(
                    N,
                )));
                let mut handles = Vec::new();
                for _ in 0..16 {
                    // 16 threads
                    let m = map.clone();
                    handles.push(tokio::spawn(async move {
                        for _ in 0..(N / 16) {
                            let k = thread_rng().gen_range(0..N as u64);
                            m.lock().insert(k, k + 1);
                        }
                    }));
                }
                for h in handles {
                    h.await.unwrap();
                }
            })
    });

    group.bench_function("dashmap_multi_thread", |b| {
        b.to_async(Runtime::new().unwrap())
            .iter(|| async {
                let map = Arc::new(DashMap::<u64, u64>::with_capacity(N));
                let mut handles = Vec::new();
                for _ in 0..16 {
                    // 16 threads
                    let m = map.clone();
                    handles.push(tokio::spawn(async move {
                        for _ in 0..(N / 16) {
                            let k = thread_rng().gen_range(0..N as u64);
                            m.insert(k, k + 1);
                        }
                    }));
                }
                for h in handles {
                    h.await.unwrap();
                }
            })
    });

    group.bench_function("hashbrown_multi_thread_tokio", |b| {
        b.to_async(Runtime::new().unwrap())
            .iter(|| async {
                let map = Arc::new(TokioMutex::new(
                    HashbrownHashMap::<u64, u64>::with_capacity(N),
                ));
                let mut handles = Vec::new();
                for _ in 0..16 {
                    // 16 threads
                    let m = map.clone();
                    handles.push(tokio::spawn(async move {
                        for _ in 0..(N / 16) {
                            let k = thread_rng().gen_range(0..N as u64);
                            m.lock().await.insert(k, k + 1);
                        }
                    }));
                }
                for h in handles {
                    h.await.unwrap();
                }
            })
    });

    group.bench_function("std_hashmap_multi_thread_tokio", |b| {
        b.to_async(Runtime::new().unwrap())
            .iter(|| async {
                let map = Arc::new(TokioMutex::new(StdHashMap::<u64, u64>::with_capacity(N)));
                let mut handles = Vec::new();
                for _ in 0..16 {
                    // 16 threads
                    let m = map.clone();
                    handles.push(tokio::spawn(async move {
                        for _ in 0..(N / 16) {
                            let k = thread_rng().gen_range(0..N as u64);
                            m.lock().await.insert(k, k + 1);
                        }
                    }));
                }
                for h in handles {
                    h.await.unwrap();
                }
            })
    });

    group.finish();
}

criterion_group!(benches, bench_maps);
criterion_main!(benches);
