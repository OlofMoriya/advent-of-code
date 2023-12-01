use core::fmt;
use std::error::Error;

use crate::deserializable::Deserializable;

#[derive(Debug)]
pub struct RockPaperScissorsError {}

impl Error for RockPaperScissorsError {}
impl fmt::Display for RockPaperScissorsError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RockPaperScissors had an error")
    }
} 

#[derive(Debug)]
enum Outcome {
    Win = 6,
    Tie = 3,
    Loss = 0,
}

#[derive(Debug)]
enum Play {
    Rock = 1, 
    Paper = 2, 
    Scissor = 3,
}

#[derive(Debug)]
pub struct RockPaperScissors {
  elf_play: Play,
  player_play: Play,
  desired_outcome: Outcome
}

impl Deserializable for RockPaperScissors {
    type Err = RockPaperScissorsError;
    fn deserialize(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(" ").unwrap();

        let elf_play = match left {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissor,
            _ => return Err(RockPaperScissorsError{})
        };

        let player_play = match right {
            "X" => Play::Rock,
            "Y" => Play::Paper,
            "Z" => Play::Scissor,
            _ => return Err(RockPaperScissorsError{})
        };
        
        let outcome = match right {
            "X" => Outcome::Loss,
            "Y" => Outcome::Tie,
            "Z" => Outcome::Win,
            _ => return Err(RockPaperScissorsError{})
        };

        return Ok(RockPaperScissors {elf_play, player_play, desired_outcome: outcome});
    }
}

impl RockPaperScissors {
    pub fn calculate_score(self) -> u32 {
        let outcome = self.calc_outcome();
        return outcome + self.player_play as u32;
    }

    pub fn calculate_score_with_second_strat(self) -> u32 {
        let play =  match self.desired_outcome {
            Outcome::Loss =>  match self.elf_play {
                Play::Rock => Play::Scissor, 
                Play::Paper => Play::Rock, 
                Play::Scissor => Play::Paper, 
            }, 
            Outcome::Tie =>  self.elf_play,
            Outcome::Win => {
                match self.elf_play {
                    Play::Rock => Play::Paper, 
                    Play::Paper => Play::Scissor, 
                    Play::Scissor => Play::Rock, 
                }
            }
        };

        return self.desired_outcome as u32 + play as u32;
    }

    fn calc_outcome(&self) -> u32 {
        let outcome = match self.elf_play {
            Play::Rock => match self.player_play {
                Play::Rock => Outcome::Tie,
                Play::Paper => Outcome::Win,
                Play::Scissor => Outcome::Loss,
            },
            Play::Paper => match self.player_play {
                Play::Rock => Outcome::Loss,
                Play::Paper => Outcome::Tie,
                Play::Scissor => Outcome::Win,
            },
            Play::Scissor => match self.player_play {
                Play::Rock => Outcome::Win,
                Play::Paper => Outcome::Loss,
                Play::Scissor => Outcome::Tie,
            }
        };
        return outcome as u32;
    }
}
