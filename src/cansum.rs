use std::collections::HashMap;

use crate::memoize::memoize;

/**
 * Returns true if the target_sum can be generated from the numbers in the vector using memoization.
 */
pub fn can_sum(cache: &mut HashMap<i32, bool>, target_sum: i32, numbers: &Vec<i32>) -> bool {
    let x = Some(target_sum);
    match x {
        Some(x) if x == 0 => return true,
        Some(x) if x < 0 => return false,
        _ => (),
    }

    for num in numbers {
        let remainder = target_sum - num;
        if memoize(cache, can_sum, remainder, numbers) {
            return true;
        }
    }
    false
}

/**
 * Returns true if the target_sum can be generated from the numbers in the vector using tabulation.
 */
pub fn can_sum_tab(target_sum: usize, numbers: &Vec<usize>) -> bool {
    let mut table: Vec<bool> = vec![false; target_sum + 1];
    table[0] = true;
    for i in 0..=target_sum {
        if table[i] {
            for num in numbers {
                if i + num <= target_sum {
                    table[i + num] = true;
                }
            }
        }
    }

    table[target_sum]
}

#[cfg(test)]
mod tests {

    use super::{can_sum, can_sum_tab};
    use std::collections::HashMap;

    #[test]
    fn can_sum_seven() {
        let cache = &mut HashMap::new();
        let result = can_sum(cache, 7, &vec![2, 3]);
        assert!(result, "Result: {}", result);
        let cache = &mut HashMap::new();
        let result = can_sum(cache, 7, &vec![5, 3, 4, 7]);
        assert!(result, "Result: {}", result);
        let cache = &mut HashMap::new();
        let result = can_sum(cache, 7, &vec![2, 4]);
        assert!(!result, "Result: {}", result);
        let cache = &mut HashMap::new();
        let result = can_sum(cache, 8, &vec![2, 3, 5]);
        assert!(result, "Result: {}", result);
        let cache = &mut HashMap::new();
        let result = can_sum(cache, 1100, &vec![7, 14]);
        assert!(!result, "Result: {}", result);
    }

    #[test]
    fn can_sum_tabulation_seven() {
        let result = can_sum_tab(7, &vec![2, 3]);
        assert!(result, "Result: {}", result);
        let result = can_sum_tab(7, &vec![5, 3, 4, 7]);
        assert!(result, "Result: {}", result);
        let result = can_sum_tab(7, &vec![2, 4]);
        assert!(!result, "Result: {}", result);
        let result = can_sum_tab(8, &vec![2, 3, 5]);
        assert!(result, "Result: {}", result);
        let result = can_sum_tab(1100, &vec![7, 14]);
        assert!(!result, "Result: {}", result);
    }
}
