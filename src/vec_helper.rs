use std::collections::HashSet;

pub fn filter_uniq<T>(vec: &Vec<T>) -> Vec<T> where T: Eq + std::hash::Hash + Clone {
    vec.into_iter().cloned()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

pub fn find_duplicate<T>(vectors: Vec<Vec<T>>) -> Vec<T> where T: Eq + Clone + std::hash::Hash {
    let duplicates = vectors[0].to_vec();
    let folded = vectors
        .iter()
        .fold(duplicates, |acc, n| acc
              .iter()
              .filter(|c| n.contains(c))
              .cloned()
              .collect());

    let unique = filter_uniq(&folded);
    return unique;
}
