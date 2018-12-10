extern crate regex;

use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::Path;

use self::regex::Regex;

pub fn first_star() -> Result<(), Box<Error + 'static>> {
    let input = fs::read_to_string(Path::new("./data/day3.txt"))?;
    // The starting r negate the need to escape special char in a string
    let re =
        Regex::new(r"(?P<id>\d+) @ (?P<left>\d+),(?P<top>\d+): (?P<width>\d+)x(?P<height>\d+)")
            .unwrap();
    let mut fabric: Vec<Vec<i32>> = vec![vec![0; 1000]; 1000];
    let mut answer = 0;
    // The iterator act as a defacto g flag
    for values in re.captures_iter(&input) {
        let left = values["left"].parse::<usize>().unwrap();
        let width = values["width"].parse::<usize>().unwrap();

        for line in fabric.iter_mut().skip(left).take(width) {
            let top = values["top"].parse::<usize>().unwrap();
            let height = values["height"].parse::<usize>().unwrap();

            for column in line.iter_mut().skip(top).take(height) {
                *column += 1;
                if *column == 2 {
                    answer += 1;
                }
            }
        }
    }

    println!("Overlapping elements are {}", answer);

    Ok(())
}

// Could most probably do both star in on code, and/or use HashMap/HashSet more creatively
pub fn second_star() -> Result<(), Box<Error + 'static>> {
    let input = fs::read_to_string(Path::new("./data/day3.txt"))?;
    let re =
        Regex::new(r"(?P<id>\d+) @ (?P<left>\d+),(?P<top>\d+): (?P<width>\d+)x(?P<height>\d+)")
            .unwrap();
    let mut fabric: Vec<Vec<i32>> = vec![vec![0; 1000]; 1000];
    let mut answer = HashSet::new();
    for values in re.captures_iter(&input) {
        let mut overlap = false;
        let left = values["left"].parse::<usize>().unwrap();
        let width = values["width"].parse::<usize>().unwrap();
        let id = values["id"].parse::<i32>().unwrap();

        for line in fabric.iter_mut().skip(left).take(width) {
            let top = values["top"].parse::<usize>().unwrap();
            let height = values["height"].parse::<usize>().unwrap();

            for column in line.iter_mut().skip(top).take(height) {
                if *column != 0 {
                    overlap = true;
                    answer.remove(column);
                }
                *column = id;
            }
        }

        if !overlap {
            answer.insert(id);
        }
    }

    println!("Non overlapping IDs are: {:?}", answer);

    Ok(())
}
