use std::collections::HashMap;

pub fn how_can_sum(
    target_sum: i32,
    numbers: &Vec<i32>,
    cache: &mut HashMap<i32, Option<Vec<i32>>>,
) -> Option<Vec<i32>> {
    if target_sum == 0 {
        return Some(vec![]);
    }

    if target_sum < 0 {
        return None;
    }

    match cache.get(&target_sum).cloned() {
        Some(x) => x,
        None => {
            for number in numbers.iter() {
                let remainder = target_sum - number;
                let remainder_result: Option<Vec<i32>> = how_can_sum(remainder, numbers, cache);

                if let Some(mut result) = remainder_result {
                    result.push(*number);
                    cache.insert(target_sum, Some(result));
                    if let Some(x) = cache.get(&target_sum) {
                        return x.clone();
                    }
                }
            }
            cache.insert(target_sum, None);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::how_can_sum;
    use std::collections::HashMap;

    #[test]
    fn how_can_sum_seven() {
        let memo = &mut HashMap::new();
        let result = how_can_sum(7, &vec![2, 3], memo);
        let expected_result = Some(vec![3, 2, 2]);
        assert!(result.iter().zip(&expected_result).all(|(a, b)| a == b));
    }
    #[test]
    fn how_can_sum_three_hundred() {
        let memo = &mut HashMap::new();
        let result = how_can_sum(300, &vec![7, 14], memo);
        assert_eq!(result, None);
    }
}
