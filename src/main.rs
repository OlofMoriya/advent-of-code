#![feature(slice_group_by)]
mod input_helper;
mod aoc_22_01;
mod deserializable;
mod u32_extension;

use crate::aoc_22_01::{solve, solve_two};  

fn main() {
    println!("{}", solve());
    println!("{}", solve_two());
}

