
use crate::input_helper::read_input_with_none;

pub fn solve() -> String { 
    let max = read_input_with_none::<u32>("input/22_01_test")
        .group_by(|a,b| a.is_some() && b.is_some())
        .into_iter()
        .map(|g| g
             .iter()
             .filter_map(|i| i.as_ref())
             .sum())
        .fold(0, |a,b| a.max(b));
    return format!("The elf with the most calories is carrying {:?} calories.", max);
}

pub fn solve_two()-> String {
    let mut elves: Vec<u32> = read_input_with_none::<u32>("input/22_01_test")
        .group_by(|a,b| a.is_some() && b.is_some())
        .into_iter()
        .map(|g| g
             .iter()
             .filter_map(|i| i.as_ref())
             .sum())
        .collect();
    elves.sort();
    let cumulative_max: u32 = elves.iter().rev().take(3).into_iter().sum();
    return format!("The three elves with the most calories are carrying {:?} calories togther.",cumulative_max);
}

