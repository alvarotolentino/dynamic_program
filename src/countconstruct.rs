use std::collections::HashMap;

pub fn count_construct(
    target: &str,
    word_bank: &Vec<&str>,
    cache: &mut HashMap<String, i32>,
) -> i32 {
    if target.is_empty() {
        return 1;
    }

    let mut total_count = 0;

    match cache.get(target) {
        Some(x) => *x,
        None => {
            for word in word_bank.iter() {
                if target.starts_with(word) {
                    let remainder = &target[word.len()..target.len()];
                    let result = count_construct(remainder, word_bank, cache);
                    cache.insert(target.to_owned(), result);
                    total_count += cache.get(target).unwrap();
                }
            }
            cache.insert(target.to_owned(), total_count);
            total_count
        }
    }
}

#[cfg(test)]
mod tests {
    use super::count_construct;
    use std::collections::HashMap;

    #[test]
    fn count_construct_short_string() {
        let memo = &mut HashMap::new();
        let result = count_construct("purple", &vec!["purp", "p", "ur", "le", "purpl"], memo);
        assert_eq!(result, 2)
    }
    #[test]
    fn count_construct_long_string() {
        let memo = &mut HashMap::new();
        let result = count_construct(
            "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeef",
            &vec!["e", "ee", "eee", "eeee", "eeeee"],
            memo,
        );
        assert_eq!(result, 0)
    }
}
