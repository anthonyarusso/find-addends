use criterion::{black_box, criterion_group, criterion_main, Criterion};
use revised::{find_addends, use_combos};

use std::fs::File;
use std::io;
use std::io::prelude::*;

use std::time::Duration;

fn read_values(path: String) -> Vec<f32> {
    // file io
    let file = File::open(path).unwrap();
    let mut reader = io::BufReader::new(file);
    let mut buf = String::new();
    let mut contents: Vec<f32> = Vec::with_capacity(1000);
    let mut line_number = 1;

    while reader.read_line(&mut buf).unwrap() > 0 {
        let line = buf.trim_end();

        if let Ok(x) = line.parse::<f32>() {
            contents.push(x);
        } else {
            println!("Error parsing value on line {}", line_number);
        };

        buf.clear();
        line_number += 1;
    } // end while read_line

    contents
}

pub fn benchmark(c: &mut Criterion) {
    // initialize our allocated vector
    // let mut sets: Vec<f32> = Vec::new(); 

    let mut aoc = c.benchmark_group("Advent of Code Data");
    aoc.measurement_time(Duration::from_millis(10500));
    let aoc_data = read_values("./data/input.txt".to_string());
    aoc.bench_function("use_combos", |b| b.iter(|| use_combos(black_box(2020.0), black_box(3), &aoc_data, None)));
    aoc.bench_function("find_addends", |b| b.iter(|| find_addends(black_box(2020.0), black_box(3), &aoc_data, None)));
    aoc.finish();

    let mut five = c.benchmark_group("five_addends");
    five.sample_size(50).measurement_time(Duration::from_secs(80));
    let five_data = read_values("./data/five_addends_2020_sum.txt".to_string());
    five.bench_function("use_combos", |b| b.iter(|| use_combos(black_box(2020.0), black_box(5), &five_data, None)));
    five.bench_function("find_addends", |b| b.iter(|| find_addends(black_box(2020.0), black_box(5), &five_data, None)));
    five.finish();

    let mut test_two = c.benchmark_group("test_two");
    test_two.sample_size(20).measurement_time(Duration::from_secs(500));
    let two_data = read_values("./data/2.txt".to_string());
    test_two.bench_function("use_combos", |b| b.iter(|| use_combos(black_box(2020.0), black_box(5), &two_data, None)));
    test_two.bench_function("find_addends", |b| b.iter(|| find_addends(black_box(2020.0), black_box(5), &two_data, None)));
    test_two.finish();

    let mut four_perfects = c.benchmark_group("four_perfects");
    four_perfects.sample_size(10).measurement_time(Duration::from_secs(60));
    let four_perfects_data = read_values("./data/four_perfects_2020_sum.txt".to_string());
    four_perfects.bench_function("use_combos", |b| b.iter(|| use_combos(black_box(2020.0), black_box(5), &four_perfects_data, None)));
    four_perfects.bench_function("find_addends", |b| b.iter(|| find_addends(black_box(2020.0), black_box(5), &four_perfects_data, None)));
    four_perfects.finish();

    let mut one_big = c.benchmark_group("one_big");
    one_big.sample_size(10).measurement_time(Duration::from_secs(80));
    let one_big_data = read_values("./data/five_addends_1_big_2020_sum.txt".to_string());
    one_big.bench_function("use_combos", |b| b.iter(|| use_combos(black_box(2020.0), black_box(5), &one_big_data, None)));
    one_big.bench_function("find_addends", |b| b.iter(|| find_addends(black_box(2020.0), black_box(5), &one_big_data, None)));
    one_big.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
