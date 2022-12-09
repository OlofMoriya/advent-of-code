use std::collections::HashMap;

use crate::input_helper;
use crate::vec_helper::filter_uniq;
use super::model::Move;

pub fn solve() -> String {

    let moves: Vec<Move> = input_helper::read_input("input/22_09");
    let mut head_pos: (isize, isize) = (0,0);
    let mut tail_pos: (isize, isize) = (0,0);
    let mut passed_possitions = vec!("0,0".to_string());

    for rope_move in moves {
        match rope_move {
            Move::Up(l) => {
               for _ in 0..l {
                   head_pos.1 -= 1;
                   if tail_pos.1.abs_diff(head_pos.1) == 2 {
                        tail_pos.1 -= 1;
                        if tail_pos.0 != head_pos.0 {
                            tail_pos.0 = head_pos.0;
                        } 
                   }
                   passed_possitions.push(format!("{},{}", tail_pos.0, tail_pos.1));
                   println!("move {:?}, passes possition {},{}", rope_move, tail_pos.1, tail_pos.0);
               } 
            }
            Move::Down(l) => {
               for _ in 0..l {
                   head_pos.1 += 1;
                   if (tail_pos.1).abs_diff(head_pos.1) == 2 {
                        tail_pos.1 += 1;
                        if tail_pos.0 != head_pos.0 {
                            tail_pos.0 = head_pos.0;
                        } 
                   }
                   passed_possitions.push(format!("{},{}", tail_pos.0, tail_pos.1));
                   println!("move {:?}, passes possition {},{}", rope_move, tail_pos.1, tail_pos.0);
               } 
            },
            Move::Left(l) => {
               for _ in 0..l {
                   head_pos.0 -= 1;
                   if (tail_pos.0).abs_diff(head_pos.0) == 2 {
                        tail_pos.0 -= 1;
                        if tail_pos.1 != head_pos.1 {
                            tail_pos.1 = head_pos.1;
                        } 
                   }
                   passed_possitions.push(format!("{},{}", tail_pos.0, tail_pos.1));
                   println!("move {:?}, passes possition {},{}", rope_move, tail_pos.1, tail_pos.0);
               } 
            },
            Move::Right(l) => {
               for _ in 0..l {
                   head_pos.0 += 1;
                   if (tail_pos.0).abs_diff(head_pos.0) == 2 {
                        tail_pos.0 += 1;
                        if tail_pos.1 != head_pos.1 {
                            tail_pos.1 = head_pos.1;
                        } 
                   }
                   passed_possitions.push(format!("{},{}", tail_pos.0, tail_pos.1));
                   println!("move {:?}, passes possition {},{}", rope_move, tail_pos.1, tail_pos.0);
               } 
            },
        } 
    }
    
    let uniq = filter_uniq(&passed_possitions).len();
    return format!("the tail passes {} possitions", uniq);
}

pub fn solve_two() -> String {
    return format!("{}", 9);
}
