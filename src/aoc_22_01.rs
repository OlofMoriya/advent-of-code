use std::fs;

fn read_lines(path:&str) -> Vec<String> {
    let input = fs::read("input/22_01");
    match input {
        Err(error) => {
            println!("{}", error);
            panic!();
        },
        Ok(content) => {
            let all_as_string = String::from_utf8_lossy(&content);
            return all_as_string
                .lines()
                .map(|l| String::from(l))
                     .collect();
        }
    }
}

pub fn parse_elves_calories() -> Vec<u32>{
    let mut current = 0;
    let mut elves = Vec::<u32>::new();

    read_lines("input/22_01")
        .iter()
        .for_each(|l| 
          if l == "" {
              elves.push(current); 
              current = 0;
          }
          else {
              current += l.parse::<u32>().unwrap();
          });

    elves.push(current);
    return elves;
}

pub fn solve()-> String {
    let mut elves = parse_elves_calories();
    elves.sort();

    return format!("The elf with the most calories is carrying {:?} calories.", elves.last().unwrap());
}

pub fn solve_two()-> String {
    let mut elves = parse_elves_calories();
    elves.sort();

    let mut sum_of_three_last = 0;
    for i in 0..3 {
        let n = elves.pop().unwrap();
        sum_of_three_last += n;
    }

    return format!("The three elves with the most calories are carrying {:?} calories togther.", sum_of_three_last);
}
