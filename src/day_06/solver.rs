use itertools::Itertools;

use crate::vec_helper::filter_uniq;

pub fn solve() -> String {
    let chars: Vec<char> = include_str!("../../input/22_06")
        .chars()
        .collect();
    let first = chars
        .windows(4)
        .enumerate()
        .find_or_first(|x| {
            let v = x.1.clone().to_vec();
            filter_uniq(&v).len() == 4
    });
    
    return format!("first marker at {}", first.expect("aoc will not lie").0 + 4);
}


pub fn solve_two() -> String {
    let string = include_str!("../../input/22_06");
    let chars: Vec<char> = string.chars().collect();
    let first = chars.windows(14).enumerate().find_or_first(|x| {
        let v = x.1.clone().iter().collect::<Vec<&char>>();
        filter_uniq(&v).len() == 14
    });
    
    return format!("first message at {}", first.expect("aoc will not lie").0 + 14);
}

