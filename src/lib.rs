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
    // whether we push perfects to `bigs` (true) or `smalls` (false)
    let mut p_to_b: bool = true;
    let mut perfects: usize = 0;
    let average: f32 = sum / (addend_count as f32);

    // sort the values
    for entry in values.iter() {
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
            perfects += 1;

            if perfects == addend_count {
                // all perfects
                return vec![average; addend_count];
            }
        }
    } // end 'for entry'

    for small_count in 1..addend_count {
        for small_combo in smalls.iter().combinations(small_count) {
            for mut big_combo in bigs.iter().combinations(addend_count - small_count) {
                if sum_combo(&small_combo) + sum_combo(&big_combo) == sum {
                    let mut solution: Vec<&&f32> = small_combo;
                    solution.append(&mut big_combo);

                    // convert Vec<&&f32> into Vec<f32> and return it
                    return solution.into_iter().map(|&&x| x).collect();
                }
            }
        }
    } // end 'for small_count'

    Vec::new()
}

#[inline]
pub fn use_combos(sum: f32, addend_count: usize, values: &Vec<f32>) -> Vec<f32> {
    for combo in values.iter().copied().combinations(addend_count) {
        if combo.iter().sum::<f32>() == sum {
            return combo;
        }
    }
    Vec::new()
}
