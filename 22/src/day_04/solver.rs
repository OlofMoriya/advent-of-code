use crate::input_helper;

use super::model::Pair;

pub fn solve() -> String {
    let all:Vec<Pair> = input_helper::read_input("input/22_04");
    let count = all.iter().map(|p| p.is_completly_overlapped()).filter(|p| *p).collect::<Vec<bool>>().len();
    return format!("{} elfs have basically nothing to do", count);
}

pub fn solve_two() -> String {
    let all:Vec<Pair> = input_helper::read_input("input/22_04");
    let count = all.iter().map(|p| p.clone().has_overlap()).filter(|p| *p).collect::<Vec<bool>>().len();
    return format!("{} pairs have overlap", count);
}
