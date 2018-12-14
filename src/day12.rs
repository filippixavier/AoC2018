extern crate regex;

use self::regex::Regex;

use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::HashMap;

fn prepare_input() -> (String, HashMap<String, char>) {
    let input = fs::read_to_string(Path::new("./data/day12.txt")).unwrap();
    let initial_state_reg = Regex::new(r"((?:#|\.)+)").unwrap();
    let initial_state = if let Some(cap) = initial_state_reg.captures(&input){
        cap[1].to_string()
    } else {
        String::from(".")
    };
    let mut transformation = HashMap::<String, char>::new();
    let transformation_reg = Regex::new(r"((?:\.|#){5}) => (\.|#)").unwrap();

    for capture in transformation_reg.captures_iter(&input) {
        let (key, value) = (capture[1].to_string(), capture[2].chars().nth(0).unwrap());
        transformation.insert(key, value);
    }

    (initial_state, transformation)
}

pub fn first_star() -> Result<(), Box<Error + 'static>> {
    let (mut state, transformation) = prepare_input();
    let mut start = 0;
    for _ in 0..20 {
        if state.ends_with("#") {
            state.push_str("..");
        }
        if state.starts_with("#") {
            state.insert_str(0, "..");
            start -= 2;
        }
        let mut new_state = String::new();
        let mut cloned_state = state.clone().chars().collect::<Vec<char>>();
        for i in 0..state.chars().count() {
            let mut key = String::new();
            if i == 0 {
                key.push_str("..");
            } else if i == 1 {
                key.push('.');
                key.push(*cloned_state.get(i - 1).unwrap());
            } else {
                key.push(*cloned_state.get(i - 2).unwrap());
                key.push(*cloned_state.get(i - 1).unwrap());
            }
            key.push(*cloned_state.get(i).unwrap());
            key.push(*cloned_state.get(i+1).unwrap_or(&'.'));
            key.push(*cloned_state.get(i+2).unwrap_or(&'.'));

            new_state.push(*transformation.get(&key).unwrap_or(&'.'));
        }
        state = new_state;
    }
    println!("State after 20 steps:\n{}", state);
    let answer = state.chars().enumerate().fold(0, |acc, (index, ch)| {
        if ch == '#' {
            return acc + (index as i32 + start);
        }
        acc
    });
    println!("Pots values: {}", answer);
    Ok(())
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    Ok(())
}