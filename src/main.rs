#![allow(dead_code, unused_variables)]
mod simple_struct;
mod deserializable;
mod u32_extension;
mod input_helper;
mod aoc_21_18;

use crate::aoc_21_18::{solve_snails_two, solve_snails};  

//use simple_struct::SimpleType;
//use input_helper::read_input;

fn main() {
//    let numbers = read_input::<u32>("input_u32");
//    println!("{:?}", numbers);

//    let structs = read_input::<SimpleType>("input_simple_struct");
//    println!("{:?}", structs);

    println!("{}", solve_snails());
    println!("{}", solve_snails_two());
}

