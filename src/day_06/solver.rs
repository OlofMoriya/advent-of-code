use itertools::Itertools;

use crate::vec_helper::filter_uniq;

pub fn solve() -> String {
    let window_size = 4;
    let chars: Vec<char> = include_str!("../../input/22_06")
        .chars()
        .collect();
    let first = chars
        .windows(window_size)
        .position(|x| {
            let v = x.clone().to_vec();
            filter_uniq(&v).len() == window_size
        });
    
    return format!("first marker at {}", first.expect("aoc will not lie") + window_size);
}

pub fn solve_two() -> String {
    let window_size = 14;
    let string = include_str!("../../input/22_06");
    let chars: Vec<char> = string.chars().collect();
    let first = chars
        .windows(window_size)
        .position(|x| {
        let v = x.clone().iter().collect::<Vec<&char>>();
        filter_uniq(&v).len() == window_size
    });
    
    return format!("first message at {}", first.expect("aoc will not lie") + window_size);
}

