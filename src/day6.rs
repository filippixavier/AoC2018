extern crate regex;

use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::HashMap;

use self::regex::Regex;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Source {
    area: i32,
    infinite: bool,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

pub fn first_star() -> Result<(), Box<Error + 'static>> {
    let input = fs::read_to_string(Path::new("./data/day6.txt"))?;
    let reg = Regex::new(r"(?P<x>\d+), (?P<y>\d+)").unwrap();

    let mut map = HashMap::<Point, (Option<Source>, i32)>::new(); // Each coordinate covered is either owned by a source or shared between sources (None)
    let mut sources = HashMap::<Source, Point>::new();

    let mut progress = Vec::<Point>::new();

    for capture in reg.captures_iter(&input) {
        let (x, y) = (capture.get(1).unwrap().as_str().parse::<i32>().unwrap(), capture.get(2).unwrap().as_str().parse::<i32>().unwrap());
        let new_point = Point{x, y};
        let new_source = Source{area: 1, infinite: false};

        sources.insert(new_source, new_point);
        map.insert(new_point, (Some(new_source), 0));
        progress.push(new_point);
    }

    while progress.len() > 0 {
        let pt = progress.pop().unwrap();
        let current_src_opt;
        let current_dist;
        {
            let (ref_current_src_opt, ref_current_dist) = map.get(&pt).unwrap();
            current_src_opt = *ref_current_src_opt;
            current_dist = *ref_current_dist;
        }

        let (up_point, _down_point, _left_point, _right_point) = (Point{x: pt.x, y: pt.y - 1}, Point{x: pt.x, y: pt.y + 1}, Point{x: pt.x - 1, y: pt.y}, Point{x: pt.x + 1, y: pt.y});
        
        if !is_infinite(current_dist, &up_point, &sources) {
            let map_value = match map.get(&up_point) {
                // Zone déjà réclamé
                Some((claim_src_opt, claim_dist)) => {
                    if *claim_dist > current_dist + 1 {
                        // Zone récupéré par le nouveau point
                        if let Some(mut current_src) = current_src_opt {
                            current_src.area += 1;
                        }
                        if let Some(mut old_src) = claim_src_opt {
                            old_src.area -= 1;
                        }
                        progress.push(up_point);
                        (current_src_opt, current_dist + 1)
                    } else if *claim_dist == current_dist + 1 {
                        //Zone équivalente
                        if let Some(mut old_src) = claim_src_opt {
                            old_src.area -= 1;
                        };
                        (None, *claim_dist)
                    } else {
                        (*claim_src_opt, *claim_dist)
                    }
                },
                None => (current_src_opt, current_dist + 1)
            };
            map.insert(up_point, map_value);
        } else {
            if let Some(mut current_src) = current_src_opt {
                current_src.infinite = true;
            }
        }
    }

    println!("Biggest safe area: {}", 1);

    Ok(())
}


fn claim_cell(pt: &Point, source_opt: &Option<Source>, dist: i32, map: &HashMap<Point, (Option<Source>, i32)>, progress: &Vec<Point>) {
    let map_value = match map.get(pt) {
        // Zone déjà réclamé
        Some((claim_src_opt, claim_dist)) => {
            if *claim_dist > dist + 1 {
                // Zone récupéré par le nouveau point
                if let Some(mut current_src) = source_opt {
                    current_src.area += 1;
                }
                if let Some(mut old_src) = claim_src_opt {
                    old_src.area -= 1;
                }
                progress.push(*pt);
                (source_opt, dist + 1)
            } else if *claim_dist == dist + 1 {
                //Zone équivalente
                if let Some(mut old_src) = claim_src_opt {
                    old_src.area -= 1;
                };
                (None, *claim_dist)
            } else {
                (*claim_src_opt, *claim_dist)
            }
        },
        None => (source_opt, dist + 1)
    };
    map.insert(*pt, map_value);
}


fn is_infinite(start_dist: i32, pt_end: &Point, sources: &HashMap<Source, Point>) -> bool {
    sources.values().all(|&pt| {
        let new_man = (pt_end.x - pt.x).abs() + (pt_end.y - pt.y).abs();
        return new_man > start_dist
    })
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    Ok(())
}