use std::collections::HashMap;

use crate::input_helper;
use crate::vec_helper::filter_uniq;
use super::model::Move;

pub fn solve() -> String {

    let moves: Vec<Move> = input_helper::read_input("input/22_09_test");
    let mut head_pos: (isize, isize) = (0,0);
    let mut tail_pos: (isize, isize) = (0,0);
    let mut passed_possitions = vec!("0,0".to_string());

    for rope_move in moves {
        match rope_move {
            Move::Up(l) => {
               for _ in 0..l {
                   head_pos.1 -= 1;
                   let new_pos = rope_move.move_point(&head_pos, &mut tail_pos);
                   //println!("move {:?} makes head {},{}  and tail {},{}", rope_move, head_pos.1, head_pos.0, tail_pos.1, tail_pos.0);
                   passed_possitions.push(new_pos);
               } 
            }
            Move::Down(l) => {
               for _ in 0..l {
                   head_pos.1 += 1;
                   let new_pos = rope_move.move_point(&head_pos, &mut tail_pos);
                   passed_possitions.push(new_pos);
               } 
            },
            Move::Left(l) => {
               for _ in 0..l {
                   head_pos.0 -= 1;
                   let new_pos = rope_move.move_point(&head_pos, &mut tail_pos);
                   passed_possitions.push(new_pos);
               } 
            },
            Move::Right(l) => {
               for _ in 0..l {
                   head_pos.0 += 1;
                   let new_pos = rope_move.move_point(&head_pos, &mut tail_pos);
                   passed_possitions.push(new_pos);
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
