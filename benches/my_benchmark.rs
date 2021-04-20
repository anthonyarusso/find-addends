use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use revised::{find_addends, use_combos};

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use rand::Rng;
use rand::distributions::{Distribution, Uniform};

use std::time::Duration;

const SETS: u32 = 100;

fn gen_values(count: u32, max: u32, guarantee_solution: u32, file_path: String) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let range = Uniform::from(1..max);
    let mut values: Vec<u32> = Vec::with_capacity(count as usize);
    let mut values_string: String = String::new();

    for _ in 0..count {
        let random = range.sample(&mut rng);
        values.push(random);
        values_string = format!("{}{}\n", &values_string, random.to_string());
    }

    // Write to file
    let path = Path::new(&file_path);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(values_string.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why),
        Ok(_) => {},
    }

    values
}

fn read_values(path: String) -> Vec<u32> {
    // file io
    let file = File::open(path).unwrap();
    let mut reader = io::BufReader::new(file);
    let mut buf = String::new();
    let mut contents: Vec<u32> = Vec::with_capacity(200);
    let mut line_number = 1;

    while reader.read_line(&mut buf).unwrap() > 0 {
        let line = buf.trim_end();

        if let Ok(x) = line.parse::<u32>() {
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
    // let mut sets: Vec<u32> = Vec::new(); 

    let mut aoc = c.benchmark_group("Advent of Code Data");
    aoc.measurement_time(Duration::from_millis(9000));
    let aoc_data = read_values("./data/input.txt".to_string());
    aoc.bench_function("use_combos", |b| b.iter(|| use_combos(black_box(2020), black_box(3), &aoc_data)));
    aoc.bench_function("find_addends", |b| b.iter(|| find_addends(black_box(2020), black_box(3), &aoc_data)));
    aoc.finish();

    let mut five = c.benchmark_group("five_addends");
    five.measurement_time(Duration::from_millis(9000));
    let five_data = read_values("./data/five_addends_2020_sum.txt".to_string());
    five.bench_function("use_combos", |b| b.iter(|| use_combos(black_box(2020), black_box(5), &five_data)));
    five.bench_function("find_addends", |b| b.iter(|| find_addends(black_box(2020), black_box(5), &five_data)));
    five.finish();

    /*
    let mut rand_100_group = c.benchmark_group(format!("{} Sets of 100 Random Data Entries", SETS));
    for i in 0..20 {
        let set = read_values(format!("./data/random/100/{}.txt", i));
        rand_100_group.bench_function(
            BenchmarkId::new("use_combos", i), |b| b.iter(|| use_combos(black_box(2020), black_box(3), &set)));
        rand_100_group.bench_function(
            BenchmarkId::new("find_addends", i), |b| b.iter(|| find_addends(black_box(2020), black_box(3), &set)));
    }
    rand_100_group.finish();

    let mut rand_200_group = c.benchmark_group(format!("{} Sets of 200 Random Data Entries", SETS));
    for i in 0..20 {
        let set = read_values(format!("./data/random/200/{}.txt", i));
        rand_200_group.bench_function(
            BenchmarkId::new("use_combos", i), |b| b.iter(|| use_combos(black_box(2020), black_box(3), &set)));
        rand_200_group.bench_function(
            BenchmarkId::new("find_addends", i), |b| b.iter(|| find_addends(black_box(2020), black_box(3), &set)));
    }
    rand_200_group.finish();

    let mut rand_1000_group = c.benchmark_group(format!("{} Sets of 1000 Random Data Entries", SETS));
    for i in 0..20 {
        let set = read_values(format!("./data/random/200/{}.txt", i));
        rand_1000_group.bench_function(
            BenchmarkId::new("use_combos", i), |b| b.iter(|| use_combos(black_box(2020), black_box(3), &set)));
        rand_1000_group.bench_function(
            BenchmarkId::new("find_addends", i), |b| b.iter(|| find_addends(black_box(2020), black_box(3), &set)));
    }
    rand_1000_group.finish();
    */
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
