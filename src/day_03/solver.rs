use std::collections::HashSet;

use crate::input_helper;

use super::model::Rugsack;

fn filter_uniq(vec: Vec<char>) -> Vec<char> {
    vec.into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

fn char_to_u32(c:char) -> u32 {
    let n = c as u32;
    if n < 97 {
        return n - 38;
    }  else {
        return n - 96;
    }
}

pub fn find_duplicate(a: &Vec<char>, b: &Vec<char>, c: &Vec<char>) -> Vec<char> {
    let mut duplicates = vec!();
    a .iter()
        .for_each(|char| 
                  if b.contains(char) && c.contains(char){
                        duplicates.push(char.clone());
            }
        );

    let unique = filter_uniq(duplicates);
    return unique;
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
        .map(|r:&[Rugsack]| find_duplicate(&r[0].items, &r[1].items, &r[2].items)[0])
        .collect();

    let sum:u32 = x.into_iter().map(|c| char_to_u32(c)).sum();
    return format!("{}", sum);
}

