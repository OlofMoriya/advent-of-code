#![allow(dead_code, unused_variables)]
mod simple_struct;
mod deserializable;
mod u32_extension;
mod input_helper;
mod aoc_22_01;

use crate::aoc_22_01::{solve, solve_two};  

fn main() {
    println!("{}", solve());
    println!("{}", solve_two());
}

