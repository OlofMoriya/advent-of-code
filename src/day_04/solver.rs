use crate::input_helper;

use super::model::Pair;

pub fn solve() -> String {
    let all:Vec<Pair> = input_helper::read_input("input/22_04");
    let contained:Vec<bool> = all.iter().map(|p| p.is_completly_overlapped()).filter(|p| *p).collect();
    let count = contained.len();
    return format!("{} elfs has basically nothing to do", count);
}

pub fn solve_two() -> String {
    let all:Vec<Pair> = input_helper::read_input("input/22_04");
    let has_overlap:Vec<bool> = all.iter().map(|p| p.clone().has_overlap()).filter(|p| *p).collect();

    let count = has_overlap.len();
    return format!("{} pairs has overlap", count);
}
