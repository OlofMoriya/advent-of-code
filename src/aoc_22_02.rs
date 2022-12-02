use std::num::ParseIntError;

use crate::deserializable::Deserializable;
use crate::input_helper;

#[derive(Debug)]
pub struct Rps {
  left: u32,
  right: u32,
}

impl Deserializable for Rps {
    type Err = ParseIntError;
    fn deserialize(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(" ").unwrap();

        let left_n = match left {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => panic!()
        };

        let right_n = match right {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!()
        };
        return Ok(Rps {left: left_n, right: right_n});
    }
}

impl Rps {
    fn calculate_score(self) -> u32 {
        return self.right + self.calc_outcome();
    }

    fn calculate_score_with_new_strat(self) -> u32 {
        return match self.right {
            1 =>  match self.left {
                1 => 3, 
                2 => 1, 
                3 => 2,
                _ => panic!()
            }, 
            2 =>  self.left + 3,
            3 => {
                6 + match self.left {
                    1 => 2,
                    2 => 3,
                    3 => 1,
                    _ => panic!()
                }
            }
            _ => panic!()
        }
    }

    fn calc_outcome(self) -> u32 {
        let sub = self.left as i32 - self.right as i32;
        return match sub {
            -2 => 0,
            -1 => 6,
            0 => 3,
            1 => 0,
            2 => 6,
            _ => panic!()
        }
    }
}

pub fn solve()-> String {
    let input = input_helper::read_input::<Rps>("input/22_02");
    let score:u32 = input.into_iter().map(|rps| rps.calculate_score()).sum();

    return format!("the total score is {}", score);
}

pub fn solve_two() -> String {
    let input = input_helper::read_input::<Rps>("input/22_02");
    let score:Vec<u32> = input.into_iter().map(|rps| rps.calculate_score_with_new_strat()).collect();
    //println!("{:?}", score);
    let sum:u32 = score.iter().sum(); 
    return format!("with new strategy the total score is {}", sum);
}
