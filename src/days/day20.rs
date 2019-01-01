use std::error::Error;

use std::fs;
use std::path::Path;

use std::collections::HashMap;

pub fn first_star() -> Result<(), Box<Error + 'static>> {
    let input = fs::read_to_string(Path::new("../data/day20.txt")).unwrap();
    let mut main_stack = Vec::<Vec<usize>>::new();
    let mut current_stack = Vec::<usize>::new();
    let mut counter = 0;

    for instruction in input.chars() {
        match instruction {
            'E' | 'N' | 'S' | 'W' => counter += 1,
            '(' => {
                current_stack.push(counter);
                main_stack.push(current_stack);
                current_stack = Vec::new();
                counter = 0;
            }
            ')' => {
                current_stack.push(counter);
                let path_len = if *current_stack.last().unwrap() == 0 {
                    0
                } else {
                    *current_stack.iter().max().unwrap()
                };

                current_stack = main_stack.pop().unwrap();
                counter = current_stack.pop().unwrap();
                counter += path_len;
            }
            '|' => {
                current_stack.push(counter);
                counter = 0;
            }
            _ => {}
        }
    }

    println!("Longest shortest path: {}", counter);

    Ok(())
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    let input = fs::read_to_string(Path::new("../data/day20.txt")).unwrap();
    let mut main_stack = Vec::<((i32, i32), usize)>::new();
    let mut map = HashMap::<(i32, i32), usize>::new();
    let mut dist = 0;
    let mut position = (0, 0);

    map.insert(position, dist);

    for instruction in input.chars() {
        let mut moved = false;
        match instruction {
            'N' => {
                position.0 += 1;
                dist += 1;
                moved = true;
            }
            'S' => {
                position.0 -= 1;
                dist += 1;
                moved = true;
            }
            'E' => {
                position.1 += 1;
                dist += 1;
                moved = true;
            }
            'W' => {
                position.1 -= 1;
                dist += 1;
                moved = true;
            }
            '|' => {
                let (old_pos, old_dist) = *main_stack.last().unwrap();
                position = old_pos;
                dist = old_dist;
            }
            '(' => {
                main_stack.push((position, dist));
            }
            ')' => {
                main_stack.pop();
            }
            _ => {}
        }
        if moved && !map.contains_key(&position) {
            map.insert(position, dist);
        }
    }

    let answer = map.values().filter(|&x| *x > 999).count();
    println!("Second star answer: {}", answer);

    Ok(())
}
