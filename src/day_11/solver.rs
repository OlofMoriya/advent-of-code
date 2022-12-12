use crate::input_helper;

use super::model::Monkey;

pub fn solve() -> String {

    let mut monkeys = input_helper::read_input_w_separator::<Monkey>("input/22_11_test", "\n\n");

    let mut inspections = vec![0;monkeys.len()];
    for iterations in 0..10000 {
        for i in 0..monkeys.len() {
            let mut monkey = monkeys[i].clone();
            inspections[monkey.index] += monkey.items.len();
            for _ in 0..monkey.items.len() {
                let mut item = monkey.items.pop().unwrap();
                //println!("popped item: {}", item); 
                item.operations.push(monkey.operation.clone());
                //println!("after op {:?} item is {}", monkey.operation, item); 
                //item = item / 3;
                //println!("after divition {} item % test_divisible {}", item, item % monkey.test_divisible);
                if item.eval(monkey.test_divisible) {
                    //println!("throw to {}", monkey.throw_to.0); 
                    monkeys[monkey.throw_to.0].items.insert(0, item);
                } else {
                    //println!("throw to {}", monkey.throw_to.1); 
                    monkeys[monkey.throw_to.1].items.insert(0, item);
                }
            }
            monkeys[i] = monkey;
        }
        if iterations == 0 || iterations == 19 || iterations % 9999 == 0 {
            
            println!("{:?}", &inspections);
        }
    }
    inspections.sort();
    inspections.reverse();
    println!("{:?}", &inspections);
    let product = inspections.into_iter().take(2).reduce(|acc,e| acc * e);
    return format!("{:?}", product);
}

pub fn solve_two() -> String {
    return format!("{}", 1);
}
