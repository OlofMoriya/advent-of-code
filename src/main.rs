#![feature(slice_group_by)]
mod input_helper;
mod vec_helper;
mod deserializable;
mod u32_extension;

mod day_03;

use crate::day_03::solver::{solve, solve_two};  

fn main() {
    println!();
    println!();
    println!("1. {}", solve());
    println!("2. {}", solve_two());
}

