extern crate regex;

use std::error::Error;

use std::fs;
use std::path::Path;

use self::regex::Regex;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct SpaceTimePoint {
    x: i32,
    y: i32,
    z: i32,
    t: i32
}

impl SpaceTimePoint {
    fn manhattan_dist(&self, other: &SpaceTimePoint) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs() + (self.t - other.t).abs()
    }
}

fn prepare_input() -> Vec<SpaceTimePoint> {
    let input = fs::read_to_string(Path::new("./data/day25.txt")).unwrap();
    let point_reg = Regex::new(r"(-?\d+),(-?\d+),(-?\d+),(-?\d+)").unwrap();

    let mut points = Vec::new();

    for cap in point_reg.captures_iter(&input) {
        points.push(SpaceTimePoint{
            x: cap[1].parse().unwrap(), 
            y: cap[2].parse().unwrap(), 
            z: cap[3].parse().unwrap(), 
            t: cap[4].parse().unwrap()
        });
    }

    points
}

pub fn first_star() -> Result<(), Box<Error + 'static>> {
    let mut points = prepare_input();
    let mut constellations: Vec<Vec<SpaceTimePoint>> = Vec::new();

    while !points.is_empty() {
        let pt = points.pop().unwrap();

        let mut non_matching_constellations = Vec::new();
        let mut matching_constellations = vec![pt];
        while !constellations.is_empty() {
            let mut constellation = constellations.pop().unwrap();
            if constellation.iter().any(|star| star.manhattan_dist(&pt) <= 3) {
                matching_constellations.append(&mut constellation);
            } else {
                non_matching_constellations.push(constellation);
            }
        }
        non_matching_constellations.push(matching_constellations);
        constellations = non_matching_constellations;
    }

    println!("Number of constellations is {}", constellations.len());

    Ok(())
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    println!("Got all stars!");
    Ok(())
}