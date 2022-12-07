use std::collections::HashMap;

pub fn solve() -> String {
    let mut current_dirs:Vec<String> = vec!();
    let mut dirs:HashMap<Vec<String>, u64> = HashMap::new();
    let input = include_str!("../../input/22_07_test");
    input.lines().for_each(|o| {
        let mut split = o.split(" ").collect::<Vec<&str>>();
        split.push(" "); 
        match split[0..3] {
            ["$", "cd", ".."] => {
                current_dirs.pop();
            },
            ["$", "cd", str] => {
                current_dirs.push(str.to_string());
                dirs.insert(current_dirs.clone(), 0);
            },
            [value, _name, _] => {
                match value.parse::<u64>() {
                    Ok(number) => {
                        let mut path_to_add = vec!();
                        current_dirs
                            .iter()
                            .for_each(|cd| {
                                path_to_add.push(cd.clone());
                                dirs.entry(path_to_add.clone()).and_modify(|d| *d += number);
                             });
                    },
                    Err(..) => {},
                    
                };
            },
            _ => unreachable!()
        };
    });

    let root = &current_dirs[0..1];
    let space_needed:u64 = 30000000 - (70000000 - dirs.get(root).expect("root"));
    
    let values:Vec<u64> = dirs.into_values().collect();
    let values_for_min  = values.clone();

    let total:u64 = values.iter().filter(|s| *s <= &100000).sum::<u64>().clone();
    let min = values_for_min.into_iter().filter(|v| v >= &space_needed).min();

    return format!("{}, 2. {}", total, min.unwrap());
}

pub fn solve_two() -> String {
    return format!("");
}

