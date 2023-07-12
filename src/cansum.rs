use std::collections::HashMap;

use crate::memoize::memoize;

/**
 * Returns true if the target_sum can be generated from the numbers in the vector.
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

#[cfg(test)]
mod tests {
    use super::can_sum;
    use std::collections::HashMap;

    #[test]
    fn can_sum_seven() {
        let cache = &mut HashMap::new();
        let result = can_sum(cache, 7, &vec![2, 3]);
        assert!(result);
        let cache = &mut HashMap::new();
        let result = can_sum(cache, 7, &vec![5, 3, 4, 7]);
        assert!(result);
        let cache = &mut HashMap::new();
        let result = can_sum(cache, 7, &vec![2, 4]);
        assert!(!result);
        let cache = &mut HashMap::new();
        let result = can_sum(cache, 8, &vec![2, 3, 5]);
        assert!(result);
        let cache = &mut HashMap::new();
        let result = can_sum(cache, 1100, &vec![7, 14]);
        assert!(!result);
    }
}
