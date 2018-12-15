use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug)]
enum Turn {
    Left,
    Right,
    Straight,
}

#[derive(Copy, Clone, Debug)]
enum Orientation {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Clone, Debug)]
struct Cart {
    position: (usize, usize),
    direction: (i32, i32),
    cross_turn: Turn,
    orientation: Orientation,
}

impl Cart {
    fn turn_left(&mut self) {
        use day13::Orientation::*;
        self.direction = match self.direction {
            (0, 1) => (1, 0),
            (1, 0) => (0, -1),
            (0, -1) => (-1, 0),
            (-1, 0) => (0, 1),
            _ => unreachable!(),
        };

        self.orientation = match self.orientation {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        };
    }

    fn turn_right(&mut self) {
        use day13::Orientation::*;
        self.direction = match self.direction {
            (0, 1) => (-1, 0),
            (-1, 0) => (0, -1),
            (0, -1) => (1, 0),
            (1, 0) => (0, 1),
            _ => unreachable!(),
        };
        self.orientation = match self.orientation {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        };
    }

    fn handle_slash(&mut self, slash: char) {
        use day13::Orientation::*;
        match (&self.orientation, slash) {
            (Up, '/') => self.turn_right(),
            (Up, '\\') => self.turn_left(),
            (Down, '\\') => self.turn_left(),
            (Down, '/') => self.turn_right(),
            (Left, '/') => self.turn_left(),
            (Left, '\\') => self.turn_right(),
            (Right, '/') => self.turn_left(),
            (Right, '\\') => self.turn_right(),
            _ => unreachable!(),
        };
    }

    fn handle_crossroad(&mut self) {
        use day13::Turn::*;
        self.cross_turn = match self.cross_turn {
            Left => {
                self.turn_left();
                Straight
            }
            Straight => Right,
            Right => {
                self.turn_right();
                Left
            }
        };
    }
}

impl Iterator for Cart {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<(usize, usize)> {
        self.position.0 = (self.position.0 as i32 + self.direction.0) as usize;
        self.position.1 = (self.position.1 as i32 + self.direction.1) as usize;
        Some(self.position)
    }
}

fn get_map() -> (
    Vec<Cart>,
    HashMap<(usize, usize), char>,
    HashMap<(usize, usize), char>,
) {
    let input = fs::read_to_string(Path::new("./data/day13.txt")).unwrap();
    let mut carts = Vec::<Cart>::new();
    let mut rails = HashMap::<(usize, usize), char>::new();
    let mut occupied_tiles = HashMap::<(usize, usize), char>::new();

    let lines = input.split('\n');
    for (line_number, line_content) in lines.enumerate() {
        for (char_index, char_value) in line_content.chars().enumerate() {
            match char_value {
                '+' => {
                    rails.insert((char_index, line_number), '+');
                }
                '-' => {
                    rails.insert((char_index, line_number), '-');
                }
                '|' => {
                    rails.insert((char_index, line_number), '|');
                }
                '/' => {
                    rails.insert((char_index, line_number), '/');
                }
                '\\' => {
                    rails.insert((char_index, line_number), '\\');
                }
                '<' => {
                    rails.insert((char_index, line_number), '*');
                    occupied_tiles.insert((char_index, line_number), '-');
                    carts.push(Cart {
                        position: (char_index, line_number),
                        direction: (-1, 0),
                        cross_turn: Turn::Left,
                        orientation: Orientation::Left,
                    });
                }
                '^' => {
                    rails.insert((char_index, line_number), '*');
                    occupied_tiles.insert((char_index, line_number), '|');
                    carts.push(Cart {
                        position: (char_index, line_number),
                        direction: (0, -1),
                        cross_turn: Turn::Left,
                        orientation: Orientation::Up,
                    });
                }
                '>' => {
                    rails.insert((char_index, line_number), '*');
                    occupied_tiles.insert((char_index, line_number), '-');
                    carts.push(Cart {
                        position: (char_index, line_number),
                        direction: (1, 0),
                        cross_turn: Turn::Left,
                        orientation: Orientation::Right,
                    });
                }
                'v' => {
                    rails.insert((char_index, line_number), '*');
                    occupied_tiles.insert((char_index, line_number), '|');
                    carts.push(Cart {
                        position: (char_index, line_number),
                        direction: (0, 1),
                        cross_turn: Turn::Left,
                        orientation: Orientation::Down,
                    });
                }
                _ => {}
            }
        }
    }

    (carts, rails, occupied_tiles)
}

pub fn first_star() -> Result<(), Box<Error + 'static>> {
    let (mut carts, mut rails, mut occupied_tiles) = get_map();
    let crash_zone;

    'collide: loop {
        carts.sort_by(|cart_a, cart_b| {
            if cart_a.position.1 == cart_b.position.1 {
                return cart_a.position.0.cmp(&cart_b.position.0);
            }
            cart_a.position.1.cmp(&cart_b.position.1)
        });
        for cart in carts.iter_mut() {
            // Instead of using another HashMap, it should be possible to store removed tiles in a vector sorted in the same order than the carts
            rails.insert(
                cart.position,
                occupied_tiles.remove(&cart.position).unwrap(),
            );
            let new_pos = cart.next().unwrap();
            let tile = *rails.get(&new_pos).unwrap_or(&'*');
            match tile {
                '*' => {
                    crash_zone = new_pos;
                    break 'collide;
                }
                '+' => {
                    cart.handle_crossroad();
                }
                '/' => {
                    cart.handle_slash('/');
                }
                '\\' => {
                    cart.handle_slash('\\');
                }
                _ => {}
            }
            rails.insert(cart.position, '*');
            occupied_tiles.insert(cart.position, tile);
        }
    }
    println!("Crash in area {}, {}", crash_zone.0, crash_zone.1);
    Ok(())
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    let (mut carts, mut rails, mut occupied_tiles) = get_map();

    while carts.len() > 1 {
        carts.sort_by(|cart_a, cart_b| {
            if cart_a.position.1 == cart_b.position.1 {
                return cart_a.position.0.cmp(&cart_b.position.0);
            }
            cart_a.position.1.cmp(&cart_b.position.1)
        });
        let mut colliding_pos: HashSet<(usize, usize)> = HashSet::new();
        for cart in carts.iter_mut() {
            rails.insert(
                cart.position,
                occupied_tiles.remove(&cart.position).unwrap(),
            );
            if colliding_pos.contains(&cart.position) {
                continue;
            }
            let new_pos = cart.next().unwrap();
            let tile = *rails.get(&new_pos).unwrap_or(&'*');
            match tile {
                '*' => {
                    colliding_pos.insert(new_pos);
                    continue;
                }
                '+' => {
                    cart.handle_crossroad();
                }
                '/' => {
                    cart.handle_slash('/');
                }
                '\\' => {
                    cart.handle_slash('\\');
                }
                _ => {}
            }
            rails.insert(cart.position, '*');
            occupied_tiles.insert(cart.position, tile);
        }
        carts = carts
            .iter()
            .cloned()
            .filter(|cart| {
                if colliding_pos.contains(&cart.position) {
                    if let Some(tile) = occupied_tiles.remove(&cart.position) {
                        rails.insert(cart.position, tile);
                    }
                    return false;
                }
                true
            })
            .collect::<_>();
    }
    let cart_posi = if let Some(cart) = carts.pop() {
        cart.position
    } else {
        (0, 0)
    };

    println!("Last cart standing: {}, {}", cart_posi.0, cart_posi.1);
    Ok(())
}
