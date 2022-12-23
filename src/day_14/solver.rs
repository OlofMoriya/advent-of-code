use std::{cmp::Ordering, collections::{hash_map, HashMap}};

use itertools::Itertools;

use crate::input_helper;

pub fn solve() -> String {
    /*let input = include_str!("../../input/22_14");
    let map:Vec<(usize, usize)> = input.lines().map(|l| l.split(" -> ").tuple_windows().map(|(a,b)| {
        let (a_x, a_y) = a
            .split_once(",")
            .map(|(a_x, a_y)|  (a_x.parse::<usize>().expect("number"), 
                               a_y.parse::<usize>().expect("number"))
                 )
                 .expect("only valid data");
        let (b_x, b_y) = b.split_once(",")
            .map(|(b_x, b_y)|  (b_x.parse::<usize>().expect("number"), 
                               b_y.parse::<usize>().expect("number"))
                 )
            .expect("only valid data");

        match a_x {
            _ if a_x == b_x => {
                return (a_y.min(b_y)..=a_y.max(b_y)).into_iter().map(|i| (a_x, i)).collect::<Vec<(usize, usize)>>()
            },
            _ => {
                return (a_x.min(b_x)..=a_x.max(b_x)).into_iter().map(|i| (i, a_y)).collect::<Vec<(usize, usize)>>()
            }
 
        }
    })).flatten().flatten().collect();

    let min_x = map.iter().min_by(|a,b| match a.0 {
        _ if a.0 == b.0 => Ordering::Equal,
        _ if a.0 > b.0 => Ordering::Greater,
        _ if a.0 < b.0 => Ordering::Less,
        _ => unreachable!("")
    }).expect("non empty");
    let min_y = map.iter().min_by(|a,b| match a.1 {
        _ if a.1 == b.1 => Ordering::Equal,
        _ if a.1 > b.1 => Ordering::Greater,
        _ if a.1 < b.1 => Ordering::Less,
        _ => unreachable!("")
    }).expect("non empty");
    let max_x = map.iter().max_by(|a,b| match a.0 {
        _ if a.0 == b.0 => Ordering::Equal,
        _ if a.0 > b.0 => Ordering::Greater,
        _ if a.0 < b.0 => Ordering::Less,
        _ => unreachable!("")
    }).expect("non empty");
    let max_y = map.iter().max_by(|a,b| match a.1 {
        _ if a.1 == b.1 => Ordering::Equal,
        _ if a.1 > b.1 => Ordering::Greater,
        _ if a.1 < b.1 => Ordering::Less,
        _ => unreachable!("")
    }).expect("non empty");
    //println!("min {},{} max {},{}", min_x.0, min_y.1, max_x.0, max_y.1);

/*
    for row in 0..=max_y.1 {
        println!();

        for column in min_x.0..=max_x.0 {
            if row == 0 && column == 500 {
                print!("+");
            }
            if map.iter().any(|p| p == &(column, row)) {
                print!("#");
            } else {
                print!(".");
            }
        }  
    }

    println!();
*/
    let mut map = map.clone();
    let mut stable = 0;
    let mut became_stable = true;
    while became_stable {
        became_stable = false;
        let mut sand = (500, 1);
        while sand.1 <= max_y.1 + 3 && !became_stable {
           if !map.iter().any(|p| p == &(sand.0, sand.1 + 1)) {
               sand = (sand.0, sand.1 + 1); 
           } else if !map.iter().any(|p| p == &(sand.0 - 1, sand.1 + 1)) {
               sand = (sand.0 - 1, sand.1 + 1); 
           } else if !map.iter().any(|p| p == &(sand.0 + 1, sand.1 + 1)) {
               sand = (sand.0 + 1, sand.1 + 1); 
           } else {
               //became stable 
               map.push(sand);
               stable += 1;
               became_stable = true;
               println!("sand became stable at {:?}", sand);
           }
        } 
    }

    return format!("There are {} sand in the cave", stable);*/
        return "for now".to_string();
}

pub fn solve_two() -> String {
    let input = include_str!("../../input/22_14");
    let map:Vec<(usize, usize)> = input.lines().map(|l| l.split(" -> ").tuple_windows().map(|(a,b)| {
        let (a_x, a_y) = a
            .split_once(",")
            .map(|(a_x, a_y)|  (a_x.parse::<usize>().expect("number"), 
                               a_y.parse::<usize>().expect("number"))
                 )
                 .expect("only valid data");
        let (b_x, b_y) = b.split_once(",")
            .map(|(b_x, b_y)|  (b_x.parse::<usize>().expect("number"), 
                               b_y.parse::<usize>().expect("number"))
                 )
            .expect("only valid data");

        match a_x {
            _ if a_x == b_x => {
                return (a_y.min(b_y)..=a_y.max(b_y)).into_iter().map(|i| (a_x, i)).collect::<Vec<(usize, usize)>>()
            },
            _ => {
                return (a_x.min(b_x)..=a_x.max(b_x)).into_iter().map(|i| (i, a_y)).collect::<Vec<(usize, usize)>>()
            }
 
        }
    })).flatten().flatten().collect();

    let min_x = map.iter().min_by(|a,b| match a.0 {
        _ if a.0 == b.0 => Ordering::Equal,
        _ if a.0 > b.0 => Ordering::Greater,
        _ if a.0 < b.0 => Ordering::Less,
        _ => unreachable!("")
    }).expect("non empty");
    let min_y = map.iter().min_by(|a,b| match a.1 {
        _ if a.1 == b.1 => Ordering::Equal,
        _ if a.1 > b.1 => Ordering::Greater,
        _ if a.1 < b.1 => Ordering::Less,
        _ => unreachable!("")
    }).expect("non empty");
    let max_x = map.iter().max_by(|a,b| match a.0 {
        _ if a.0 == b.0 => Ordering::Equal,
        _ if a.0 > b.0 => Ordering::Greater,
        _ if a.0 < b.0 => Ordering::Less,
        _ => unreachable!("")
    }).expect("non empty");
    let max_y = map.iter().max_by(|a,b| match a.1 {
        _ if a.1 == b.1 => Ordering::Equal,
        _ if a.1 > b.1 => Ordering::Greater,
        _ if a.1 < b.1 => Ordering::Less,
        _ => unreachable!("")
    }).expect("non empty");
    //println!("min {},{} max {},{}", min_x.0, min_y.1, max_x.0, max_y.1);

/*
    for row in 0..=max_y.1 {
        println!();

        for column in min_x.0..=max_x.0 {
            if row == 0 && column == 500 {
                print!("+");
            }
            if map.iter().any(|p| p == &(column, row)) {
                print!("#");
            } else {
                print!(".");
            }
        }  
    }

    println!();
*/
    fn check_available(map: &HashMap<(usize,usize), bool>, point: &(usize, usize), max_y: usize) -> bool {
        //println!("checking max_y == {}, and point.y {}", max_y, point.1);
        return point.1 < max_y + 2 && !map.contains_key(point);
    }

    let mut map = map.clone();
    let mut hash_map:HashMap<(usize,usize), bool> = HashMap::new();
    map.into_iter().for_each(|p| {
        hash_map.insert(p, true);
    });

    let mut stable = 0;
    let mut became_stable = true;
    let mut stuck_at_entrance = false;
    while !stuck_at_entrance {
        became_stable = false;
        let mut sand = (500, 0);
        while sand.1 <= max_y.1 + 3 && !became_stable {
           if check_available(&hash_map, &(sand.0, sand.1 + 1), max_y.1)  {
               sand = (sand.0, sand.1 + 1); 
           } else if check_available(&hash_map, &(sand.0 - 1, sand.1 + 1), max_y.1) {
               sand = (sand.0 - 1, sand.1 + 1); 
           } else if check_available(&hash_map, &(sand.0 + 1, sand.1 + 1), max_y.1) {
               sand = (sand.0 + 1, sand.1 + 1); 
           } else {
               //became stable 
               hash_map.insert(sand, true);
               stable += 1;
               became_stable = true;
               println!("Sand became stable at {:?} number of stable {}", sand, stable);
           }
        } 
        if sand == (500,0) {
            stuck_at_entrance = true;
        }
    }

    return format!("There are {} sand in the cave", stable);
}
