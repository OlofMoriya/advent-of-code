use core::fmt;
use std::error::Error;

use crate::deserializable::Deserializable;

#[derive(Debug)]
pub struct XError {
    message: String
}

impl Error for XError {
}

impl fmt::Display for XError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parsing had an error, {}", self.message)
    }
} 


#[derive(Debug, Clone)]
pub enum Operation {
    Add(u64),
    Multiply(u64), 
    Square,
}

impl Operation {
    pub fn do_op(&self, value: u64, divisible: u64) -> u64 {
       return match self {
           Operation::Add(v) => (v + value) % divisible,
           Operation::Multiply(v) => (v * value) %divisible,
           Operation::Square => (value * value)%divisible,
       }
    }
}

#[derive(Debug, Clone)]
pub struct Item {
   pub value: u64,
   pub operations: Vec<Operation>,
}

impl Item {
    fn new (value:u64) -> Item {
        return Item {
            value,
            operations: vec!(),
        };
    }

    pub fn eval (&self, divisor: u64) -> bool {
        let mut total = self.value;
        for op in &self.operations {
            total = op.do_op(total, divisor); 
        } 
        return total == 0;
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    pub index: usize,
    pub items: Vec<Item>,
    pub operation: Operation,
    pub test_divisible: u64,
    pub throw_to: (usize, usize),
}

impl Deserializable for Monkey {
    type Err = XError;
    fn deserialize(s: &str) -> Result<Self, Self::Err> {
        let rows:Vec<&str> = s.lines().collect();

        //Line 0
        let r = rows[0].to_string();
        let (_, number) = r.split_once(" ").unwrap();
        let number = number.trim_end_matches(":");
        let index = number.parse::<usize>().unwrap();

        //Line 1, Items
        let r = rows[1].to_string();
        let (_, numbers) = r.split_once(": ").unwrap();
        let mut numbers: Vec<u64> = numbers.split(", ").map(|n| n.parse::<u64>().unwrap()).collect(); 
        numbers.reverse();
        
        //Line 2, Operation
        let r = rows[2].to_string();
        let (operation, value) = r.split_once("old ").unwrap().1.split_once(" ").unwrap();
        println!("|{}|,|{}|", operation, value);
        let operation: Operation = match value.parse::<u64>() {
            Ok(v) => {
                match operation {
                    "+" => Operation::Add(v),
                    "*" => Operation::Multiply(v),
                    _ => unreachable!()
                }
            },
            Err(_) => match operation {
                    "+" => Operation::Multiply(2),
                    "*" => Operation::Square,
                    _ => unreachable!()
                }
            
        };
        
        //Line 3, Test
        let divisible:u64 = rows[3].to_string().split_once("by ").unwrap().1.parse().unwrap();
        numbers = numbers.iter().map(|n| n % divisible).collect();
        //Line 4, True
        let true_case:usize = rows[4].to_string().split_once("monkey ").unwrap().1.parse().unwrap();
        let false_case:usize = rows[5].to_string().split_once("monkey ").unwrap().1.parse().unwrap();
        
        return Ok(Monkey {
            index: index,
            items: numbers.into_iter().map(|n| Item::new(n)).collect(),
            operation: operation,
            test_divisible: divisible,
            throw_to: (true_case, false_case),
        });
    }
}

impl Monkey {
}
