#![feature(portable_simd)]
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::{
    hint::black_box,
    simd::{cmp::SimdOrd as _, num::SimdUint as _, Simd},
};
const LANES: usize = 16;

fn find_min(arr: &[u32]) -> u32 {
    let mut min = u32::MAX;
    for &el in arr {
        if el < min {
            min = el;
        }
    }
    min
}

fn find_min_black_box(arr: &[u32]) -> u32 {
    let mut min = u32::MAX;
    for &el in arr {
        if el < black_box(min) {
            min = el;
        }
    }
    min
}

fn find_min_std(arr: &[u32]) -> u32 {
    *arr.iter().min().unwrap()
}

fn find_min_simd(arr: &[u32]) -> u32 {
    let lines = arr.len() / LANES;
    if lines == 0 {
        return *arr.iter().min().unwrap();
    }

    let mut min: Simd<u32, LANES> = Simd::from_slice(&arr[0..]);

    for i in 1..lines {
        let slice = Simd::from_slice(&arr[i * LANES..]);
        min = min.simd_min(slice);
    }

    let mut min = min.reduce_min();
    if arr.len() % LANES != 0 {
        for el in &arr[lines * LANES..] {
            if *el < min {
                min = *el;
            }
        }
    }
    min
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("min");

    for size in [1_000, 1_000_000, 10_000_000].iter() {
        let arr = r_vec(*size);

        group.bench_with_input(BenchmarkId::new("sequential", size), &arr, |b, arr| {
            b.iter(|| black_box(find_min(black_box(&arr))));
        });
        group.bench_with_input(
            BenchmarkId::new("sequential_with_black_box", size),
            &arr,
            |b, arr| {
                b.iter(|| black_box(find_min_black_box(black_box(&arr))));
            },
        );

        group.bench_with_input(BenchmarkId::new("simd", size), &arr, |b, arr| {
            b.iter(|| black_box(find_min_simd(black_box(&arr))));
        });

        group.bench_with_input(BenchmarkId::new("std", size), &arr, |b, arr| {
            b.iter(|| black_box(find_min_std(black_box(&arr))));
        });
    }
}

fn r_vec(size: usize) -> Vec<u32> {
    let mut v = Vec::with_capacity(size);
    for i in 0..size {
        v.push(i as u32);
    }
    v
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
