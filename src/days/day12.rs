extern crate regex;

use self::regex::Regex;

use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::HashMap;

fn prepare_input() -> (String, HashMap<String, char>) {
    let input = fs::read_to_string(Path::new("../data/day12.txt")).unwrap();
    let initial_state_reg = Regex::new(r"((?:#|\.)+)").unwrap();
    let initial_state = if let Some(cap) = initial_state_reg.captures(&input) {
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

fn pots_values(steps: i64) -> i64 {
    let (mut state, transformation) = prepare_input();
    let mut states = Vec::<String>::new();
    let all_pots_reg = Regex::new(r"(#+\.*)+#").unwrap();
    let mut start: i64 = 0;
    for i in 0..steps {
        if state.ends_with('#') {
            state.push_str("..");
        }
        if state.starts_with('#') {
            state.insert_str(0, "..");
            start -= 2;
        }
        let mut new_state = String::new();
        let mut cloned_state = state.clone().chars().collect::<Vec<char>>();
        for j in 0..state.chars().count() {
            let mut key = String::new();
            if j == 0 {
                key.push_str("..");
            } else if j == 1 {
                key.push('.');
                key.push(cloned_state[j - 1]);
            } else {
                key.push(cloned_state[j - 2]);
                key.push(cloned_state[j - 1]);
            }
            key.push(cloned_state[j]);
            key.push(*cloned_state.get(j + 1).unwrap_or(&'.'));
            key.push(*cloned_state.get(j + 2).unwrap_or(&'.'));
            new_state.push(*transformation.get(&key).unwrap_or(&'.'));
        }
        state = new_state;

        if let Some(found) = all_pots_reg.find(&state.clone()) {
            let capture = &state[found.start()..found.end()].to_string();
            // Shortcut since in my case the input have a cycle of 1, shifting to the right
            if states.iter().position(|x| *x == *capture).is_some() {
                println!("{}, {}", steps, i);
                start += steps - 1 - i; // -1 since the range is [0, steps[
                break;
            } else {
                states.push(capture.to_string());
            }
        }
    }
    println!(
        "State after {} steps:\n{}, start at: {}",
        steps, state, start
    );
    state.chars().enumerate().fold(0, |acc, (index, ch)| {
        if ch == '#' {
            return acc + (index as i64 + start);
        }
        acc
    })
}

pub fn first_star() -> Result<(), Box<Error + 'static>> {
    let answer = pots_values(20);
    println!("Pots values: {}", answer);
    Ok(())
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    let answer = pots_values(50_000_000_000);
    // 3650000001849 too high
    println!("Pots values: {}", answer);
    Ok(())
}
