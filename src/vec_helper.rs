use std::collections::HashSet;

pub fn filter_uniq<T>(vec: Vec<T>) -> Vec<T> where T: Eq + std::hash::Hash {
    vec.into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

