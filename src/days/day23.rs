extern crate regex;

use std::error::Error;

use std::cmp;

use std::fs;
use std::path::Path;

use self::regex::Regex;

#[derive(Clone, Copy, Debug)]
struct Drone {
    position: (i64, i64, i64),
    radius: u64
}

fn prepare_input() -> Vec<Drone> {
    let input = fs::read_to_string(Path::new("./data/day23.txt")).unwrap();
    let reg = Regex::new(r"<(-?\d+),(-?\d+),(-?\d+)>.*?(\d+)").unwrap();
    let mut nanodrones = Vec::new();

    for cap in reg.captures_iter(&input) {
        let (pos_x, pos_y, pos_z, radius) = (cap[1].parse::<i64>().unwrap(), cap[2].parse::<i64>().unwrap(), cap[3].parse::<i64>().unwrap(), cap[4].parse::<u64>().unwrap());
        nanodrones.push(Drone{position: (pos_x, pos_y, pos_z), radius});
    }

    nanodrones
}

fn manhattan_dist(pos_a: (i64, i64, i64), pos_b: (i64, i64, i64)) -> u64 {
    ((pos_a.0 - pos_b.0).abs() + (pos_a.1 - pos_b.1).abs() + (pos_a.2 - pos_b.2).abs()) as u64
}

pub fn first_star() -> Result<(), Box<Error + 'static>> {
    let mut nanodrones = prepare_input();

    nanodrones.sort_by(|&a, b| a.radius.cmp(&b.radius));

    let nano_leader = nanodrones.last().unwrap();
    let count = nanodrones.iter().fold(0, |acc, drone| {
        if manhattan_dist(nano_leader.position, drone.position) <= nano_leader.radius {
            return acc + 1
        }
        acc
    });

    println!("Number of nanobots in range: {}", count);

    Ok(())
}

#[derive(Clone, Copy, Debug)]
struct Area {
    min_x: i64,
    min_y: i64,
    min_z: i64,
    max_x: i64,
    max_y: i64,
    max_z: i64
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    let nanodrones = prepare_input();

    let min_x = nanodrones.iter().min_by(|x, y| x.position.0.cmp(&y.position.0)).unwrap().position.0;
    let min_y = nanodrones.iter().min_by(|x, y| x.position.1.cmp(&y.position.1)).unwrap().position.1;
    let min_z = nanodrones.iter().min_by(|x, y| x.position.2.cmp(&y.position.2)).unwrap().position.2;
    let max_x = nanodrones.iter().max_by(|x, y| x.position.0.cmp(&y.position.0)).unwrap().position.0;
    let max_y = nanodrones.iter().max_by(|x, y| x.position.1.cmp(&y.position.1)).unwrap().position.1;
    let max_z = nanodrones.iter().max_by(|x, y| x.position.2.cmp(&y.position.2)).unwrap().position.2;

    // Algorithm switch: instead of computing every intersecting area, and recursively computing new intersecting area from the previous ones until none can be derived (which I don't know how to do)
    // Let's perform a dichotomial search

    let mut areas: Vec<(Area, Vec<Drone>)> = vec![(Area{min_x, max_x, min_y, max_y, min_z, max_z}, nanodrones)];
    while !areas.is_empty() {
        let (area, drones_in_range) = areas.pop().unwrap();
        let len_x = (area.max_x - area.min_x).abs();
        let len_y = (area.max_y - area.min_y).abs();
        let len_z = (area.max_z - area.min_z).abs();

        let max;
        {
            let m_1 = cmp::max(len_x, len_y);
            max = cmp::max(len_z, m_1);
        }

        let mut first_area = area;
        let mut second_area = area;

        // let filter_func_a;
        // let filter_func_b;

        if max == len_z {
            first_area.max_z -= len_z / 2;
            second_area.min_z += len_z / 2;
           /* filter_func_a: |x| ;
            filter_func_a: |y| ;*/
        } else if max == len_y {
            first_area.max_y -= len_y / 2;
            second_area.min_y += len_y / 2;
        } else {
            first_area.max_x -= len_x / 2;
            second_area.min_x += len_x / 2;
        }

        /*let first_area_drones = drones_in_range.iter().cloned().filter(filter_func_a).collect::<Vec<Drone>>();
        let second_area_drones = drones_in_range.iter().cloned().filter(filter_func_b).collect::<Vec<Drone>>();*/
    }

    Ok(())
}
