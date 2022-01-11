use std::collections::HashMap;

/// Check if a combination of provided `numbers` can sum up to `target_sum`.
fn can_sum(target_sum: i64, numbers: &[i64], reuse: bool) -> bool {
    let mut memo: HashMap<i64, bool> = HashMap::new();

    return _can_sum(target_sum, numbers, reuse, &mut memo);
}

fn _can_sum(target_sum: i64, numbers: &[i64], reuse: bool,
            memo: &mut HashMap<i64, bool>) -> bool {

    if let Some(c) = memo.get(&target_sum) { return *c }
    if target_sum == 0 { return true }
    if target_sum < 0 { return false }

    for num in numbers {
        let remainder = target_sum - num;
        let can_sum_remainder = if reuse {
            _can_sum(remainder, numbers, reuse, memo)
        } else {
            _can_sum(remainder, &numbers[1..], reuse, memo)
        };

        if can_sum_remainder {
            memo.insert(target_sum, true);
            return true;
        }
    }

    memo.insert(target_sum, false);
    return false;
}

/// Return any combination of provided `numbers` that can sum up to
/// `target_sum`.
fn how_sum(target_sum: i64, numbers: &[i64], reuse: bool) -> Option<Vec<i64>> {
    let mut memo: HashMap<i64, Option<Vec<i64>>> = HashMap::new();

    _how_sum(target_sum, numbers, reuse, &mut memo)
}

fn _how_sum(target_sum: i64, numbers: &[i64], reuse: bool,
            memo: &mut HashMap<i64, Option<Vec<i64>>>) -> Option<Vec<i64>> {

    if memo.contains_key(&target_sum) {
        if let Some(cached_sum) = memo.get(&target_sum) {
            return cached_sum.clone();
        }
        return None;
    }
    if target_sum == 0 { return Some(vec![]) }
    if target_sum < 0 { return None }

    for num in numbers {
        let remainder = target_sum - num;
        let how_sum_partial = if reuse {
            _how_sum(remainder, numbers, reuse, memo)
        } else {
            _how_sum(remainder, &numbers[1..], reuse, memo)
        };

        if let Some(mut rem_sum) = how_sum_partial {
            rem_sum.push(*num);
            let rem_sum_clone = rem_sum.clone();
            memo.insert(target_sum, Some(rem_sum_clone));
            return Some(rem_sum);
        }
    }

    memo.insert(target_sum, None);
    None
}

/// Return any shortest length combination of provided `numbers` that can sum up
/// to `target_sum`.
fn best_sum(target_sum: i64, numbers: &[i64], reuse: bool) -> Option<Vec<i64>> {
    let mut memo: HashMap<i64, Option<Vec<i64>>> = HashMap::new();

    _best_sum(target_sum, numbers, reuse, &mut memo)
}

fn _best_sum(target_sum: i64, numbers: &[i64], reuse: bool,
             memo: &mut HashMap<i64, Option<Vec<i64>>>) -> Option<Vec<i64>> {

    if memo.contains_key(&target_sum) {
        if let Some(cached_sum) = memo.get(&target_sum) {
            return cached_sum.clone();
        }
        return None;
    }
    if target_sum == 0 { return Some(vec![]) }
    if target_sum < 0 { return None }

    let mut best: Option<Vec<i64>> = None;

    for num in numbers {
        let remainder = target_sum - num;
        let best_sum_partial = if reuse {
            _best_sum(remainder, numbers, reuse, memo)
        } else {
            _best_sum(remainder, &numbers[1..], reuse, memo)
        };

        if let Some(mut rem_sum) = best_sum_partial {
            rem_sum.push(*num);

            match best {
                Some(ref best_sum) => {
                    if best_sum.is_empty() || rem_sum.len() < best_sum.len() {
                        best = Some(rem_sum);
                    }
                },
                None => {
                    best = Some(rem_sum);
                },
            }
        }
    }

    memo.insert(target_sum, best.clone());
    best
}

/// Return all combination of provided `numbers` that sum up to `target_sum`.
fn all_sum(target_num: i64, numbers: &[i64], reuse: bool) -> Vec<Vec<i64>> {
    if target_num == 0 { return vec![vec![]] }
    if target_num < 0 { return vec![] }

    let mut results: Vec<Vec<i64>> = vec![];

    for num in numbers {
        let remainder = target_num - num;
        let rem_sum_list = if reuse {
            all_sum(remainder, numbers, reuse)
        } else {
            all_sum(remainder, &numbers[1..], reuse)
        };

        for mut rem_sum in rem_sum_list {
            rem_sum.push(*num);
            results.push(rem_sum);
        }
    }

    results
}

fn main() {
    println!("=== CAN SUM with reuse ===");
    println!("{}", can_sum(7, &[2, 3, 5, 7], true));
    println!("{}", can_sum(7, &[2, 4], true));

    println!("=== CAN SUM without reuse ===");
    println!("{}", can_sum(7, &[2, 3, 5, 7], false));
    println!("{}", can_sum(7, &[2, 4], false));

    println!("=== HOW SUM with reuse ===");
    println!("{:?}", how_sum(7, &[2, 3, 5, 7], true));
    println!("{:?}", how_sum(7, &[2, 4], true));

    println!("=== HOW SUM without reuse ===");
    println!("{:?}", how_sum(7, &[2, 3, 5, 7], false));
    println!("{:?}", how_sum(7, &[2, 4], false));

    println!("=== BEST SUM with reuse ===");
    println!("{:?}", best_sum(7, &[2, 3, 5, 7], true));
    println!("{:?}", best_sum(7, &[2, 4], true));

    println!("=== BEST SUM without reuse ===");
    println!("{:?}", best_sum(7, &[2, 3, 5, 7], false));
    println!("{:?}", best_sum(7, &[2, 4], false));

    println!("=== ALL SUM with reuse ===");
    println!("{:?}", all_sum(7, &[2, 3, 5, 7], true));
    println!("{:?}", all_sum(7, &[2, 4], true));

    println!("=== ALL SUM without reuse ===");
    println!("{:?}", all_sum(7, &[2, 3, 5, 7], false));
    println!("{:?}", all_sum(7, &[2, 4], false));
}
