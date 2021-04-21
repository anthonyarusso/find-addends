use itertools::Itertools;

#[inline]
pub fn sum_combo(values: &Vec<&&f32>) -> f32 {
    let mut sum = 0.0;

    for value in values {
        sum += **value;
    }
    sum
}

#[inline]
pub fn find_addends(sum: f32, addend_count: usize, values: &Vec<f32>) -> Vec<f32> {
    let mut bigs: Vec<&f32> = Vec::with_capacity(1000);
    let mut smalls: Vec<&f32> = Vec::with_capacity(1000);
    let mut perfects: usize = 0;
    let average: f32 = sum / (addend_count as f32);

    // sort the values
    for entry in values.iter() {
        if *entry > average {
            bigs.push(&entry);
        } else if *entry < average {
            smalls.push(&entry);
        } else {
            perfects += 1;

            if perfects == addend_count {
                // all perfects
                return vec![average; addend_count];
            }
        }
    }

    if perfects > 0 {
        println!("perfects found, time to recurse");
    } else {
        // number of small addends we consider
        for small_count in 1..addend_count {
            for small_combo in smalls.iter().combinations(small_count) {
                for big_combo in bigs.iter().combinations(addend_count - small_count) {
                    if sum_combo(&small_combo) + sum_combo(&big_combo) == sum {
                        println!("yay");
                    }
                }
            }
        }
    }

    // convert Vec<&f32> into Vec<f32> and return it
    return smalls.into_iter().copied().collect();
}

#[inline]
pub fn use_combos(sum: u32, addend_count: u32, values: &Vec<u32>) -> Vec<u32> {
    for combo in values.iter().copied().combinations(addend_count as usize) {
        if combo.iter().sum::<u32>() == sum {
            return combo;
        }
    }
    Vec::new()
}
