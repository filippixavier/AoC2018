use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug)]
struct Cart {
    current_dir: (i32, i32),
    next_rotation: Rotate
}

#[derive(Debug)]
enum Rotate {
    Left,
    Straight,
    Right
}

impl Iterator for Cart {
    type Item = (i32, i32);
    fn next(&mut self) -> Option<Self::Item> {
        Some((1, 1))
    }
}

impl Cart {
    fn new(orientation: char) -> Self {
        Cart{current_dir: (0, 1), next_rotation: Rotate::Left}
    }
}

fn get_map() -> Vec<Vec<i32>> {
    let input = fs::read_to_string(Path::new("./data/day12.txt")).unwrap();
    let mut cart_id = 6;
    input.split('\n').map(|line| {
        line.chars().map(|chara| {
            match chara {
                '|' => 1,
                '-' => 2,
                '/' => 3,
                '\\'=> 4,
                '+' => 5,
                '^' | 'v' | '<' | '>' => {
                    cart_id += 1;
                    cart_id - 1
                },
                _ => 0
            }
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>()
}

pub fn first_star() -> Result<(), Box<Error + 'static>> {
    get_map();
    Ok(())
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    Ok(())
}