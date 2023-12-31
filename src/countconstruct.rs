use std::collections::HashMap;

use crate::memoize::memoize;

/**
 * Returns the number of ways the target can be generated from the words in the word_bank using memoization.
 */
pub fn count_construct<'a>(
    cache: &mut HashMap<&'a str, usize>,
    target: &'a str,
    word_bank: &Vec<&'a str>,
) -> usize {
    if target.is_empty() {
        return 1;
    }

    let mut total_count = 0;

    for word in word_bank.iter() {
        if target.starts_with(word) {
            let remainder = &target[word.len()..target.len()];
            let result = memoize(cache, count_construct, remainder, word_bank);
            total_count += result;
        }
    }
    total_count
}

/**
 * Returns the number of ways the target can be generated from the words in the word_bank using tabulation.
 */
pub fn count_construct_tab(target: &str, word_bank: &Vec<&str>) -> usize {
    let mut table = vec![0; target.len() + 1];
    table[0] = 1;

    for i in 0..target.len() {
        if table[i] > 0 {
            for word in word_bank {
                if target[i..].starts_with(word) {
                    table[i + word.len()] += table[i];
                }
            }
        }
    }
    table[target.len()]
}

#[cfg(test)]
mod tests {
    use super::{count_construct, count_construct_tab};
    use std::collections::HashMap;

    #[test]
    fn count_construct_short_string() {
        let cache = &mut HashMap::new();
        let result = count_construct(cache, "purple", &vec!["purp", "p", "ur", "le", "purpl"]);
        assert_eq!(result, 2, "result: {}", result);
    }
    #[test]
    fn count_construct_long_string() {
        let cache = &mut HashMap::new();
        let result = count_construct(
            cache,
            "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeef",
            &vec!["e", "ee", "eee", "eeee", "eeeee"],
        );
        assert_eq!(result, 0, "result: {}", result)
    }

    #[test]
    fn count_construct_tab_short_string() {
        let result = count_construct_tab("purple", &vec!["purp", "p", "ur", "le", "purpl"]);
        assert_eq!(result, 2, "result: {}", result);
    }

    #[test]
    fn count_construct_tab_long_string() {
        let result = count_construct_tab(
            "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeef",
            &vec!["e", "ee", "eee", "eeee", "eeeee", "eeeeee"],
        );
        assert_eq!(result, 0, "result: {}", result)
    }
}
