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

    let mut areas: Vec<(Area, Vec<Drone>)> = vec![(Area{min_x, max_x, min_y, max_y, min_z, max_z}, nanodrones.clone())];
    let mut candidates: Vec<(Area, Vec<Drone>)> = Vec::new();
    while candidates.is_empty() {
        areas.sort_by(|(_, drones_a), (_, drones_b)| drones_a.len().cmp(&drones_b.len()));

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

        let first_area_drones;
        let second_area_drones;

        if max == len_z {
            first_area.max_z -= cmp::max(1, len_z / 2);
            second_area.min_z += cmp::max(1, len_z / 2);

            first_area_drones = drones_in_range.iter().cloned().filter(|x| x.position.2 - x.radius as i64 <= first_area.max_z).collect::<Vec<Drone>>();
            second_area_drones = drones_in_range.iter().cloned().filter(|x| x.position.2 + x.radius as i64 >= second_area.min_z).collect::<Vec<Drone>>();

        } else if max == len_y {
            first_area.max_y -= cmp::max(1, len_y / 2);
            second_area.min_y += cmp::max(1, len_y / 2);

            first_area_drones = drones_in_range.iter().cloned().filter(|x| x.position.1 - x.radius as i64 <= first_area.max_y).collect::<Vec<Drone>>();
            second_area_drones = drones_in_range.iter().cloned().filter(|x| x.position.1 + x.radius as i64 >= second_area.min_y).collect::<Vec<Drone>>();

        } else {
            first_area.max_x -= cmp::max(1, len_x / 2);
            second_area.min_x += cmp::max(1, len_x / 2);

            first_area_drones = drones_in_range.iter().cloned().filter(|x| x.position.0 - x.radius as i64 <= first_area.max_x).collect::<Vec<Drone>>();
            second_area_drones = drones_in_range.iter().cloned().filter(|x| x.position.0 + x.radius as i64 >= second_area.min_x).collect::<Vec<Drone>>();
        }

        if !first_area_drones.is_empty() {
            if first_area.max_x == first_area.min_x && first_area.max_y == first_area.min_y && first_area.max_z == first_area.min_z {
                candidates.push((first_area, first_area_drones));
            } else {
                areas.push((first_area, first_area_drones));
            }
        }

        if !second_area_drones.is_empty() {
            if second_area.max_x == second_area.min_x && second_area.max_y == second_area.min_y && second_area.max_z == second_area.min_z {
                candidates.push((second_area, second_area_drones));
            } else {
                areas.push((second_area, second_area_drones));
            }
        }
    }

    candidates.sort_by(|(_, drones_a), (_, drones_b)| drones_a.len().cmp(&drones_b.len()));

    for (zone, bots) in candidates.iter() {
        println!("({}, {}, {}): {}", zone.min_x, zone.min_y, zone.min_z, bots.len());
        for i in nanodrones.iter() {
            if manhattan_dist((zone.min_x, zone.min_y, zone.min_z), i.position) > i.radius {
                println!("Nope! ({}, {}, {}) {} is out of range", i.position.0, i.position.1, i.position.2, i.radius);
            }
        }
    }

    Ok(())
}
