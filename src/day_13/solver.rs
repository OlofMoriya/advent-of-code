use itertools::Itertools;

use crate::deserializable::Deserializable;

use super::model::{Part, Comp};

pub fn solve() -> String {
    let input = include_str!("../../input/22_13"); 
    let mut fine = vec!();
    input.split("\r\n\r\n")
        .enumerate()
        .for_each(|(i, l)| {
            println!("{}", l);
            let lines:Vec<Part> = l.lines().map(|li| Part::deserialize(li).ok().expect("First try")).collect();
            lines.iter().for_each(|l| println!("made l: {:?}", l));
            match lines[0].before(&lines[1]) {
                Comp::True =>{
                    println!("{:?}, is before {:?}",lines[0], lines[1]); 
                    fine.push(i+1);
                }
                Comp::False => {
                    println!("{:?}, is NOT before {:?}",lines[0], lines[1]); 
                },
                Comp::Inconclusive => {panic!("Should not be inconclusive")}
            }
    });

    return format!("indexes: {:?}", fine.iter().sum::<usize>());
}

pub fn solve_two() -> String {
    let input = include_str!("../../input/22_13"); 
    let mut parts:Vec<Part> = input.lines().filter(|l| l != &"").map(|l| {
            println!("{}", l);
            Part::deserialize(l).ok().expect("First try")
    }).collect();
    let part_two = Part::deserialize("[[2]]").ok().expect("");
    let part_six = Part::deserialize("[[6]]").ok().expect("");

    parts.push(part_two.clone());
    parts.push(part_six.clone());
    
    parts.sort_by(|a,b| match a.before(b) {
        Comp::True => std::cmp::Ordering::Less,
        Comp::False => std::cmp::Ordering::Greater,
        Comp::Inconclusive => std::cmp::Ordering::Equal,
    });

    let index_of_two = parts.clone().into_iter().find_position(|p| match p.before(&part_two){
        Comp::Inconclusive => true,
        _ => false
    });
    let index_of_six = parts.clone().into_iter().find_position(|p| match p.before(&part_six){
        Comp::Inconclusive => true,
        _ => false
    });

    println!("first: {:?}", parts.first());
    println!("last: {:?}", parts.last());

    println!("2:{:?} and 6:{:?}", index_of_two, index_of_six);
    return format!("indexes: {}", (index_of_two.unwrap().0 + 1) * (index_of_six.unwrap().0 + 1));
}


