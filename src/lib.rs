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
pub fn find_addends(sum: f32, addend_count: usize, values: &Vec<f32>, iterations: Option<&mut u128>) -> Vec<f32> {
    let mut bigs: Vec<&f32> = Vec::with_capacity(1000);
    let mut smalls: Vec<&f32> = Vec::with_capacity(1000);
    // whether we push perfects to `bigs` (true) or `smalls` (false)
    let mut p_to_b: bool = true;
    let average: f32 = sum / (addend_count as f32);

    // debug counter
    let mut iter_count: u128 = 0;

    // sort the values
    for entry in values.into_iter() {
        if *entry > average {
            bigs.push(&entry);
        } else if *entry < average {
            smalls.push(&entry);
        } else {
            if p_to_b {
                bigs.push(&entry);
            } else {
                smalls.push(&entry);
            }
            p_to_b = !p_to_b;
        }
    } // end 'for entry'

    for small_count in 1..addend_count {
        for small_combo in smalls.iter().combinations(small_count) {
            for mut big_combo in bigs.iter().combinations(addend_count - small_count) {
                iter_count += 1;
                
                if sum_combo(&small_combo) + sum_combo(&big_combo) == sum {
                    let mut solution: Vec<&&f32> = small_combo;
                    solution.append(&mut big_combo);

                    // 'return' iterations via reference
                    if let Some(iterations) = iterations {
                        *iterations = iter_count;
                    }

                    // convert Vec<&&f32> into Vec<f32> and return it
                    return solution.into_iter().map(|&&x| x).collect();
                }
            }
        }
    } // end 'for small_count'

    Vec::new()
}

#[inline]
pub fn use_combos(sum: f32, addend_count: usize, values: &Vec<f32>, iterations: Option<&mut u128>) -> Vec<f32> {
    // debug counter
    let mut iter_count: u128 = 0;

    for combo in values.iter().copied().combinations(addend_count) {
        iter_count += 1;

        if combo.iter().sum::<f32>() == sum {
            if let Some(iterations) = iterations {
                *iterations = iter_count;
            }

            return combo;
        }
    }
    Vec::new()
}
