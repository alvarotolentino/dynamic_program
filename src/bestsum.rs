use std::collections::HashMap;

use crate::memoize::memoize;

/**
 * Return the shortest combination of numbers that add up to exactly the target sum using memoization.
 */
pub fn best_sum(
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

    let mut shortest_combination: Option<Vec<i32>> = None;

    for num in numbers.iter() {
        let remainder: i32 = target_sum - num;
        let remainder_combination = memoize(cache, best_sum, remainder, numbers);
        if let Some(mut combination) = remainder_combination {
            combination.push(*num);
            match shortest_combination {
                Some(shortest) if combination.len() < shortest.len() => {
                    shortest_combination = Some(combination);
                }
                None => {
                    shortest_combination = Some(combination);
                }
                _ => {}
            }
        }
    }
    shortest_combination
}

/**
 * Return the shortest combination of numbers that add up to exactly the target sum using tabulation.
 */
pub fn best_sum_tab(target_sum: usize, numbers: &Vec<usize>) -> Option<Vec<usize>> {
    let mut table = vec![None; target_sum + 1];
    table[0] = Some(vec![]);

    for i in 0..target_sum {
        if let Some(x) = table[i].clone() {
            for num in numbers {
                if i + num <= target_sum {
                    let mut new_vec = x.clone();
                    new_vec.push(*num);
                    
                    match table[i + num].clone() {
                        Some(vec) if vec.len() > new_vec.len() => {
                            table[i + num] = Some(new_vec);
                        }
                        None => {
                            table[i + num] = Some(new_vec);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    table[target_sum].clone()
}

#[cfg(test)]
mod tests {
    use super::{best_sum, best_sum_tab};
    use std::collections::HashMap;
    #[test]
    fn best_sum_seven() {
        let cache = &mut HashMap::new();
        let result = best_sum(cache, 7, &vec![5, 2, 3, 4, 7]);
        let expected_result = Some(vec![7]);
        assert!(
            result.iter().zip(&expected_result).all(|(a, b)| a == b),
            "{:?} != {:?}",
            result,
            expected_result
        );
    }

    #[test]
    fn best_sum_one_hundred() {
        let cache = &mut HashMap::new();
        let result = best_sum(cache, 100, &vec![1, 2, 5, 25]);
        let expected_result = Some(vec![25, 25, 25, 25]);
        assert!(
            result.iter().zip(&expected_result).all(|(a, b)| a == b),
            "{:?} != {:?}",
            result,
            expected_result
        );
    }

    #[test]
    fn best_sum_tabulation_seven() {
        let result = best_sum_tab(7, &vec![5, 2, 3, 4, 7]);
        let expected_result = Some(vec![7]);
        assert!(
            result.iter().zip(&expected_result).all(|(a, b)| a == b),
            "{:?} != {:?}",
            result,
            expected_result
        );
    }

    #[test]
    fn best_sum_tabulation_one_hundred() {
        let result = best_sum_tab(100, &vec![1, 2, 5, 25, 100]);
        let expected_result = Some(vec![100]);
        assert!(
            result.iter().zip(&expected_result).all(|(a, b)| a == b),
            "{:?} != {:?}",
            result,
            expected_result
        );
    }
}
