use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::env;
use std::time::Instant;

use revised::{find_addends, use_combos};

fn sum_as_f64(values: &Vec<f32>) -> f64 {
    let mut sum: f64 = 0.0;
    for value in values.iter() {
        sum += *value as f64;
    }
    sum
}

fn product_as_f64(values: &Vec<f32>) -> f64 {
    let mut product: f64 = 1.0;
    for value in &*values {
        product *= *value as f64;
    }
    product
}

fn main() -> io::Result<()> {
    // command-line arguments
    let file_name: String = env::args().nth(1).expect("Please provide a text file.");
    let arg2 = env::args().nth(2).expect("Please provide the desired sum.");
    let arg3 = env::args().nth(3).expect("Please provide the number of addends.");
    let arg4 = env::args().nth(4).unwrap_or("false".to_string());
    let sum: f32 = arg2.parse().expect("Please provide a whole number sum.");
    let parts: usize = arg3.parse().expect("Please provide a whole number of addends.");
    let test_both: bool = arg4.parse().expect("Please provide \"true\" or no argument.");

    // file io
    let file = File::open(&file_name)?;
    let mut reader = io::BufReader::new(file);
    let mut buf = String::new();
    let mut contents: Vec<f32> = Vec::with_capacity(1000);
    let mut line_number = 1;

    while reader.read_line(&mut buf)? > 0 {
        let line = buf.trim_end();

        if let Ok(x) = line.parse::<f32>() {
            contents.push(x);
        } else {
            println!("Error parsing value on line {}", line_number);
        };

        buf.clear();
        line_number += 1;
    } // end while read_line

    let mut fa_iters: u128 = 0;
    let fa_instant = Instant::now();
    let solution = find_addends(sum, parts, &contents, Some(&mut fa_iters));

    println!("find_addends completed in {} ms with {} iterations",
             fa_instant.elapsed().as_millis(),
             fa_iters);
    println!("solution set: {:?}, sum: {}, product: {}",
             solution,
             sum_as_f64(&solution),
             product_as_f64(&solution));

    if test_both {
        let mut uc_iters: u128 = 0;
        let uc_instant = Instant::now();
        let combo_solution = use_combos(sum, parts, &contents, Some(&mut uc_iters));

        println!("use_combos completed in {} ms with {} iterations",
                 uc_instant.elapsed().as_millis(),
                 uc_iters);
        println!("solution_set: {:?}, sum: {}, product: {}",
                 combo_solution,
                 sum_as_f64(&combo_solution),
                 product_as_f64(&combo_solution));
    }

    Ok(())
}
