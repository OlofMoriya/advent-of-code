use std::fs;

use itertools::Itertools;

pub fn solve() -> String {
    let input = fs::read("input/22_05");
    match input {
        Err(error) => {
            println!("{}", error);
            panic!();
        },
        Ok(content) => {
            let all_as_string = String::from_utf8_lossy(&content);
            let (start, move_orders) = all_as_string.split_once("\n\n").expect("There should be two sections");

            let mut lines:Vec<&str> = start.lines().collect();
            //Pop the numbers from the input
            _ = lines.pop();

            //Assumes whitespace padding in the input
            let mut original_stacks:Vec<Vec<char>> = vec!(vec!();lines[0].len()/4 + 1);  

            lines.iter().for_each(|line| {
                for (i, chunk) in line.chars().chunks(4).into_iter().enumerate(){
                    let desc:Vec<char> = chunk.collect();
                    if desc[1] != ' ' {
                        original_stacks[i].insert(0, desc[1]); 
                    }
                }
            });

            move_orders.lines().map(|l| {
                let words = l.split(" ").collect::<Vec<&str>>();
                assert!(words.len() > 5);
                return (words[1].parse::<usize>().expect("move order should specify count"), 
                        words[3].parse::<usize>().expect("move order should specify from stack"), 
                        words[5].parse::<usize>().expect("move order should specify to stack")) 
            }).for_each(|i|{
                for _ in 0..i.0 {
                    let moving_crate = original_stacks[i.1-1].pop().expect("aoc should not give instructions that can't be followed");
                    original_stacks[i.2-1].push(moving_crate);
                }
            });

            let answer:String = original_stacks.iter().map(|s| s.last().expect("s should have some values here")).collect();

            return format!("top crates are {:?}", answer);
        }
    }
}


pub fn solve_two() -> String {
    let input = fs::read("input/22_05");
    match input {
        Err(error) => {
            println!("{}", error);
            panic!();
        },
        Ok(content) => {
            let all_as_string = String::from_utf8_lossy(&content);
            let (start, move_orders) = all_as_string.split_once("\n\n").unwrap();

            let mut lines:Vec<&str> = start.lines().collect();
            _ = lines.pop();
            let mut original_stacks:Vec<Vec<char>> = vec!(vec!();lines[0].len()/4 + 1);  
            lines.iter().for_each(|l| {
                println!("line: {}", l);
                for c in l.chars().chunks(4).into_iter().enumerate(){

                    println!("chunk: {}", l);
                    let desc:Vec<char> = c.1.into_iter().collect();
                    if desc[1] != ' ' {
                        println!("pushing: {} to {}", desc[1], c.0);
                        original_stacks[c.0].insert(0, desc[1]); 
                        println!("pushed: {} to {}", desc[1], c.0);
                    }
                }
            });


            println!("starting stacks: {:?}", original_stacks);

            move_orders.lines().map(|l| {
                let words = l.split(" ").collect::<Vec<&str>>();
                return (words[1].parse::<usize>().unwrap(), words[3].parse::<usize>().unwrap(), words[5].parse::<usize>().unwrap()) 
            }).for_each(|i|{
                println!("move {} from {} to {}", i.0, i.1, i.2);
                let mut temp = vec!();
                for b in 0..i.0 {
                    let new = original_stacks[i.1-1].pop().unwrap();
                    temp.insert(0, new);
                }
                original_stacks[i.2-1].append(&mut temp.clone());
                println!("{:?}", original_stacks);
            });

            let answer:String = original_stacks.iter().map(|s| s.last().unwrap()).collect();

            return format!("top crates are {:?}", answer);
        }
    }

}
