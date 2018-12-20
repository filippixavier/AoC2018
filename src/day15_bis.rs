use std::error::Error;

use std::fs;
use std::path::Path;

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Elf,
    Goblin,
}

struct Character {
    position: usize,
    hp: i32,
    atk: i32,
    clan: Clan,
    id: usize,
}

impl Character {
    fn new(position: usize, clan: Clan) -> Self {
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

fn get_map() -> (Vec<Tile>, i32) {
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

    (map, line_size as i32)
}

fn get_npcs(map: &[Tile]) -> (Vec<Character>, Vec<Character>) {
    let mut elves = Vec::new();
    let mut goblins = Vec::new();

    for (index, tile) in map.iter().enumerate() {
        match tile {
            Tile::Goblin => {
                goblins.push(Character::new(index, Clan::Goblins));
            }
            Tile::Elf => {
                elves.push(Character::new(index, Clan::Elves));
            }
            _ => {}
        }
    }

    (elves, goblins)
}

fn greedy_find_next_position(starts: &[usize], goals: &[usize], map: &[Tile], line_size: i32) -> Option<usize> {
	let mut dists_from_a_goal = HashMap::<usize, usize>::new();
	let mut nodes_to_be_treated = goals.to_vec();
	let mut explored_node = Vec::<usize>::new();
	let mut min_dist: Option<usize> = None;
	let mut candidates = Vec::new();
	let neighbors = &[-line_size, -1, 1, line_size];

	for goal in goals {
		dists_from_a_goal.insert(*goal, 0);
	}

	'main: while !nodes_to_be_treated.is_empty() {
		nodes_to_be_treated.sort_by(|a, b| b.cmp(a));
		let node = nodes_to_be_treated.pop().unwrap();
		if let Some(min) = min_dist {
			if dists_from_a_goal[&node] > min {
				break 'main;
			}
		}
		explored_node.push(node);
		let node_value = dists_from_a_goal[&node];

		for offset in neighbors {
			let neighbor = (node as i32 + offset) as usize;
			if explored_node.contains(&neighbor) || map[neighbor] != Tile::Empty {
				continue;
			}

			if starts.contains(&neighbor) && !dists_from_a_goal.contains_key(&neighbor) {
				min_dist = if let Some(min) = min_dist {
					if min <= node_value {
						Some(min)
					} else {
						Some(node_value)
					}
				} else {
					Some(node_value)
				};
				candidates.push(neighbor);
				continue 'main;
			}

			if let Some(value) = dists_from_a_goal.get(&neighbor) {
				if value >= &node_value {
					continue;
				}
			}

			nodes_to_be_treated.push(neighbor);
			dists_from_a_goal.insert(neighbor, node_value + 1);
		}
	}

	if !candidates.is_empty() {
		candidates.sort_by(|a, b| b.cmp(a));
		return candidates.pop();
	}

	None
}

fn in_range(from: usize, to: usize, line_size: i32) -> bool {
	let dist = ((from as i32) - (to as i32)).abs();
	return dist == 1 || dist == line_size;
}

fn move_attack(attacker: &mut Character, enemies: &mut Vec<Character>, map: &mut Vec<Tile>, line_size: i32) {
    let neighbors = &[-line_size, -1, 1, line_size];
    if enemies.iter().all(|enemy| !in_range(attacker.position, enemy.position, line_size)) {
        let starts = neighbors.iter().map(|n| (n + attacker.position as i32) as usize).filter(|&posi| map[posi] == Tile::Empty).collect::<Vec<usize>>();
        let mut ends = Vec::<usize>::new();

        for enemy in enemies.iter() {
            ends.append(&mut neighbors.iter().map(|n| (n + enemy.position as i32) as usize).filter(|&posi| map[posi] == Tile::Empty).collect::<Vec<usize>>());
        }

        if let Some(new_posi) = greedy_find_next_position(&starts, &ends, map, line_size) {
            map[attacker.position] = Tile::Empty;
            map[new_posi] = match attacker.clan {
                Clan::Goblins => Tile::Goblin,
                Clan::Elves => Tile::Elf
            };
            attacker.position = new_posi;
        }
    }

    let mut is_dead = false;
    let mut swap_remove_index = 0;
    let mut dead_position = 0;
    {
        let attack_on_titan = enemies.iter_mut().enumerate().filter(|(_, enemy)| in_range(attacker.position, enemy.position, line_size)).min_by(|(_, enemy_a), (_, enemy_b)| {
            if enemy_a.hp != enemy_b.hp {
                enemy_a.hp.cmp(&enemy_b.hp)
            } else {
                enemy_a.position.cmp(&enemy_b.position)
            }
        });

        if let Some((index, victim)) = attack_on_titan {
            victim.hp -= attacker.atk;
            if victim.hp <= 0 {
                is_dead = true;
                dead_position = victim.position;
                swap_remove_index = index;
            }
        }
    }
    if is_dead {
        map[dead_position] = Tile::Empty;
        enemies.swap_remove(swap_remove_index);
    }
}

pub fn first_star() -> Result<(), Box<Error + 'static>> { 
	use self::Tile::*;

    let (mut map, line_size) = get_map();
    let (mut elves, mut goblins) = get_npcs(&map);

    let mut round = 0;
    let surviving_team;

    'main: loop {
        // visualize(&map, line_size);
        // Can't iter through map since we need both to alter (so mutable borrow) it and to send it to the a* function (so another borrow within)
        let mut already_moved = HashSet::new();
        for index in 0..map.len() {
            let tile = map[index];
            match tile {
                Goblin => {
                    //Stop condition
                    if elves.is_empty() {
                        surviving_team = goblins;
                        break 'main;
                    }
                    let mut goblin = goblins
                        .iter_mut()
                        .find(|goblin| goblin.position == index)
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
                    println!("{}", elves.len());
                    let mut elf = elves
                        .iter_mut()
                        .find(|elf| {
                            println!("{}, {}", elf.position, index);
                            elf.position == index
                        })
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
	Ok(())
}