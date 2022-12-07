use std::{vec, rc::Weak};

use crate::input_helper;

use super::model::Output;

struct Directory {
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
} 

pub fn solve() -> String {
    let mut root: Option<Directory> = None;
    let mut pwd: Weak<Directory> = Weak::new();
    let input = input_helper::read_input::<Output>("input/22_07_test");
    input.iter().for_each(|o| {
        match &o.command {
            Some(c) => {
                if c.starts_with("cd"){
                    let (_cd, d_name) = c.split_once(" ").expect("should always be 'cs dir' as a command");
                    let dir = Directory{ name: d_name.to_string(), files: vec!(), dirs: vec!()};

                    if root.is_some(){
                        root = Some(dir); 
                    } else {
                        unsafe{
                            let t = pwd.upgrade();
                            t.expect("I don't know what I'm doing do I ?").add_dir(dir);
                        }
                        pwd = Weak{ptr: Rdir};
                    }
                        
                } 
            } 
            None => (),
        }
    });
    return format!("{:?}", input);
}

pub fn solve_two() -> String {
    let input = input_helper::read_input::<Output>("input/22_07_test");
    return format!("{}", 7);
}
