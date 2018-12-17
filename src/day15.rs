use std::error::Error;

use std::fs;
use std::path::Path;

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
struct Character {
    position: (usize, usize),
    hp: i32,
    atk: i32,
    clan: Clan,
}

impl Character {
    fn new(position: (usize, usize), clan: Clan) -> Self {
        Character {
            position,
            clan,
            hp: 200,
            atk: 3,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Clan {
    Goblins,
    Elves,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Elf,
    Goblin,
}

type Map = Vec<Tile>;

fn get_map() -> (Vec<Tile>, usize) {
    use self::Tile::*;

    let input = fs::read_to_string(Path::new("./data/day15.txt")).unwrap();
    let mut map = Vec::<Tile>::new();
    let mut line_size = 0;

    for (line_number, line) in input.trim().split('\n').enumerate() {
        line_size = line.chars().count();
        for (column_number, column) in line.chars().enumerate() {
            match column {
                '#' => {
                    map.push(Wall);
                }
                '.' => {
                    map.push(Empty);
                }
                'G' => {
                    map.push(Goblin);
                }
                'E' => {
                    map.push(Elf);
                }
                _ => unreachable!(),
            }
        }
    }

    (map, line_size)
}

fn get_npcs(map: &Map, line_size: usize) -> (Vec<Character>, Vec<Character>) {
    let mut elves = Vec::new();
    let mut goblins = Vec::new();

    for (index, tile) in map.iter().enumerate() {
        let position = (index % line_size, index / line_size);
        match tile {
            Tile::Goblin => {
                goblins.push(Character::new(position, Clan::Goblins));
            }
            Tile::Elf => {
                elves.push(Character::new(position, Clan::Elves));
            }
            _ => {}
        }
    }

    (elves, goblins)
}
#[derive(Debug, Copy, Clone)]
struct Node {
    c: usize,
    y: usize,
    heuristic: usize,
    cost: usize,
}

type Position = (usize, usize);

// Our Heuristic function, go from a to b by taking the shortest path without diagonals
fn manhattan_dist(start: Position, end: Position) -> i32 {
    (start.0 as i32 - end.0 as i32).abs() + (start.1 as i32 - end.1 as i32).abs()
}

fn reconstruct_path(came_from: HashMap<Position, Position>, goal: Position) -> Vec<Position> {
    let mut path = vec![goal];
    let mut current_node = goal;
    while came_from.contains_key(&current_node) {
        current_node = came_from[&current_node];
        path.push(current_node);
    }
    path.reverse();
    path
}

// Roughly translated from the pseudocode found on https://en.wikipedia.org/wiki/A*_search_algorithm#Pseudocode
fn a_star(
    start: Position,
    goal: Position,
    map: &[Tile],
    line_size: usize,
) -> Option<(Vec<Position>, HashMap<Position, i32>)> {
    let mut closed_set = HashSet::<Position>::new();
    let mut open_set = HashSet::<Position>::new();
    open_set.insert(start);

    let mut came_from = HashMap::<Position, Position>::new();

    // g_score Map the cost of getting from the start node to the key node
    let mut g_score = HashMap::<Position, i32>::new();
    g_score.insert(start, 0);

    // f_score is the total cost of getting from start to goal passing by that node
    let mut f_score = HashMap::<Position, i32>::new();
    f_score.insert(start, manhattan_dist(start, goal));

    let neighbors = &[(0, -1), (-1, 0), (1, 0), (0, 1)];

    while !open_set.is_empty() {
        let current;
        // The scope is here to allow the later insertion into f_score, otherwise we would have a dangling borrow to f_score preventing us to do so
        {
            let (temp, _) = f_score
                .iter()
                .filter(|(key, _)| open_set.contains(key))
                .max_by(|(_, val_a), (_, val_b)| val_a.cmp(val_b))
                .unwrap();
            current = temp.clone();
        }

        if current == goal {
            return Some((reconstruct_path(came_from, goal), f_score));
        }

        open_set.remove(&current);
        closed_set.insert(current);

        for n in neighbors {
            if current.0 as i32 + n.0 < 0 || current.1 as i32 + n.1 < 0 {
                continue;
            }
            let neighbor = (
                (n.0 + current.0 as i32) as usize,
                (n.1 + current.1 as i32) as usize,
            );

            // Do not consider it if it's an obstacle
            if closed_set.contains(&neighbor)
                || map[neighbor.0 + neighbor.1 * line_size] != Tile::Empty
            {
                continue;
            }
            // Distance to neighbor is constant as we only look at direct neighbors
            let tentative_g_score = g_score[&current] + 1;

            if !open_set.contains(&neighbor) {
                open_set.insert(neighbor);
            } else if tentative_g_score >= g_score[&neighbor] {
                continue;
            }

            came_from.insert(neighbor, current);
            g_score.insert(neighbor, tentative_g_score);
            f_score.insert(neighbor, tentative_g_score + manhattan_dist(neighbor, goal));
        }
    }
    None
}

pub fn first_star() -> Result<(), Box<Error + 'static>> {
    use self::Tile::*;

    let (mut map, line_size) = get_map();
    let (mut elves, mut goblins) = get_npcs(&map, line_size);

    let mut round = 0;
    let mut surviving_team;

    let neighbors = &[(0, -1), (-1, 0), (1, 0), (0, 1)];

    'main: loop {
        // Can't iter through map since we need both to alter (so mutable borrow) it and to send it to the a* function (so another borrow within) 
        for index in 0..map.len() {
            let tile = map[index];
            let position = (index % line_size, index / line_size);
            let mut closest_enemy = Vec::new();
            let mut closest_enemy_position = (0, 0);
            let mut closest_enemy_map = HashMap::new();
            match tile {
                Goblin => {
                    //Stop condition
                    if elves.is_empty() {
                        surviving_team = goblins;
                        break 'main;
                    }
                    let mut goblin = goblins
                        .iter_mut()
                        .find(|goblin| goblin.position == position)
                        .unwrap();
                    /*Move*/
                    // Look for closest elf
                    for elf in &elves {
                        if let Some((path_to_enemy, f_map)) = a_star(goblin.position, elf.position, &map, line_size) {
                            if closest_enemy.len() == 1 {
                                break;
                            }
                            if closest_enemy.is_empty() || closest_enemy.len() > path_to_enemy.len() {
                                closest_enemy = path_to_enemy;
                                closest_enemy_position = elf.position;
                                closest_enemy_map = f_map;
                            } else if closest_enemy.len() == path_to_enemy.len() {
                                if elf.position.1 < closest_enemy_position.1 || elf.position.1 == closest_enemy_position.1 && elf.position.0 == closest_enemy_position.0 {
                                    closest_enemy = path_to_enemy;
                                    closest_enemy_position = elf.position;
                                }
                            }
                        }
                    }
                    // If elf is close enough but not in range
                    if !closest_enemy.is_empty() {
                        let mut next_position = (999, 999);
                        let mut distance = 999;
                        for n in neighbors {
                            if goblin.position.0 as i32 + n.0 < 0 || goblin.position.1 as i32 + n.1 < 0 {
                                continue;
                            }
                            let neighbor = (
                                (n.0 + goblin.position.0 as i32) as usize,
                                (n.1 + goblin.position.1 as i32) as usize,
                            );
                            if let Some(dist) = closest_enemy_map.get(&neighbor) {
                                if dist < &distance {
                                    distance = *dist;
                                    next_position = neighbor;
                                }
                            }
                        }

                        map.insert(goblin.position.0 + goblin.position.1 * line_size, Tile::Empty);
                        goblin.position = next_position;
                        map.insert(goblin.position.0 + goblin.position.1 * line_size, Tile::Goblin);
                    }
                    //ATTACC
                    {
                        let mut attack_on_titanelf: Option<Character> = None;
                        for n in neighbors {
                            if goblin.position.0 as i32 + n.0 < 0 || goblin.position.1 as i32 + n.1 < 0 {
                                continue;
                            }
                            let neighbor = (
                                (n.0 + goblin.position.0 as i32) as usize,
                                (n.1 + goblin.position.1 as i32) as usize,
                            );
                            if let Some(elf) = elves.iter().find(|elf| elf.position == neighbor) {
                                if let Some(victim) = attack_on_titanelf {
                                    if victim.hp < elf.hp {
                                        attack_on_titanelf = Some(*elf);
                                    }
                                } else {
                                    attack_on_titanelf = Some(*elf);
                                }
                            }
                        }
                        if let Some(mut victim) = attack_on_titanelf {
                            victim.hp -= goblin.atk;
                            if victim.hp <= 0 {
                                // ELF BE DEAD x_x
                            }
                        }
                    }
                }
                Elf => {
                    if goblins.is_empty() {
                        surviving_team = elves;
                        break 'main;
                    }
                    let mut elf = elves
                        .iter_mut()
                        .find(|elf| elf.position == position)
                        .unwrap();
                    //Stop condition
                }
                _ => {}
            }
        }

        round += 1;
    }

    Ok(())
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    Ok(())
}
