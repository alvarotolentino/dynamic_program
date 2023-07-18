use std::collections::HashMap;

use crate::memoize::memoize;

/**
 * Returns a vector of numbers that add up to the target_sum.
 */
pub fn how_can_sum(
    cache: &mut HashMap<i32, Option<Vec<i32>>>,
    target_sum: i32,
    numbers: &Vec<i32>,
) -> Option<Vec<i32>> {
    if target_sum == 0 {
        return Some(vec![]);
    }

    if target_sum < 0 {
        return None;
    }

    for number in numbers.iter() {
        let remainder = target_sum - number;
        let remainder_result: Option<Vec<i32>> = memoize(cache, how_can_sum, remainder, numbers);
        if let Some(mut result) = remainder_result {
            result.push(*number);

            if let Some(x) = cache.get(&target_sum) {
                return x.clone();
            }
        }
    }
    None
}

/**
 * Returns a vector of numbers that add up to the target_sum.
 */
pub fn how_can_sum_tab<'a>(target_sum: usize, numbers: &Vec<usize>) -> Option<Vec<usize>> {
    let mut table: Vec<Option<Vec<usize>>> = vec![None; target_sum + 1];
    table[0] = Some(vec![]);
    for i in 0..=target_sum {
        if let Some(x) = table[i].clone() {
            for num in numbers {
                if i + num <= target_sum {
                    let mut new_vec = x.clone();
                    new_vec.push(*num);
                    table[i + num] = Some(new_vec);
                }
            }
        }
    }

    table[target_sum].clone()
}

#[cfg(test)]
mod tests {

    use super::{how_can_sum, how_can_sum_tab};
    use std::collections::HashMap;

    #[test]
    fn how_can_sum_seven() {
        let cache = &mut HashMap::new();
        let result = how_can_sum(cache, 7, &vec![2, 3]);
        let expected_result = Some(vec![3, 2, 2]);
        assert!(
            result.iter().zip(&expected_result).all(|(a, b)| a == b),
            "Result: {:?}",
            result
        );
    }

    #[test]
    fn how_can_sum_three_hundred() {
        let cache = &mut HashMap::new();
        let result = how_can_sum(cache, 300, &vec![7, 14]);
        assert_eq!(result, None, "Result: {:?}", result);
    }

    #[test]
    fn how_can_sum_seven_tab() {
        let result = how_can_sum_tab(7, &vec![2, 3]);
        let expected_result = Some(vec![3, 2, 2]);
        assert!(
            result.iter().zip(&expected_result).all(|(a, b)| a == b),
            "Result: {:?}",
            result
        );
    }

    #[test]
    fn how_can_sum_three_hundred_tab() {
        let result = how_can_sum_tab(300, &vec![7, 14]);
        assert_eq!(result, None, "Result: {:?}", result);
    }
}
