#![feature(slice_group_by)]
mod input_helper;
mod day_02;
mod deserializable;
mod u32_extension;

use crate::day_02::solver::{solve, solve_two};  

fn main() {
    println!();
    println!();
    println!("1. {}", solve());
    println!("2. {}", solve_two());
}

