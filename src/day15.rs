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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

    for line in input.trim().split('\n') {
        line_size = line.chars().count();
        for column in line.trim().chars() {
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
    path.pop();
    path
}

// Roughly translated from the pseudocode found on https://en.wikipedia.org/wiki/A*_search_algorithm#Pseudocode
fn a_star(
    start: Position,
    goal: Position,
    map: &[Tile],
    line_size: usize,
) -> Option<Vec<Position>> {
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
                // Min by is not constant since it returns the first min it encounter in case of equality, and order can't be assurer in a Hashmap
                // Less efficient, but with the wanted constrain
                //.min_by(|v_a, v_b| v_a.cmp(v_b))
                .fold(None, |acc, (key, val)| {
                    if let Some((p_key, p_val)) = acc {
                        if val < p_val {
                            Some((key, val))
                        } else if val == p_val && (key.1 < p_key.1
                            || key.1 == p_key.1 && key.0 < p_key.0) {
                            Some((key, val))
                        } else {
                            Some((p_key, p_val))
                        }
                    } else {
                        Some((key, val))
                    }
                })
                .unwrap();
            current = temp.clone();
        }
        if current == goal {
            return Some(reconstruct_path(came_from, goal));
        }

        open_set.remove(&current);
        closed_set.insert(current);

        for n in neighbors {
            let neighbor = (
                (n.0 + current.0 as i32) as usize,
                (n.1 + current.1 as i32) as usize,
            );

            // Do not consider it if it's an obstacle
            if closed_set.contains(&neighbor)
                || (neighbor != goal && map[neighbor.0 + neighbor.1 * line_size] != Tile::Empty)
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

fn move_attack(attacker: &mut Character, enemies: &mut Vec<Character>, map: &mut Vec<Tile>, line_size: usize) {
    let mut closest_enemy = Vec::new();
    let mut closest_enemy_position = (0, 0);
    /*Move*/
    // Look for closest enemy
    for enemy in enemies.iter() {
        if let Some(path_to_enemy) = a_star(attacker.position, enemy.position, &map, line_size) {
            if closest_enemy.len() == 1 {
                break;
            }
            if closest_enemy.is_empty() || closest_enemy.len() > path_to_enemy.len() {
                closest_enemy = path_to_enemy;
                closest_enemy_position = enemy.position;
            } else if closest_enemy.len() == path_to_enemy.len() {
                if enemy.position.1 < closest_enemy_position.1 || enemy.position.1 == closest_enemy_position.1 && enemy.position.0 == closest_enemy_position.0 {
                    closest_enemy = path_to_enemy;
                    closest_enemy_position = enemy.position;
                }
            }
        }
    }
    // If enemy is close enough but not in range
    if !closest_enemy.is_empty() && closest_enemy.len() > 1 {
        let next_position = closest_enemy.pop().unwrap();
        map[attacker.position.0 + attacker.position.1 * line_size] = Tile::Empty;
        attacker.position = next_position;
        map[attacker.position.0 + attacker.position.1 * line_size] = match attacker.clan {
            Clan::Elves => Tile::Elf,
            Clan::Goblins => Tile::Goblin,
        };
    }
    //ATTACC
    {
        let mut swap_remove_index = 0;
        let mut dead_position = 0;
        let mut is_dead = false;
        {
            let attack_on_titan: Option<(&mut Character, usize)> = enemies.iter_mut().enumerate().filter(|(_, enemy)| manhattan_dist(enemy.position, attacker.position) == 1).fold(None, |acc, (index, enemy)| {
                if let Some((previous, pr_index)) = acc {
                    if enemy.hp < previous.hp 
                    || enemy.position.1 < previous.position.1
                    || enemy.position.1 == previous.position.1 && enemy.position.0 < previous.position.0 {
                        return Some((enemy, index));
                    }
                    return Some((previous, pr_index));
                } else {
                    return Some((enemy, index));
                }
            });
            if let Some((victim, index)) = attack_on_titan {
                victim.hp -= attacker.atk;
                if victim.hp <= 0 {
                    is_dead = true;
                    swap_remove_index = index;
                    dead_position = victim.position.0 + victim.position.1 * line_size;
                }
            }
        }
        if is_dead {
           map[dead_position] = Tile::Empty;
           enemies.swap_remove(swap_remove_index);
        }
    }
}

pub fn first_star() -> Result<(), Box<Error + 'static>> {
    use self::Tile::*;

    let (mut map, line_size) = get_map();
    let (mut elves, mut goblins) = get_npcs(&map, line_size);

    let mut round = 0;
    let surviving_team;

    'main: loop {
        // Can't iter through map since we need both to alter (so mutable borrow) it and to send it to the a* function (so another borrow within) 
        let mut already_moved = HashSet::new();
        for index in 0..map.len() {
            let tile = map[index];
            let position = (index % line_size, index / line_size);
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
                        // Hashing the raw pointer to the object, we can't use the object itself as an hash since it mutate through the game
                    if already_moved.insert(goblin as *const Character) {
                        move_attack(&mut goblin, &mut elves, &mut map, line_size);
                    }
                }
                Elf => {
                    //Stop condition
                    if goblins.is_empty() {
                        surviving_team = elves;
                        break 'main;
                    }
                    let mut elf = elves
                        .iter_mut()
                        .find(|elf| elf.position == position)
                        .unwrap();
                    if already_moved.insert(elf as *const Character) {
                        move_attack(&mut elf, &mut goblins, &mut map, line_size);
                    }
                }
                _ => {}
            }
        }
        round += 1;


        if round == 50 {
            surviving_team = elves;
            break;
        }
    }

    for (index, t) in map.iter().enumerate() {
        let tile = match t {
            Tile::Empty => '.',
            Tile::Goblin => 'G',
            Tile::Elf => 'E',
            Tile::Wall => '#'
        };
        if index >= line_size - 1 && index % line_size == line_size - 1  {
            print!("{}\n", tile);
        } else {
            print!("{}", tile);
        }
    }
    println!("Surviving team: {}, on {} rounds", surviving_team.len(), round);
    Ok(())
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    Ok(())
}
