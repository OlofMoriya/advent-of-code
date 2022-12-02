use std::fs;

use crate::deserializable::Deserializable;

#[allow(dead_code)]
pub fn read_input_with_none<T>(file_path: &str) -> Vec<Option<T>> where T: Deserializable {
    let input = fs::read(file_path);
    match input {
        Err(error) => {
            println!("{}", error);
            panic!();
        },
        Ok(content) => {
            let all_as_string = String::from_utf8_lossy(&content);
            let rows = all_as_string.lines();
            let parsed_items:Vec<Option<T>> = rows
                .map(|r| match Deserializable::deserialize(r) 
                     { 
                         Ok(parsed_type)=> Some(parsed_type), 
                         Err(..) => {
                             None
                         }
                     })
                .collect();
               
            return parsed_items;
        }
    }
}

#[allow(dead_code)]
pub fn read_input<T>(file_path: &str) -> Vec<T> where T: Deserializable {
    let input = fs::read(file_path);
    match input {
        Err(error) => {
            println!("{}", error);
            panic!();
        },
        Ok(content) => {
            let all_as_string = String::from_utf8_lossy(&content);
            let rows = all_as_string.lines();
            let parsed_items:Vec<T> = rows
                .filter_map(|r| match Deserializable::deserialize(r) 
                     { 
                         Ok(parsed_type)=> Some(parsed_type), 
                         Err(..) => {
                             println!("Cannor parse: {:?}", r);
                             None
                         }
                     })
                .collect();
            return parsed_items;
        }
    }
}
