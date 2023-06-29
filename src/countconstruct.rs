use std::collections::HashMap;

pub fn count_construct(target: &str, word_bank: &Vec<&str>, memo: &mut HashMap<String, i32>) -> i32 {
    if let Some(x) = memo.get(target) {
        return *x;
    }
    if target.is_empty() {
        return 1;
    }

    let mut total_count = 0;

    for word in word_bank.iter() {
        if target.starts_with(word) {
            let remainder = &target[word.len()..target.len()];
            let result = count_construct(remainder, word_bank, memo);
            memo.insert(target.to_owned(), result);
            total_count += memo.get(target).unwrap();
        }
    }

    total_count
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
