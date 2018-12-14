extern crate regex;

use self::regex::Regex;

use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::HashMap;
use std::collections::VecDeque;

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

fn prepare_binary_input() -> (VecDeque<i32>, Vec<i32>) {
    let input = fs::read_to_string(Path::new("./data/day12.txt")).unwrap();
    let initial_state_reg = Regex::new(r"((?:#|\.)+)").unwrap();
    let initial_state = if let Some(cap) = initial_state_reg.captures(&input){
        cap[1].to_string().chars().map(|x| {
            if x == '#' {
                return 1;
            }
            return 0;
        }).collect::<VecDeque<i32>>()
    } else {
        VecDeque::<i32>::new()
    };
    let mut transformation = vec!(0;0b11111);
    let transformation_reg = Regex::new(r"((?:\.|#){5}) => (\.|#)").unwrap();

    for capture in transformation_reg.captures_iter(&input) {
        let (key, value) = (usize::from_str_radix(&capture[1].chars().map(|x| {
            if x == '#' {
                return '1';
            }
            return '0';
        }).collect::<String>(), 2).unwrap(), capture[2].chars().nth(0).unwrap());
        
        transformation[key] = if value == '#' {
            1
        } else {
            0
        };
    }

    (initial_state, transformation)
}

fn binary_pots_value(steps: u64) -> i32 {
    let (mut state, transformation) = prepare_binary_input();
    let mut start = 0;
    for _ in 0..steps {
        let mut key = 0b0;
        
        let begin = state.pop_front().unwrap();
        state.push_front(begin);
        let end = state.pop_back().unwrap();
        state.push_back(end);
        
        if begin == 1 {
            state.push_front(0);
            state.push_front(0);
            start -= 2;
        }
        if end == 1 {
            state.push_back(0);
            state.push_back(0);
        }
        
        let mut temp = state.clone();
        for (index, val) in state.iter().enumerate() {
            key = (key << 1 + val) & 0b11111;
            temp[index] = transformation[key];
        }
        state = temp;
    }
    println!("State after {} steps:\n{:?}", steps, state);
    state.iter().enumerate().fold(0, |acc, (index, value)| {
        if *value == 1 {
            return acc + index as i32 + start;
        }
        acc
    })
}

fn pots_values(steps: u64) -> i32 {
    let (mut state, transformation) = prepare_input();
    let mut start = 0;
    for _ in 0..steps {
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
    println!("State after {} steps:\n{}", steps, state);
    let answer = state.chars().enumerate().fold(0, |acc, (index, ch)| {
        if ch == '#' {
            return acc + (index as i32 + start);
        }
        acc
    });
    answer
}

pub fn first_star() -> Result<(), Box<Error + 'static>> {
    let answer = pots_values(20);
    println!("Pots values: {}", answer);
    Ok(())
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    let answer = binary_pots_value(20);
    println!("Pots values: {}", answer);
    Ok(())
}