use crate::input_helper;

use super::model::Instruction;

pub fn solve() -> String {
    let mut input = input_helper::read_input::<Instruction>("input/22_10");
    input.reverse();
    let mut x = 1;
    let mut signals: Vec<isize> = vec!();

    fn add_signal(x:isize, index: isize, signals: &mut Vec<isize>) -> () {
        if (index-20) % 40 == 0 {
            signals.push(index * x);
        } 
    }

    let total_ticks = input.iter().map(|i|  match i {
        Instruction::AddX(_) => 2,
        Instruction::Noop() => 1,
    }).sum::<usize>();

    let mut current_instruction: Option<Instruction> = None;
    for i in 1..=total_ticks {
        
        let instruction: Instruction;
        if current_instruction.is_none() {
            instruction = input.pop().expect("The should still be instructions left");
        }
        else {
            instruction = current_instruction.clone().expect("Has to be one");
        }
        match instruction {
            Instruction::AddX(v) => {
                if current_instruction.is_some(){
                    add_signal(x, i as isize, &mut signals);
                    current_instruction = None;
                    x += v;
                } else {
                    add_signal(x, i as isize, &mut signals);
                    current_instruction = Some(instruction);
                }
            },
            Instruction::Noop() => {
                add_signal(x, i as isize, &mut signals);
            },
        };

        let should_print = x.abs_diff((i as isize)%40) <= 1;
        print!("{}", if should_print {"#"} else {"."});
        if i % 40 == 0 {
            println!();
        }
    }
    return format!("The signal sum is {}", signals.iter().sum::<isize>());
}

pub fn solve_two() -> String {
    return "10".to_string();
}
