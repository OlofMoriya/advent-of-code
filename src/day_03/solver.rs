use crate::input_helper;
use crate::vec_helper::find_duplicate;

use super::model::Rugsack;

fn char_to_u32(c:char) -> u32 {
    let n = c as u32;
    if n < 97 {
        return n - 38;
    }  else {
        return n - 96;
    }
}

pub fn solve() -> String {
    let sum:u32 = input_helper::read_input("input/22_03_test")
        .into_iter()
        .map(|r:Rugsack| r.find_duplicate())
        .flatten()
        .map(|c| char_to_u32(c))
        .sum();

    return format!("{:?}",sum)
}
pub fn solve_two() -> String {
    let x:Vec<char> = input_helper::read_input("input/22_03_test")
        .chunks_exact(3)
        .map(|r:&[Rugsack]| find_duplicate(r.iter().map(|v| (v.items.clone())).collect())[0])
        .collect();

    let sum:u32 = x.into_iter().map(|c| char_to_u32(c)).sum();
    return format!("{}", sum);
}

