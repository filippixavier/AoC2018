extern crate regex;

use std::error::Error;
use std::fs;
use std::path::Path;
use std::cmp::*;

use std::collections::HashMap;

use self::regex::Regex;

pub fn first_star() -> Result<(), Box<Error + 'static>> {
    let input = fs::read_to_string(Path::new("./data/day6.txt"))?;
    let reg = Regex::new(r"(?P<x>\d+), (?P<y>\d+)").unwrap();
    
    // Our map and a list of discovered location
    let mut map = HashMap::<(i32, i32), (String, i32)>::new();
    let mut ids = HashMap::<String, (i32, i32)>::new();
    let mut area_h = HashMap::<String, i32>::new();
    
    //Iterating over input's capture match (one match = one input)
    for cap in reg.captures_iter(input.as_str()) {
        let new_pt_id = cap.get(0).unwrap().as_str();
        let (x, y) = (cap.get(1).unwrap().as_str().parse::<i32>().unwrap(), cap.get(2).unwrap().as_str().parse::<i32>().unwrap());
        let mut area = 1;

        // Iterating over each already discovered points
        for (other_id, (other_x, other_y)) in ids.iter() {
            let (start_x, end_x) = (min(x, *other_x), max(x, *other_x));
            let (start_y, end_y) = (min(y, *other_y), max(y, *other_y));
            // Iterating over the area between a discovered point and the new point
            for temp_x in start_x..end_x + 1 {
                for temp_y in start_y..end_y + 1 {
                    let manhattan_dist = (temp_x - x).abs() + (temp_y - y).abs();
                    let other_manhattan_dist = (temp_x - *other_x).abs() + (temp_y - *other_y).abs();

                    //Systematically recalculate manhattan distance
                    let (temp_id, temp_dist) = match map.get(&(temp_x, temp_y)) {
                        Some((claiming_id, claiming_dist)) => {
                            if *claiming_dist > manhattan_dist {
                                if let Some(previous_area) = area_h.get_mut(claiming_id) {
                                    *previous_area -= 1;
                                }
                                area += 1;
                                (new_pt_id.to_string(), manhattan_dist)
                            } else if *claiming_dist == manhattan_dist {
                                if let Some(previous_area) = area_h.get_mut(claiming_id) {
                                    *previous_area -= 1;
                                }
                                let result_id = ".".to_string();
                                (result_id, *claiming_dist)
                            } else {
                                (claiming_id.clone(), *claiming_dist)
                            }
                        }
                        None => {
                            if manhattan_dist > other_manhattan_dist {
                                if let Some(previous_area) = area_h.get_mut(other_id) {
                                    *previous_area += 1;
                                }
                                (other_id.clone(), other_manhattan_dist)
                            } else if manhattan_dist == other_manhattan_dist {
                                (".".to_string(), manhattan_dist)
                            } else {
                                area += 1;
                                (new_pt_id.to_string(), manhattan_dist)
                            }
                        }
                    };
                    map.insert((temp_x, temp_y), (temp_id, temp_dist));
                }
            }
        }
        ids.insert(new_pt_id.to_string(), (x, y));
        area_h.insert(new_pt_id.to_string(), area);
    }
    let mut answer = 0;

    for val in area_h.values() {
        answer = max(answer, *val);
    }

    println!("Biggest safe area: {}", answer);

    Ok(())
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    Ok(())
}