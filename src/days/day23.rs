extern crate regex;

use std::error::Error;

use std::fs;
use std::path::Path;

use self::regex::Regex;

#[derive(Clone, Copy, Debug)]
struct Drone {
	position: (i64, i64, i64),
	radius: u64
}

#[derive(Clone, Copy, Debug)]
struct Field {
	min_x: i64,
	min_y: i64,
	min_z: i64,
	max_x: i64,
	max_y: i64,
	max_z: i64,
}

impl Field {
	fn intersect(&self, other: &Field) -> Option<Field> {
		None
	}
}

impl From<Drone> for Field {
	fn from(drone: Drone) -> Self {
		let min_x = drone.position.0 - drone.radius as i64;
		let min_y = drone.position.1 - drone.radius as i64;
		let min_z = drone.position.2 - drone.radius as i64;

		let max_x = drone.position.0 + drone.radius as i64;
		let max_y = drone.position.1 + drone.radius as i64;
		let max_z = drone.position.2 + drone.radius as i64;

		Field{min_x, max_x, min_y, max_y, min_z, max_z}
	}
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

	println!("({}, {}, {}) {}", nanodrones[1].position.0, nanodrones[1].position.1, nanodrones[1].position.2, nanodrones[1].radius);

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

pub fn second_star() -> Result<(), Box<Error + 'static>> {
	let nanodrones = prepare_input();
	let mut drones_fields = nanodrones.iter().map(|x| Field::from(*x)).collect::<Vec<_>>();





	Ok(())
}
