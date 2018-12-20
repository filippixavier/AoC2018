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
    id: (usize, usize),
}

impl Character {
    fn new(position: (usize, usize), clan: Clan) -> Self {
        Character {
            position,
            clan,
            hp: 200,
            atk: 3,
            id: position,
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

fn get_npcs(map: &[Tile], line_size: usize) -> (Vec<Character>, Vec<Character>) {
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

type Position = (usize, usize);

// Our Heuristic function, go from a to b by taking the shortest path without diagonals
fn manhattan_dist(start: Position, end: Position) -> i32 {
    (start.0 as i32 - end.0 as i32).abs() + (start.1 as i32 - end.1 as i32).abs()
}

fn reconstruct_path(came_from: &HashMap<usize, usize>, goal: usize) -> Vec<usize> {
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
fn all_star(
    start: Position,
    goals: &[Position],
    map: &[Tile],
    line_size: usize,
) -> Option<Vec<Position>> {
    let start_index = start.0 + start.1 * line_size;
    let goals_index: Vec<usize> = goals.iter().map(|pos| pos.0 + pos.1 * line_size).collect();

    let mut closed_set = HashSet::<usize>::new();
    let mut open_set = HashSet::<usize>::new();
    open_set.insert(start_index);

    let mut came_from = HashMap::<usize, usize>::new();

    // g_score Map the cost of getting from the start node to the key node
    let mut g_score = HashMap::<usize, i32>::new();
    g_score.insert(start_index, 0);

    // f_score is the total cost of getting from start to goal passing by that node
    let mut f_score = HashMap::<usize, f32>::new();
    let mut possible_goal = Vec::<usize>::new();
    f_score.insert(start_index, 0.0);

    let neighbors = &[(-(line_size as i32), 0.04), (-1, 0.03), (1, 0.02), (line_size as i32, 0.01)];

    while !open_set.is_empty() {
        let current;
        // The scope is here to allow the later insertion into f_score, otherwise we would have a dangling borrow to f_score preventing us to do so
        {
            let (temp, _) = f_score
                .iter()
                .filter(|(key, _)| open_set.contains(key))
                // .min_by(|(_, &val), (_, &val_b)| val.partial_cmp(&val_b).unwrap_or(std::cmp::Ordering::Equal))
                // Min by is not constant since it returns the first min it encounter in case of equality, and order can't be assurer in a Hashmap
                // Less efficient, but with the wanted constrain
                .fold(None, |acc: Option<(usize, f32)>, (index, val)| {
                    if let Some((p_index, p_val)) = acc {
                        if *val < p_val || (*val == p_val && index < &p_index)
                        {
                            Some((*index, *val))
                        } else {
                            Some((p_index, p_val))
                        }
                    } else {
                        Some((*index, *val))
                    }
                })
                .unwrap();
            current = temp;
        }
        open_set.remove(&current);
        closed_set.insert(current);

        if goals_index.contains(&current) {
            possible_goal.push(current);
            if possible_goal.len() == goals_index.len() {
                break;
            }
            continue;
        }

        for n in neighbors {
            let neighbor = (current as i32 + n.0) as usize;

            // Do not consider it if it's an obstacle
            if closed_set.contains(&neighbor)
                || (!goals_index.contains(&neighbor)
                    && map[neighbor] != Tile::Empty)
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
            f_score.insert(neighbor, (tentative_g_score as f32) - n.1);
        }
    }
    {
        if !possible_goal.is_empty() {
            let paths: Vec<Vec<usize>> = possible_goal.iter().cloned().map(|goal| reconstruct_path(&came_from, goal)).collect();
            let min = paths.iter().min_by(|va, vb| va.len().cmp(&vb.len())).unwrap().len();
            let min_path_index = paths.iter().cloned().filter(|path| path.len() == min).min_by(|va, vb| {
                let (end_a, end_b) = (va.last().unwrap(), vb.last().unwrap());
                return end_a.cmp(end_b);
            });

            if let Some(min_path) = min_path_index {
                return Some(min_path.iter().cloned().map(|index| (index % line_size, index / line_size)).collect::<Vec<Position>>());
            }
        }
    }
    None
}

fn move_attack(
    attacker: &mut Character,
    enemies: &mut Vec<Character>,
    map: &mut Vec<Tile>,
    line_size: usize,
) {
    let mut closest_enemy = Vec::new();
    /*Move*/
    // Look for closest enemy
    // Took the all_star idea on a js code by albertobastos
    let enemies_positions: Vec<Position> = enemies.iter().map(|e| e.position).collect();
    if let Some(result) = all_star(attacker.position, &enemies_positions, &map, line_size) {
        closest_enemy = result;
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
            let attack_on_titan: Option<(&mut Character, usize)> = enemies
                .iter_mut()
                .enumerate()
                .filter(|(_, enemy)| manhattan_dist(enemy.position, attacker.position) == 1)
                .fold(None, |acc, (index, enemy)| {
                    if let Some((previous, pr_index)) = acc {
                        if enemy.hp < previous.hp
                            || enemy.hp == previous.hp
                                && (enemy.position.1 < previous.position.1
                                    || enemy.position.1 == previous.position.1
                                        && enemy.position.0 < previous.position.0)
                        {
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
        // visualize(&map, line_size);
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
                    // We can't use raw pointer either: can mutate when removing dead character, meaning false positive when character is trying to take its turn
                    if already_moved.insert(goblin.id) {
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
                    if already_moved.insert(elf.id) {
                        move_attack(&mut elf, &mut goblins, &mut map, line_size);
                    }
                }
                _ => {}
            }
        }
        round += 1
    }

    // visualize(&map, line_size);
    let total_hp = surviving_team.iter().fold(0, |acc, survivor| {
        acc + survivor.hp
    });
    println!("Score: {} in {} rounds", total_hp * round, round);
    Ok(())
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    /*use self::Tile::*;

    let (map, line_size) = get_map();
    let (elves, goblins) = get_npcs(&map, line_size);

    let mut round;
    let mut base_elf_power = 4;
    let surviving_team;

    'main: loop {
        base_elf_power += 1;
        let (mut elves_sub, mut goblins_sub, mut map_sub) = (elves.clone(), goblins.clone(), map.clone());
        for elf in elves_sub.iter_mut() {
            elf.atk = base_elf_power;
        }
        round = 0;
        'battle: loop {
            // Can't iter through map since we need both to alter (so mutable borrow) it and to send it to the a* function (so another borrow within)
            let mut already_moved = HashSet::new();
            for index in 0..map_sub.len() {
                let tile = map_sub[index];
                let position = (index % line_size, index / line_size);
                match tile {
                    Goblin => {
                        let mut goblin = goblins_sub
                            .iter_mut()
                            .find(|goblin| goblin.position == position)
                            .unwrap();
                        // Hashing the raw pointer to the object, we can't use the object itself as an hash since it mutate through the game
                        // We can't use raw pointer either: can mutate when removing dead character, meaning false positive when character is trying to take its turn
                        if already_moved.insert(goblin.id) {
                            move_attack(&mut goblin, &mut elves_sub, &mut map_sub, line_size);
                            if elves_sub.len() != elves.len() {
                                continue 'main;
                            }
                        }
                    }
                    Elf => {
                        //Stop condition
                        if goblins_sub.is_empty() {
                            if elves_sub.len() == elves.len() {
                                surviving_team = elves_sub;
                                visualize(&map_sub, line_size);
                                break 'main;
                            }
                        }
                        let mut elf = elves_sub
                            .iter_mut()
                            .find(|elf| elf.position == position)
                            .unwrap();
                        if already_moved.insert(elf.id) {
                            move_attack(&mut elf, &mut goblins_sub, &mut map_sub, line_size);
                        }
                    }
                    _ => {}
                }
            }
            round += 1;
        }
    }
    
    let total_hp = surviving_team.iter().fold(0, |acc, survivor| {
        acc + survivor.hp
    });
    println!("Score: {} in {} rounds with {} atk", total_hp * round, round, base_elf_power);
    // 67595 too high
    // 67069 too high
    // 61490 not correct? 
    // 58753 too low*/

    Ok(())
}

fn visualize(map: &[Tile], line_size: usize) {
    let mut txt = "".to_string();
    for (index, i) in map.iter().enumerate() {
        let temp = match i {
            Tile::Empty => ".",
            Tile::Wall => "#",
            Tile::Goblin => "G",
            Tile::Elf => "E"
        };
        txt.push_str(temp);
        if index % line_size == line_size - 1 {
            txt.push('\n');
        }
    }
    println!("{}", txt);
}
