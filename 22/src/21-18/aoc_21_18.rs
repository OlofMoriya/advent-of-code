use std::error::Error;
use std::fmt;

use input_helper::read_input;
use crate::deserializable::Deserializable;
use crate::input_helper;

#[derive(Clone, Debug)]
struct Snails {
    numbers_and_depth: Vec<(u8, u8)> 
}

impl Snails {
    fn calculate_magnitude(&self) -> u32 {
        let mut vec: Vec<(u32, u8)> = self.numbers_and_depth.clone().iter().map(|p| (p.0 as u32, p.1)).collect();
        for l in (1..=5).rev() {
            let mut i = 0;
            while i < vec.len(){
                if vec[i].1 == l {
                    vec[i].1 -= 1;
                    vec[i].0 = 3 * vec[i].0 + 2 * vec[i+1].0;
                    vec.remove(i+1);
                }
                i += 1;
            } 
        };
        return vec[0].0.into();
    }

    fn add_snails(self, snails: Snails) -> Snails{
        Snails{ 
            numbers_and_depth: 
                [self.numbers_and_depth, snails.numbers_and_depth]
                .concat().into_iter().map(|p| (p.0, p.1 + 1)).collect()
        }
    }

    fn expload(mut self) -> Snails{
        let mut changed = false;
        for i in 0..self.numbers_and_depth.len() {
            if i >= self.numbers_and_depth.len() { break; }
            if self.numbers_and_depth[i].1 == 5 {
                changed = true;
                if i > 0 {
                    self.numbers_and_depth[i-1].0 += self.numbers_and_depth[i].0;
                }
                if i+2 < self.numbers_and_depth.len() {
                    self.numbers_and_depth[i+2].0 += self.numbers_and_depth[i+1].0;
                }

                self.numbers_and_depth[i].0 = 0;
                self.numbers_and_depth[i].1 -= 1;
                self.numbers_and_depth.remove(i+1);
            } 
        }

        for i in 0..self.numbers_and_depth.len() {
            if self.numbers_and_depth[i].0 > 9 {
                changed = true;
                let pre_split = self.numbers_and_depth[i].0;
                self.numbers_and_depth[i].0 = pre_split / 2;
                self.numbers_and_depth[i].1 += 1;
                self.numbers_and_depth.insert(i+1, (pre_split - (self.numbers_and_depth[i].0), self.numbers_and_depth[i].1));

                break;
            }
        }

        if changed == true {return self.expload();} else {return self;}
    }
}

#[derive(Debug)]
struct SnailError {}

impl Error for SnailError {}
impl fmt::Display for SnailError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Snails had an error")
    }
} 

impl Deserializable for Snails {
    type Err = SnailError;

    fn deserialize(string: &str) -> Result<Self, Self::Err> {
        let mut depth = 0;
        let mut n_a_d = Vec::<(u8,u8)>::new();
        let helper_zero = '0' as u8;
        string.chars().for_each(|x| 
            match x {
                '[' => depth += 1,
                ']' => depth -= 1, 
                ',' => (),
                _ => {
                    let n = x as u8;
                    let num = n - helper_zero;
                    n_a_d.push((num, depth))
                },
            });
        Ok(Snails{
            numbers_and_depth : n_a_d
        })
    }
}

pub fn solve_snails_two() -> String {
    let input: Vec<Snails> = read_input("input/21-18"); 
    let mut sums = Vec::<u32>::new();
    for i in 0..input.len() {
        for j in 0..input.len() {
             sums.push(input[i].clone().add_snails(input[j].clone()).expload().calculate_magnitude());
             sums.push(input[j].clone().add_snails(input[i].clone()).expload().calculate_magnitude());
        }
    }
    sums.sort();

    return format!("answer to highest pair is {}", sums.last().unwrap()).to_string();
}

pub fn solve_snails() -> String {
   let input: Vec<Snails> = read_input("input/21-18"); 

   //println!("{:?} thats all", input);

   let sum_of_snails = input
       .into_iter()
       .reduce(|s, n_s| s.add_snails(n_s).expload());

   format!("answer: {:?}", sum_of_snails.unwrap().calculate_magnitude())
}
