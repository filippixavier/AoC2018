extern crate regex;

use std::fs;
use std::path::Path;

use self::regex::Regex;

use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug)]
enum Region {
    Rocky,
    Wet,
    Narrow
}

fn prepare_input() -> [usize; 3] {
    let input = fs::read_to_string(Path::new("./data/day22.txt")).unwrap();
    let reg = Regex::new(r"\d+").unwrap();
    let mut values = [0; 3];

    for (index, cap) in reg.captures_iter(&input).enumerate() {
        values[index] = cap[0].parse::<usize>().unwrap();
    }

    // values[1] += 1;
    // values[2] += 1;

    values
}

fn create_map(depth: usize, target: (usize, usize)) -> Vec<usize> {
    let mut map = Vec::<usize>::new();
    let mut erosion_map = Vec::<u64>::new();
    let line_size = target.0 + 1;
    let line_number = target.1 + 1;

    for index in 0..line_size * line_number {
        let (x, y) = (index % line_size, index / line_size);
        let geo_index: u64 = match (x, y) {
            (0, 0) => 0,
            (pos_x, 0) => pos_x as u64 * 16807,
            (0, pos_y) => pos_y as u64 * 48271,
            (pos_x, pos_y) => erosion_map[pos_x - 1 + pos_y * line_size] * erosion_map[pos_x + (pos_y - 1) * line_size]
        };
        let erosion_level = (geo_index + depth as u64) % 20183;
        erosion_map.push(erosion_level);
        map.push(erosion_level as usize % 3);
    }
    *map.last_mut().unwrap() = 0;

    map
}


fn visualize(map: &[usize], line_size: usize) {
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


pub fn first_star() -> Result<(), Box<Error + 'static>> {
    let values = prepare_input();
    let map = create_map(values[0], (values[1], values[2]), (values[1] + 1, values[2] + 1));
    // visualize(&map, values[1] + 1);
    let answer: u64 = map.iter().sum();
    println!("{}", answer);

    Ok(())
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    Ok(())
}