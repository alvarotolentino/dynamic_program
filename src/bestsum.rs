use std::collections::HashMap;

pub fn best_sum(
    target_sum: i32,
    numbers: &Vec<i32>,
    memo: &mut HashMap<i32, Option<Vec<i32>>>,
) -> Option<Vec<i32>> {
    if let Some(x) = memo.get(&target_sum) {
        return x.clone();
    }

    if target_sum == 0 {
        return Some(vec![]);
    }
    if target_sum < 0 {
        return None;
    }

    let mut shortest_combination: Option<Vec<i32>> = None;

    for num in numbers.iter() {
        let remainder: i32 = target_sum - num;
        let remainder_combination = best_sum(remainder, numbers, memo);
        if let Some(mut combination) = remainder_combination {
            combination.push(*num);
            memo.insert(target_sum, Some(combination.clone()));
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

#[cfg(test)]
mod tests {
    use super::best_sum;
    use std::collections::HashMap;
    #[test]
    fn best_sum_seven() {
        let memo = &mut HashMap::new();
        let result = best_sum(7, &vec![5, 3, 4, 7], memo);
        let expected_result = Some(vec![7]);
        assert!(result.iter().zip(&expected_result).all(|(a, b)| a == b), "{:?} != {:?}", result, expected_result);
    }

    #[test]
    fn best_sum_one_hundred() {
        let memo = &mut HashMap::new();
        let result = best_sum(100, &vec![1, 2, 5, 25], memo);
        let expected_result = Some(vec![25, 25, 25, 25]);
        assert!(result.iter().zip(&expected_result).all(|(a, b)| a == b), "{:?} != {:?}", result, expected_result);
    }
}
