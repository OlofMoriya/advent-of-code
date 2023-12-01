use core::fmt;
use std::error::Error;

use crate::deserializable::Deserializable;

#[derive(Debug)]
pub struct XError {
    message: String
}

impl Error for XError {
}
impl fmt::Display for XError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parsing had an error, {}", self.message)
    }
} 

#[derive(Debug, Clone)]
pub enum Part {
    Int(usize),
    List(Vec<Part>),
}

impl Deserializable for Part {
    type Err = XError;
    fn deserialize(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            //println!("is this why?");
            panic!("Came here with empty string");
        }

        //println!("parsing {}", s);
        let trimmed = s[1..s.len()-1].to_string();
        let mut current_list:Vec<Part> = vec!();
        let mut open = 0;
        let mut sub_part_index:isize = -1;
        let mut index = 0;

        while index < trimmed.len() {
            //println!("{}",index);
            let sub_trimmed = &trimmed[index..];
            match sub_trimmed {
                _ if sub_trimmed.starts_with(",") => {
                    //Do nothing with ,
                    //We continue and advande the index 1 more. 
                }
                _ if sub_trimmed.starts_with("[") => {
                    open += 1;
                    if open == 1 {
                        sub_part_index = index as isize; 
                        //println!("opening {} at {}",open, sub_part_index);
                    }
                    //else {
                    //    println!("found opening but ignoring open: {}", open);
                    //}
                },
                _ if sub_trimmed.starts_with("]") => {
                    open -= 1;
                    if open == 0 && sub_part_index != -1 {
                        //let sub_part_end_index = trimmed[(sub_part_index as usize)..].to_string().find(working_string.as_str()).expect("always a substring") as isize + sub_part_index;

                        //println!("{} to {} will be the new substring",sub_part_index, index);

                        let sub_part = Part::deserialize(&trimmed[(sub_part_index as usize)..=index])?; 
                        current_list.push(sub_part);
                        sub_part_index = -1;
                    }
                },
                _ => {
                    if sub_part_index == -1 { 
                        let i = sub_trimmed.find(",");
                        match i {
                            Some(v) => {
                                let mut str_to_parse = &sub_trimmed[0..v];
                                str_to_parse = str_to_parse.trim_end_matches(']');
                                match str_to_parse.parse::<usize>() {
                                    Ok(value) => {
                                        current_list.push(Part::Int(value));
                                        index += v - 1;
                                    },
                                    Err(_) => {
                                        ////println!("failed parsing {:?} as a number", str_to_parse.to_string());
                                        //println!("this is probably disasturous");
                                    },
                                }
                            },
                            None => {
                                let mut str_to_parse = sub_trimmed;
                                str_to_parse = str_to_parse.trim_end_matches(']');
                                //println!("no comma in: {}", sub_trimmed);
                                //println!("This should not be happening?");
                                match str_to_parse.parse::<usize>() {
                                    Ok(v) => {
                                        current_list.push(Part::Int(v));
                                        //println!("returning none {:?}", current_list);
                                        return Ok(Part::List(current_list)); // end of list
                                    },
                                    Err(_) => {
                                        //println!("failed parsing {}", sub_trimmed);
                                        panic!("Doh?");
                                    },
                                }
                            }
                        };
                    }
                }
            };
            index += 1;
        }
        
        //println!("returning {:?}", current_list);
        return Ok(Part::List(current_list));
    }
}

pub enum Comp {
    True, 
    False, 
    Inconclusive
}

impl Part {
    pub fn before(&self, other: &Part) -> Comp {
        match self {
            Part::Int(s_value) => match other {
                Part::Int(o_value) => {
                    return match o_value {
                        _ if o_value == s_value => Comp::Inconclusive,
                        _ if o_value > s_value => Comp::True,
                        _ => Comp::False
                    }
                },
                Part::List(_) => return Part::List(vec!(self.clone())).before(other)
            },
            Part::List(s_list) => match other {
                Part::Int(_) => return self.before(&Part::List(vec!(other.clone()))),
                Part::List(o_list) => {
                    println!("comparing lists: {:?} {:?}", s_list, o_list); 
                    for i in 0..s_list.len().min(o_list.len()) {
                        match s_list[i].before(&o_list[i]) {
                            Comp::True => return Comp::True,
                            Comp::False => return Comp::False,
                            _ => (),
                        };
                    };
                    if s_list.len() > o_list.len() {return Comp::False;}
                    else if s_list.len() < o_list.len() {return Comp::True;}
                    else {
                        println!("aiko {:?}, {:?}", s_list, o_list);
                        return Comp::Inconclusive;
                    }
                },
            },
        }    
    }
}

