use std::collections::HashMap;

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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::allconstruct::all_construct;

    #[test]
    fn can_get_purple() {
        let cache = &mut HashMap::new();
        let result = all_construct(cache, "purple", &["purp", "p", "ur", "le", "purpl"]);
        let expected_result = vec![vec!["purp", "le"], vec!["p", "ur", "p", "le"]];
        assert_eq!(result, expected_result, "Result: {:?}", result);
    }

    #[test]
    fn can_get_abcdef() {
        let cache = &mut HashMap::new();
        let result = all_construct(
            cache,
            "abcdef",
            &[
                "ab",
                "abc",
                "cd",
                "def",
                "abcd",
                "ef",
                "
            c",
            ],
        );
        let expected_result = vec![
            vec!["ab", "cd", "ef"],
            vec!["abc", "def"],
            vec!["abcd", "ef"],
        ];
        assert_eq!(result, expected_result, "Result: {:?}", result);
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
        assert_eq!(result, expected_result, "Result: {:?}", result);
    }
}
