use std::error::Error;

use std::fs;
use std::path::Path;

use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum State {
    Open,
    Trees,
    Lumberyard,
}

fn prepare_input() -> (Vec<State>, usize) {
    let input = fs::read_to_string(Path::new("../data/day18.txt")).unwrap();
    let mut result = Vec::new();
    let mut count = 0;
    let mut stop_counting = false;
    for ch in input.chars() {
        match ch {
            '.' => result.push(State::Open),
            '#' => result.push(State::Lumberyard),
            '|' => result.push(State::Trees),
            _ => {
                stop_counting = true;
            }
        }
        if !stop_counting {
            count += 1;
        }
    }
    (result, count)
}

fn tile_counting(map: &[State], matching_state: State, index: usize, line_size: usize) -> usize {
    let mut count = 0;
    // left
    if index > 0 && index % line_size > 0 && (index - 1) % line_size == (index % line_size) - 1 {
        if let Some(&tile) = map.get(index - 1) {
            if tile == matching_state {
                count += 1;
            }
        }
    }
    // up-left
    if index > line_size
        && index % line_size > 0
        && (index - 1) % line_size == (index % line_size) - 1
    {
        if let Some(&tile) = map.get(index - line_size - 1) {
            if tile == matching_state {
                count += 1;
            }
        }
    }
    // up
    if index >= line_size {
        if let Some(&tile) = map.get(index - line_size) {
            if tile == matching_state {
                count += 1;
            }
        }
    }
    // up-right
    if index >= line_size
        && index % line_size < line_size - 1
        && (index + 1) % line_size == (index % line_size) + 1
    {
        if let Some(&tile) = map.get(index - line_size + 1) {
            if tile == matching_state {
                count += 1;
            }
        }
    }
    // right
    if (index + 1) % line_size == (index % line_size) + 1 {
        if let Some(&tile) = map.get(index + 1) {
            if tile == matching_state {
                count += 1;
            }
        }
        // down-right
        if let Some(&tile) = map.get(index + line_size + 1) {
            if tile == matching_state {
                count += 1;
            }
        }
    }

    // down
    if let Some(&tile) = map.get(index + line_size) {
        if tile == matching_state {
            count += 1;
        }
    }

    // down-left
    if index > 0 && index % line_size > 0 && (index - 1) % line_size == (index % line_size) - 1 {
        if let Some(&tile) = map.get(index + line_size - 1) {
            if tile == matching_state {
                count += 1;
            }
        }
    }
    count
}

fn evolve(old_state: &[State], line_size: usize) -> Vec<State> {
    use self::State::*;
    let mut new_state = old_state.to_owned();

    for (index, state) in new_state.iter_mut().enumerate() {
        match state {
            Open => {
                let count = tile_counting(&old_state, Trees, index, line_size);
                if count >= 3 {
                    *state = Trees;
                }
            }
            Trees => {
                let count = tile_counting(&old_state, Lumberyard, index, line_size);
                if count >= 3 {
                    *state = Lumberyard;
                }
            }
            Lumberyard => {
                let (count, count_trees) = (
                    tile_counting(&old_state, Lumberyard, index, line_size),
                    tile_counting(&old_state, Trees, index, line_size),
                );
                if count_trees < 1 || count < 1 {
                    *state = Open;
                }
            }
        }
    }

    new_state
}

fn visualize(map: &[State], line_size: usize) {
    let mut txt = "".to_string();
    for (index, i) in map.iter().enumerate() {
        let temp = match i {
            State::Open => ".",
            State::Lumberyard => "#",
            State::Trees => "|",
        };
        txt.push_str(temp);
        if index % line_size == line_size - 1 {
            txt.push('\n');
        }
    }
    println!("{}", txt);
}

pub fn first_star() -> Result<(), Box<Error + 'static>> {
    let (mut map, line_size) = prepare_input();
    for _ in 0..10 {
        map = evolve(&map, line_size);
    }
    let wood = map.iter().filter(|&x| *x == State::Trees).count();
    let lumberyard = map.iter().filter(|&x| *x == State::Lumberyard).count();

    println!("First star result: {}", wood * lumberyard);

    Ok(())
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    let (mut map, line_size) = prepare_input();
    let mut result = map.iter().filter(|&x| *x == State::Trees).count()
        * map.iter().filter(|&x| *x == State::Lumberyard).count();
    let mut map_states = HashMap::new();
    map_states.insert(map.clone(), 0);

    let mut results = Vec::new();
    let mut new_map;
    for i in 0..1000000000 {
        new_map = evolve(&map, line_size);
        result = new_map.iter().filter(|&x| *x == State::Trees).count()
            * new_map.iter().filter(|&x| *x == State::Lumberyard).count();
        if let Some(old_index) = map_states.insert(new_map.clone(), i) {
            let cycle: Vec<_> = results.drain(old_index..).collect();
            // Don't forget the -1: the range is [0, 1000000000[
            result = cycle[(1000000000 - i - 1) % cycle.len()];
            break;
        } else {
            results.push(result);
        }

        map = new_map;
    }
    println!("Second star result: {}", result);

    Ok(())
}
