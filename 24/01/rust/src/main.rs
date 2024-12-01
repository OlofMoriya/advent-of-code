use std::fs;

fn main() {
    println!("Hello, world!");
    let strings = read_input_as_str("/Users/olofmoriya/versioned/personal/advent-of-code/24/01/01");
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    strings.iter().for_each(|f| {
        let mut pair = f.split_whitespace();
        left.push(pair.next().unwrap().parse().unwrap());
        right.push(pair.next().unwrap().parse().unwrap());
    });

    let sum: i32 = left
        .iter()
        .map(|l| (right.iter().filter(|r| *r == l).count() as i32) * l)
        .sum();

    println!("sum: {}", sum);
}

pub fn read_input_as_str(file_path: &str) -> Vec<String> {
    let input = fs::read(file_path);
    match input {
        Err(error) => {
            println!("{}", error);
            panic!();
        }
        Ok(content) => {
            let all_as_string = String::from_utf8_lossy(&content);
            let rows = all_as_string.lines();
            return rows.map(|s| s.to_string()).collect();
        }
    }
}
