use crate::input_helper;
use crate::vec_helper::filter_uniq;
use super::model::Move;

fn move_rope(rope_move: Move, points: &mut Vec<(isize, isize)>, start_index: usize ,passed_possitions: &mut Vec<String>){
        let move_steps =  match rope_move{
            Move::Up(v) => v,
            Move::Down(v) => v,
            Move::Left(v) => v,
            Move::Right(v) => v,
        };

        for s in 0..(move_steps) {
        for i in start_index..points.len() {
            let temp_head_index = (i as isize - 1).max(0);
            let temp_head = points[temp_head_index as usize].clone();
            match rope_move {
                Move::Up(_) => {
                        if i == start_index {
                            points[i].1 -= 1;
                        } else {
                            let new_pos = rope_move.move_point(&temp_head, &mut points[i]);
                            println!("move {:?}/{s} makes {}: {},{}  and {} {},{}", rope_move,i-1, temp_head.1, temp_head.0,  i, &points[i].1, &points[i].0);
                            match new_pos.1 {
                                Some(m) => {
                                    println!("side effect {:?}", m);
                                    //let mut discard:Vec<String> = vec!();
                                    //move_rope(m, points, i, &mut discard);
                                    move_rope(m, points, i, passed_possitions);
                                },
                                None => (),
                            }
                            if i == points.len() - 1 {
                                passed_possitions.push(new_pos.0);
                            }
                        }
                }
                Move::Down(_) => {
                        if i == start_index {
                            points[i].1 += 1;
                        } else {
                            let new_pos = rope_move.move_point(&temp_head, &mut points[i]);
                            println!("move {:?} makes {}: {},{}  and {} {},{}", rope_move,i-1, temp_head.1, temp_head.0,  i, &points[i].1, &points[i].0);

                            match new_pos.1 {
                                Some(m) => {
                                    //let mut discard:Vec<String> = vec!();
                                    //println!("side effect {:?}", m);
                                    //let mut discard:Vec<String> = vec!();
                                    //move_rope(m, points, i, &mut discard);
                                    move_rope(m, points, i, passed_possitions);
                                },
                                None => (),
                            }
                            if i == points.len() - 1 {
                                passed_possitions.push(new_pos.0);
                            }
                        }
                },
                Move::Left(_) => {
                        if i == start_index {
                            points[i].0 -= 1;
                        } else {
                            let new_pos = rope_move.move_point(&temp_head, &mut points[i]);
                            println!("move {:?} makes {}: {},{}  and {} {},{}", rope_move,i-1, temp_head.1, temp_head.0,  i, &points[i].1, &points[i].0);

                            match new_pos.1 {
                                Some(m) => {
                                    //let mut discard:Vec<String> = vec!();
                                    //println!("side effect {:?}", m);
                                    //let mut discard:Vec<String> = vec!();
                                    //move_rope(m, points, i, &mut discard);
                                    move_rope(m, points, i, passed_possitions);
                                },
                                None => (),
                            }
                            if i == points.len() - 1 {
                                passed_possitions.push(new_pos.0);
                            }
                        }
                },
                Move::Right(_) => {
                        if i == start_index {
                            points[i].0 += 1;
                        } else {
                            let new_pos = rope_move.move_point(&temp_head, &mut points[i]);
                            println!("move {:?}/{s} makes {}: {},{}  and {} {},{}", rope_move,i-1, temp_head.1, temp_head.0,  i, &points[i].1, &points[i].0);
                            match new_pos.1 {
                                Some(m) => {
                                    //let mut discard:Vec<String> = vec!();
                                    //println!("side effect {:?}", m);
                                    //let mut discard:Vec<String> = vec!();
                                    //move_rope(m, points, i, &mut discard);
                                    move_rope(m, points, i, passed_possitions);
                                },
                                None => (),
                            }

                            if i == points.len() - 1 {
                                passed_possitions.push(new_pos.0);
                            }
                        }
                },
            } 
        }
        }
}

pub fn solve() -> String {

    let moves: Vec<Move> = input_helper::read_input("input/22_09_test_two");
    let mut points: Vec<(isize, isize)> = vec!((0,0);10);
    let mut passed_possitions = vec!("0,0".to_string());

    for rope_move in moves {
        println!("{:?}", rope_move);
        move_rope(rope_move, &mut points, 0, &mut passed_possitions);
        println!("current positions:");
        &points.iter().enumerate().for_each(|(i,p)| {
            print!("{i}:{:?}    ", p);
        });
        println!();

    }
    
    let uniq = filter_uniq(&passed_possitions);

    println!();
    let mut last = "".to_string();
    for p in passed_possitions {
        if p != last {
            print!("{}  ", p);
            last = p.clone();
        }
    }
    println!();
    return format!("the tail passes {} possitions", uniq.len());
}

pub fn solve_two() -> String {
    return format!("{}", 9);
}
