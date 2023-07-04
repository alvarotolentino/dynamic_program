use std::collections::HashMap;

pub fn can_sum(target_sum: i32, numbers: &Vec<i32>, cache: &mut HashMap<i32, bool>) -> bool {
    let x = Some(target_sum);
    match x {
        Some(x) if x == 0 => return true,
        Some(x) if x < 0 => return false,
        _ => (),
    }

    match cache.get(&target_sum) {
        Some(x) => *x,
        None => {
            for num in numbers {
                let remainder = target_sum - num;
                if can_sum(remainder, numbers, cache) {
                    cache.insert(target_sum, true);
                    return true;
                }
            }
            cache.insert(target_sum, false);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::can_sum;
    use std::collections::HashMap;

    #[test]
    fn can_sum_seven() {
        let memo = &mut HashMap::new();
        let result = can_sum(7, &vec![2, 3], memo);
        assert!(result);
        let memo = &mut HashMap::new();
        let result = can_sum(7, &vec![5, 3, 4, 7], memo);
        assert!(result);
        let memo = &mut HashMap::new();
        let result = can_sum(7, &vec![2, 4], memo);
        assert!(!result);
        let memo = &mut HashMap::new();
        let result = can_sum(8, &vec![2, 3, 5], memo);
        assert!(result);
        let memo = &mut HashMap::new();
        let result = can_sum(1100, &vec![7, 14], memo);
        assert!(!result);
    }
}
