extern crate regex;

use std::error::Error;

use std::fs;
use std::path::Path;

use self::regex::Regex;

#[derive(Clone, Copy, Debug)]
struct Drone {
    position: (i64, i64, i64),
    radius: u64,
}

fn prepare_input() -> Vec<Drone> {
    let input = fs::read_to_string(Path::new("./data/day23.txt")).unwrap();
    let reg = Regex::new(r"<(-?\d+),(-?\d+),(-?\d+)>.*?(\d+)").unwrap();
    let mut nanodrones = Vec::new();

    for cap in reg.captures_iter(&input) {
        let (pos_x, pos_y, pos_z, radius) = (
            cap[1].parse::<i64>().unwrap(),
            cap[2].parse::<i64>().unwrap(),
            cap[3].parse::<i64>().unwrap(),
            cap[4].parse::<u64>().unwrap(),
        );
        nanodrones.push(Drone {
            position: (pos_x, pos_y, pos_z),
            radius,
        });
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
            return acc + 1;
        }
        acc
    });

    println!("Number of nanobots in range: {}", count);

    Ok(())
}

// Big thanks to this thread: https://www.reddit.com/r/adventofcode/comments/add81t/day_23_part_2_still_stuck_on_finding_bots_in/
// And the links within
fn in_range(area: &Area, drone: &Drone) -> bool {
    let mut closest_point = drone.position;
    if closest_point.0 > area.max_x {
        closest_point.0 = area.max_x;
    } else if closest_point.0 < area.min_x {
        closest_point.0 = area.min_x;
    }

    if closest_point.1 > area.max_y {
        closest_point.1 = area.max_y;
    } else if closest_point.1 < area.min_y {
        closest_point.1 = area.min_y;
    }

    if closest_point.2 > area.max_z {
        closest_point.2 = area.max_z;
    } else if closest_point.2 < area.min_z {
        closest_point.2 = area.min_z;
    }

    manhattan_dist(closest_point, drone.position) <= drone.radius
}

#[derive(Clone, Copy, Debug)]
struct Area {
    min_x: i64,
    min_y: i64,
    min_z: i64,
    max_x: i64,
    max_y: i64,
    max_z: i64,
}

fn check_area(
    area: Area,
    drones_in_range: &[Drone],
    vec_areas: &mut Vec<(Area, Vec<Drone>)>,
    candidate: &mut (i64, i64, i64),
    max_drones: &mut usize,
) {
    let area_drones = drones_in_range
        .iter()
        .cloned()
        .filter(|x| in_range(&area, x))
        .collect::<Vec<Drone>>();

    if !area_drones.is_empty() && area_drones.len() >= *max_drones {
        if area.max_x == area.min_x && area.max_y == area.min_y && area.max_z == area.min_z {
            if *max_drones != area_drones.len() {
                *max_drones = area_drones.len();
                *candidate = (area.min_x, area.min_y, area.min_z);
            }
            if manhattan_dist((0, 0, 0), *candidate)
                > manhattan_dist((0, 0, 0), (area.min_x, area.min_y, area.min_z))
            {
                *candidate = (area.min_x, area.min_y, area.min_z);;
            }
        } else {
            vec_areas.push((area, area_drones));
        }
    }
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    let nanodrones = prepare_input();

    let min_x = nanodrones
        .iter()
        .min_by(|x, y| x.position.0.cmp(&y.position.0))
        .unwrap()
        .position
        .0;
    let min_y = nanodrones
        .iter()
        .min_by(|x, y| x.position.1.cmp(&y.position.1))
        .unwrap()
        .position
        .1;
    let min_z = nanodrones
        .iter()
        .min_by(|x, y| x.position.2.cmp(&y.position.2))
        .unwrap()
        .position
        .2;
    let max_x = nanodrones
        .iter()
        .max_by(|x, y| x.position.0.cmp(&y.position.0))
        .unwrap()
        .position
        .0;
    let max_y = nanodrones
        .iter()
        .max_by(|x, y| x.position.1.cmp(&y.position.1))
        .unwrap()
        .position
        .1;
    let max_z = nanodrones
        .iter()
        .max_by(|x, y| x.position.2.cmp(&y.position.2))
        .unwrap()
        .position
        .2;

    // Algorithm switch: instead of computing every intersecting area, and recursively computing new intersecting area from the previous ones until none can be derived (which I don't know how to do)
    // Let's perform a dichotomial search

    let mut areas: Vec<(Area, Vec<Drone>)> = vec![(
        Area {
            min_x,
            max_x,
            min_y,
            max_y,
            min_z,
            max_z,
        },
        nanodrones,
    )];
    let mut candidate: (i64, i64, i64) = (0, 0, 0);
    let mut max_drones = 0;
    while !areas.is_empty() {
        areas.sort_by(|(_, d_a), (_, d_b)| d_a.len().cmp(&d_b.len()));
        let (area, drones_in_range) = areas.pop().unwrap();

        let len_x = (area.max_x - area.min_x).abs();
        let len_y = (area.max_y - area.min_y).abs();
        let len_z = (area.max_z - area.min_z).abs();

        let mut new_max_drones = max_drones;

        check_area(
            Area {
                min_x: area.min_x,
                max_x: area.min_x + len_x / 2,
                min_y: area.min_y,
                max_y: area.min_y + len_y / 2,
                min_z: area.min_z,
                max_z: area.min_z + len_z / 2,
            },
            &drones_in_range,
            &mut areas,
            &mut candidate,
            &mut new_max_drones,
        );

        if len_x != 0 {
            check_area(
                Area {
                    min_x: area.max_x - len_x / 2,
                    max_x: area.max_x,
                    min_y: area.min_y,
                    max_y: area.min_y + len_y / 2,
                    min_z: area.min_z,
                    max_z: area.min_z + len_z / 2,
                },
                &drones_in_range,
                &mut areas,
                &mut candidate,
                &mut new_max_drones,
            );
        }

        if len_y != 0 {
            check_area(
                Area {
                    min_x: area.min_x,
                    max_x: area.min_x + len_x / 2,
                    min_y: area.max_y - len_y / 2,
                    max_y: area.max_y,
                    min_z: area.min_z,
                    max_z: area.min_z + len_z / 2,
                },
                &drones_in_range,
                &mut areas,
                &mut candidate,
                &mut new_max_drones,
            );
            if len_x != 0 {
                check_area(
                    Area {
                        min_x: area.max_x - len_x / 2,
                        max_x: area.max_x,
                        min_y: area.max_y - len_y / 2,
                        max_y: area.max_y,
                        min_z: area.min_z,
                        max_z: area.min_z + len_z / 2,
                    },
                    &drones_in_range,
                    &mut areas,
                    &mut candidate,
                    &mut new_max_drones,
                );
            }
        }

        if len_z != 0 {
            check_area(
                Area {
                    min_x: area.min_x,
                    max_x: area.min_x + len_x / 2,
                    min_y: area.min_y,
                    max_y: area.min_y + len_y / 2,
                    min_z: area.max_z - len_z / 2,
                    max_z: area.max_z,
                },
                &drones_in_range,
                &mut areas,
                &mut candidate,
                &mut new_max_drones,
            );

            if len_x != 0 {
                check_area(
                    Area {
                        min_x: area.max_x - len_x / 2,
                        max_x: area.max_x,
                        min_y: area.min_y,
                        max_y: area.min_y + len_y / 2,
                        min_z: area.max_z - len_z / 2,
                        max_z: area.max_z,
                    },
                    &drones_in_range,
                    &mut areas,
                    &mut candidate,
                    &mut new_max_drones,
                );
            }

            if len_y != 0 {
                check_area(
                    Area {
                        min_x: area.min_x,
                        max_x: area.min_x + len_x / 2,
                        min_y: area.max_y - len_y / 2,
                        max_y: area.max_y,
                        min_z: area.max_z - len_z / 2,
                        max_z: area.max_z,
                    },
                    &drones_in_range,
                    &mut areas,
                    &mut candidate,
                    &mut new_max_drones,
                );
                if len_x != 0 {
                    check_area(
                        Area {
                            min_x: area.max_x - len_x / 2,
                            max_x: area.max_x,
                            min_y: area.max_y - len_y / 2,
                            max_y: area.max_y,
                            min_z: area.max_z - len_z / 2,
                            max_z: area.max_z,
                        },
                        &drones_in_range,
                        &mut areas,
                        &mut candidate,
                        &mut new_max_drones,
                    );
                }
            }
        }

        if new_max_drones != max_drones {
            max_drones = new_max_drones;
            areas = areas
                .into_iter()
                .filter(|(_, drones)| drones.len() >= max_drones)
                .collect();
        }
    }

    println!(
        "({}, {}, {}), {}, {}",
        candidate.0,
        candidate.1,
        candidate.2,
        candidate.0 + candidate.1 + candidate.2,
        max_drones
    );

    Ok(())
}
