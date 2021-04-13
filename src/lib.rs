use itertools::Itertools;

#[inline]
pub fn find_addends(sum: u32, parts: u32, values: &Vec<u32>) -> Vec<u32> {
    let mut smalls: Vec<u32> = Vec::with_capacity(values.len());
    let mut bigs: Vec<u32> = Vec::with_capacity(values.len());
    let mut perfects = 0;
    let portion: f32 = sum as f32 / parts as f32;

    // sort values
    for x in values.iter() {
        // first we test equality which will only be true if portion
        // is a whole number and x and 'parts' are factors of sum.
        if (*x as f32) == portion {
            // we need to consider cases such as p + b + s
            perfects += 1;
            
            if perfects == parts {
                // 'values' contained 'parts' amount of perfect portions
                // e.g. sum is 100 and values contained four '25's.
                return vec![portion as u32; parts as usize]; 
            }
        } else if (*x as f32) < portion {
            smalls.push(*x);
        } else {
            bigs.push(*x);
        }
    }

    if perfects > 0 {
        // if there are perfects recurse without perfects to account for
        // "p + b + s" type scenarios. Effectively, we save this state as
        // "p + find_addends()" which ought to return the remaining 
        // "b + s" sum. values does not need perfects removed as they
        // cannot be counted twice.

        // if perfects exist then portion is a whole number and can be
        // safely cast as u32
        let mut imperfect_values =
            find_addends(sum - perfects * portion as u32, parts - perfects, values);

        // if "p + b + s" == sum
        if imperfect_values.iter().sum::<u32>() + (perfects * portion as u32) == sum {
            let mut answer = vec![portion as u32, perfects];
            answer.append(&mut imperfect_values);
            return answer;
        }
    } else {
        let mut small_count = 1;
        // We know either b or s is at most (parts - 1). Otherwise, if b or s
        // was equal to parts then they would be perfect. Call (parts - 1) "n".
        // Therefore the addends are between nb + 1s to 1b to ns.

        while small_count < parts {
            for small_combo in smalls.iter().copied().combinations(small_count as usize) {
                for mut big_combo in bigs.iter().copied().combinations((parts - small_count) as usize) {
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

pub fn use_combos(sum: u32, parts: u32, values: &Vec<u32>) -> Vec<u32> {
    for combo in values.iter().copied().combinations(parts as usize) {
        if combo.iter().sum::<u32>() == sum {
            return combo;
        }
    }
    Vec::new()
}
