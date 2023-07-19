use std::{collections::HashMap, mem};

use crate::memoize::memoize;

/**
 * Returns a vector of vectors of strings that add up to the target string.
 */
pub fn all_construct<'a>(
    cache: &mut HashMap<&'a str, Vec<Vec<&'a str>>>,
    target: &'a str,
    word_bank: &[&'a str],
) -> Vec<Vec<&'a str>> {
    if target.is_empty() {
        return vec![vec![]];
    }

    let mut result: Vec<Vec<&str>> = vec![];

    for word in word_bank.iter() {
        if let Some(suffix) = target.strip_prefix(word) {
            // let suffix = &target[word.len()..];
            let suffix_ways = memoize(cache, all_construct, suffix, word_bank);
            let target_ways = suffix_ways
                .iter()
                .map(|way| {
                    let mut way = way.clone();
                    way.insert(0, word);
                    way
                })
                .collect::<Vec<Vec<&str>>>();

            result.extend(target_ways);
        }
    }
    result
}

/**
* Returns a vector of vectors of strings that add up to the target string using tabulation. It has a poor time complexity.
*/
pub fn all_construct_tab<'a>(target: &'a str, word_bank: &[&'a str]) -> Vec<Vec<&'a str>> {
    let mut table: Vec<Vec<Vec<&str>>> = vec![vec![]; target.len() + 1];
    table[0] = vec![vec![]];

    for i in 0..target.len() {
        let current = mem::take(&mut table[i]);
        for word in word_bank {
            if target[i..].starts_with(word) {
                let mut new_combinations = mem::take(&mut table[i + word.len()]);
                current.iter().for_each(|iter| {
                    let mut iter = iter.clone();
                    iter.push(*word);
                    new_combinations.push(iter);
                });

                _ = mem::replace(&mut table[i + word.len()], new_combinations);
            }
        }
    }

    mem::take(&mut table[target.len()])
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::allconstruct::{all_construct, all_construct_tab};

    #[test]
    fn can_get_purple() {
        let cache = &mut HashMap::new();
        let result = all_construct(cache, "purple", &["purp", "p", "ur", "le", "purpl"]);
        let expected_result = vec![vec!["purp", "le"], vec!["p", "ur", "p", "le"]];
        for expected in expected_result {
            assert!(result.contains(&expected), "Result: {:?}", result);
        }
    }

    #[test]
    fn can_get_abcdef() {
        let cache = &mut HashMap::new();
        let result = all_construct(
            cache,
            "abcdef",
            &["ab", "abc", "cd", "def", "abcd", "ef", "c"],
        );
        let expected_result = vec![
            vec!["ab", "cd", "ef"],
            vec!["abc", "def"],
            vec!["abcd", "ef"],
        ];
        for expected in expected_result {
            assert!(result.contains(&expected), "Result: {:?}", result);
        }
    }

    #[test]
    fn cannot_get_long_string() {
        let cache = &mut HashMap::new();
        let result = all_construct(
            cache,
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaz",
            &["a", "aa", "aaa", "aaaa", "aaaaa"],
        );
        let expected_result: Vec<Vec<&str>> = vec![];
        for expected in expected_result {
            assert!(result.contains(&expected), "Result: {:?}", result);
        }
    }

    #[test]
    fn can_get_abcdef_tab() {
        let result = all_construct_tab("abcdef", &["ab", "abc", "cd", "def", "abcd", "ef", "c"]);
        let expected_result = vec![
            vec!["ab", "cd", "ef"],
            vec!["ab", "c", "def"],
            vec!["abc", "def"],
            vec!["abcd", "ef"],
        ];
        for expected in expected_result {
            assert!(result.contains(&expected), "Result: {:?}", result);
        }
    }

    #[test]
    fn can_get_long_string_tab() {
        let result = all_construct_tab(
            "aaaaaaaaaaaaaaaaaaaaz",
            &["a", "aa", "aaa", "aaaa", "aaaaa"],
        );
        let expected_result: Vec<Vec<&str>> = vec![];
        assert_eq!(result, expected_result, "Result: {:?}", result);
    }
}
