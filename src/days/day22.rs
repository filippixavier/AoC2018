extern crate regex;

use self::regex::Regex;

use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
enum Region {
    Rocky,
    Wet,
    Narrow
}

fn prepare_input() -> [usize;3] {
    let input = fs::read_to_string(Path::new("./data/day22.txt")).unwrap();
    let reg = Regex::new(r"\d+").unwrap();
    let mut values = [0; 3];

    for (index, cap) in reg.captures_iter(&input).enumerate() {
        values[index] = cap[0].parse::<usize>().unwrap();
    } 

    values
}

fn create_map(depth: usize, target: (usize, usize), size: (usize, usize)) -> Vec<u64>{
    let mut map = Vec::<u64>::new();
    let mut erosion_map = Vec::<u64>::new();
    for i in 0..size.0 * size.1{
        let (x, y) = (i % size.0, i / size.0);
        let mut geo_index = match (x, y) {
            (0, 0) => 0,
            (pos_x, 0) => pos_x as u64 * 16807,
            (0, pos_y) => pos_y as u64 * 48271,
            (_, _) => erosion_map[(x - 1) + y * size.0] * erosion_map[x + (y - 1) * size.0]
        };

        if (x, y) == target {
            geo_index = 0;
        }

        let erosion_level = (geo_index + depth as u64) % 20183;
        erosion_map.push(erosion_level);
        map.push(erosion_level % 3);
    }

    map
}

fn visualize(map: &[u64], line_size: usize) {
    let mut txt = "".to_string();
    for (index, i) in map.iter().enumerate() {
        let temp = match i {
            0 => ".",
            1 => "=",
            2 => "|",
            _ => unreachable!()
        };
        txt.push_str(temp);
        if index % line_size == line_size - 1 {
            txt.push('\n');
        }
    }
    println!("{}", txt);
}

fn display_path(path: &HashMap<(usize, usize), (usize, usize)>, target_index: usize) {
    let mut current = (target_index, 1);
    print!("{}, {}", current.0, current.1);

    loop {
        if let Some(pat) = path.get(&current) {
            current = *pat;
            print!(" <- {}, {}", current.0, current.1);
        } else {
            break;
        }
    }

    println!("");
}

fn dijkstra(map: &[u64], line_size: usize, target:(usize, usize)) -> u64 {
    let target_index = target.0 + target.1 * line_size;
    // A bit tough memory wise, but it helps me visualize the algorithm for now
    // Map for no equipement, torch, and climbing
    let maps = [map.iter().map(|&x| x > 0).collect::<Vec<bool>>(), map.iter().map(|&x| x != 1).collect::<Vec<bool>>(), map.iter().map(|&x| x < 2).collect::<Vec<bool>>()];

    let mut visited_nodes = HashSet::<(usize, usize)>::new();
    let mut distance_scores = HashMap::<(usize, usize), u64>::new();

    let mut from_node = HashMap::<(usize, usize), (usize, usize)>::new(); // TO -> FROM

    distance_scores.insert((0, 1), 0);

    let mut visitable_nodes = vec![(0, 1)];

    while !visitable_nodes.is_empty() {
        visitable_nodes.sort_by(|&a, &b| distance_scores[&b].cmp(&distance_scores[&a]));
        let node = visitable_nodes.pop().unwrap();
        visited_nodes.insert(node);
        let dist = *distance_scores.get(&node).unwrap();

        /*if node == (target_index, 1) {
            // display_path(&from_node, target_index);
            return dist;
        }*/

        // Left
        if node.0 % line_size > 0 && maps[node.1][node.0 - 1] {
            let next_node = (node.0 - 1, node.1);
            let next_dist = dist + 1;
            if insert_distance_score(&visited_nodes,&mut visitable_nodes, &mut distance_scores, next_node, next_dist) == true {
                from_node.insert(next_node, node);
            }
        }
        // Right
        if (node.0 + 1) % line_size < line_size - 1 && maps[node.1][node.0 + 1] {
            let next_node = (node.0 + 1, node.1);
            let next_dist = dist + 1;
            if insert_distance_score(&visited_nodes,&mut visitable_nodes, &mut distance_scores, next_node, next_dist) == true {
                from_node.insert(next_node, node);
            }
        }
        // Up
        if node.0 >= line_size && maps[node.1][node.0 - line_size] {
            let next_node = (node.0 - line_size, node.1);
            let next_dist = dist + 1;
            if insert_distance_score(&visited_nodes,&mut visitable_nodes, &mut distance_scores, next_node, next_dist) == true {
                from_node.insert(next_node, node);
            }
        }

        // Down
        if node.0 + line_size < map.len() && maps[node.1][node.0 + line_size] {
            let next_node = (node.0 + line_size, node.1);
            let next_dist = dist + 1;
            if insert_distance_score(&visited_nodes,&mut visitable_nodes, &mut distance_scores, next_node, next_dist) == true {
                from_node.insert(next_node, node);
            }
        }

        // Other maps
        if maps[(node.1 + 1) % 3][node.0] {
            let next_node = (node.0, (node.1 + 1) % 3);
            let next_dist = dist + 7;
            if insert_distance_score(&visited_nodes,&mut visitable_nodes, &mut distance_scores, next_node, next_dist) == true {      
                from_node.insert(next_node, node);
            }
        }

        if maps[(node.1 + 2) % 3][node.0] {
            let next_node = (node.0, (node.1 + 2) % 3);
            let next_dist = dist + 7;
            if insert_distance_score(&visited_nodes,&mut visitable_nodes, &mut distance_scores, next_node, next_dist) == true {
                from_node.insert(next_node, node);
            }
        }
    }

    return *distance_scores.get(&(target_index, 1)).unwrap();
}

fn insert_distance_score(visited_nodes: &HashSet<(usize, usize)>, visitable_nodes: &mut Vec<(usize, usize)>, distance_scores: &mut HashMap<(usize, usize), u64>, node_pos: (usize, usize), node_dist: u64) -> bool {
    if !visited_nodes.contains(&node_pos) {
        if distance_scores.contains_key(&node_pos) {
            if let Some(prev_dist) = distance_scores.get_mut(&node_pos) {
                if *prev_dist > node_dist {
                    *prev_dist = node_dist;
                    return true;
                }
            }
        } else {
            distance_scores.insert(node_pos, node_dist);
            visitable_nodes.push(node_pos);
            return true;
        }
    }
    false
}


pub fn first_star() -> Result<(), Box<Error + 'static>> {
    let values = prepare_input();
    let map = create_map(values[0], (values[1], values[2]), (values[1] + 1, values[2] + 1));
    // visualize(&map, values[1] + 1);
    let answer: u64 = map.iter().sum();
    println!("{}", answer);

    Ok(())
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    let values = prepare_input();
    let map = create_map(values[0], (values[1], values[2]), (values[1] + 6, values[2] + 6));
    // visualize(&map, values[1] + 6);
    let distance = dijkstra(&map, values[1] + 6, (values[1], values[2]));

    println!("Fastest way to the target take {} minutes", distance);
    // 1020 too high
    Ok(())
}