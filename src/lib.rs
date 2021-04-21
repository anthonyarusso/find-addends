use itertools::Itertools;

#[inline]
pub fn find_addends(sum: u32, addend_count: u32, values: &Vec<u32>) -> Vec<u32> {
    let mut smalls: Vec<u32> = Vec::with_capacity(values.len());
    let mut bigs: Vec<u32> = Vec::with_capacity(values.len());
    let mut perfect_count = 0;
    let average: f32 = sum as f32 / addend_count as f32;

    // sort values
    for entry in values.iter() {
        // first we test equality which will only be true if average
        // is a whole number and entry and 'addend_count' are factors of sum.
        if (*entry as f32) == average {
            // we need to consider cases such as p + b + s
            perfect_count += 1;
            
            if perfect_count == addend_count {
                // 'values' contained 'addend_count' amount of perfect portions
                // e.g. sum is 100 and values contained four '25's.
                return vec![average as u32; addend_count as usize]; 
            }
        } else if (*entry as f32) < average {
            smalls.push(*entry);
        } else {
            bigs.push(*entry);
        }
    }

    if perfect_count > 0 {
        // if there are perfects recurse without perfects to account for
        // "p + b + s" type scenarios. Effectively, we save this state as
        // "p + find_addends()" which ought to return the remaining 
        // "b + s" sum. values does not need perfects removed as they
        // cannot be counted twice.

        // if perfects exist then average is a whole number and can be
        // safely cast as u32
        let mut imperfect_values =
            find_addends(sum - perfect_count * average as u32, addend_count - perfect_count, values);

        // if "p + b + s" == sum
        if imperfect_values.iter().sum::<u32>() + (perfect_count * average as u32) == sum {
            let mut answer = vec![average as u32, perfect_count];
            answer.append(&mut imperfect_values);
            return answer;
        }
    } else {
        let mut small_count = 1;
        // We know either b or s is at most (parts - 1). Otherwise, if b or s
        // was equal to parts then they would be perfect. Call (parts - 1) "n".
        // Therefore the addends are between nb + 1s to 1b to ns.

        while small_count < addend_count {
            for small_combo in smalls.iter().copied().combinations(small_count as usize) {
                for mut big_combo in bigs.iter().copied().combinations((addend_count - small_count) as usize) {
                    if small_combo.iter().sum::<u32>() +
                        big_combo.iter().sum::<u32>() == sum {

                        let mut answer: Vec<u32> = small_combo;
                        answer.append(&mut big_combo);

                        return answer;
                    }
                }
            }
            small_count += 1;
        } // end while small_count
    }
    Vec::new()
}

pub fn use_combos(sum: u32, addend_count: u32, values: &Vec<u32>) -> Vec<u32> {
    for combo in values.iter().copied().combinations(addend_count as usize) {
        if combo.iter().sum::<u32>() == sum {
            return combo;
        }
    }
    Vec::new()
}
