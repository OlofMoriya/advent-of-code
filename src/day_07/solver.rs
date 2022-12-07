use std::collections::HashMap;
//use std::{vec, rc::Weak};

use crate::input_helper;

use super::model::Output;

/*struct Directory {
    name: String,
    files: Vec<u32>,
    dirs: Vec<Directory>
}

impl Directory {
    pub fn get_size(&self) -> u32 {
        return self.files.iter().sum::<u32>() + self.dirs.iter().map(|d| d.get_size()).sum::<u32>();
    }
    pub fn add_dir(&mut self, dir: Directory) -> () {
        self.dirs.push(dir);
    }
}*/ 

pub fn solve() -> String {
    let mut current_dirs:Vec<String> = vec!();
    let mut dirs:HashMap<String, u64> = HashMap::new();
    let input = input_helper::read_input::<Output>("input/22_07");
    input.iter().for_each(|o| {
        match &o.command {
            Some(c) => {
               if c.starts_with("cd") {
                    let (_cd, path) = c
                        .split_once(" ")
                        .expect("cd command should be splittable here");
                    if path == ".." {
                        current_dirs.pop();
                    }
                    else if path != "/" {
                        current_dirs.push(path.to_string());
                        let real_path = format!("/{}", current_dirs.join("/"));

                        dirs.insert(real_path, 0);
                    }
               }  
            },
            None => {
                let message = &o.message.clone().expect("Should have a value if no command");

                let (number_or_dir, _name_or_dir) = message
                    .split_once(" ")
                    .expect("Should have two values");

                let maybe_a_number = number_or_dir.parse::<u64>();
                match maybe_a_number {
                    Ok(v) => {
                        let mut real_path = "".to_string();
                        current_dirs.iter()
                             .for_each(|cd| {
                                 real_path = format!("{}/{}", real_path, cd);
                                 dirs.entry(real_path.clone()).and_modify(|d| *d += v);
                             });
                    }, 
                    Err(..) => {
                         
                    },
                }
            },
        };
    });
    let mut total = 0;
    for (name, size) in &dirs {
        if *size <= 100000 {
            total += size;
        } else {
        }
    }
    return format!("{}", total);
}

pub fn solve_two() -> String {

    let mut current_dirs:Vec<String> = vec!();
    let mut dirs:HashMap<String, u64> = HashMap::new();
    let input = input_helper::read_input::<Output>("input/22_07");
    input.iter().for_each(|o| {
        match &o.command {
            Some(c) => {
               if c.starts_with("cd") {
                    let (_cd, path) = c
                        .split_once(" ")
                        .expect("cd command should be splittable here");
                    if path == ".." {
                        current_dirs.pop();
                        println!("moved up .., current: {:?}", current_dirs);
                    }
                    else{
                        current_dirs.push(path.to_string());
                        let mut real_path = format!("{}", current_dirs.join("/"));
                        real_path = real_path.replace("//", "/");

                        println!("Current path to add: {}", real_path);
                        dirs.insert(real_path, 0);
                        println!("moved in {}, current: {:?}", path, current_dirs);
                    }
               }  
            },
            None => {
                let message = &o.message.clone().expect("Should have a value if no command");

                let (number_or_dir, _name_or_dir) = message
                    .split_once(" ")
                    .expect("Should have two values");

                let maybe_a_number = number_or_dir.parse::<u64>();
                match maybe_a_number {
                    Ok(v) => {
                        println!("adding {}, to: {:?}", v, current_dirs);
                        let mut real_path = "".to_string();
                        current_dirs.iter()
                             .for_each(|cd| {
                                 real_path = format!("{}/{}", real_path, cd);
                                 real_path = real_path.replace("//", "/");
                                 
                                 println!("Current path to add to: {}", real_path);
                                 dirs.entry(real_path.clone()).and_modify(|d| *d += v);
                             });
                    }, 
                    Err(..) => {
                         
                    },
                }
            },
        };
    });
    let space_needed:u64 = 30000000 - (70000000 - dirs.get(&"/".to_string()).expect("root"));
    let min = dirs.into_values().filter(|v| v >= &space_needed).min();
    return format!("min to remove {:?}", min);
}
