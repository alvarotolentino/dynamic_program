use std::{collections::HashMap, hash::Hash};

pub fn memoize<A, B, R, F>(cache: &mut HashMap<A, R>, func: F, arg1: A, arg2: B) -> R
where
    A: Eq + Hash + Clone,
    B: Eq + Hash + Clone,
    R: Clone,
    F: Fn(&mut HashMap<A, R>, A, B) -> R,
{
    match cache.get(&arg1).cloned() {
        Some(result) => result,
        None => {
            let result = func(cache, arg1.clone(), arg2);
            cache.insert(arg1, result.clone());
            result
        }
    }
}
