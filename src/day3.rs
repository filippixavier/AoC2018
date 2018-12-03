extern crate regex;

use std::fs;
use std::path::Path;

use regex::Regex;

fn main() -> Result<(), Box<Error + 'static>>{
    let input = fs::read_to_string(Path::new("./data/day3.txt"))?;
    // The starting r negate the need to escape special char in a string
    // Also: no global flag, so we need to create a more complete regex
    let re = Regex::new(r"(?P<id>\d+) @ (?P<left>\d+),(?P<top>\d+): (?P<width>\d+)x(?P<height>\d+)").unwrap();
    let mut fabric: Vec<Vec<i32>> = vec![vec![0;1000];1000];
    let mut answer = 0;
    for values in re.captures_iter(&input) {
        let left = values["left"].parse::<usize>().unwrap();
        let width = values["width"].parse::<usize>().unwrap();
        for i in left..left + width {
            let top = values["top"].parse::<usize>().unwrap();
            let height = values["height"].parse::<usize>().unwrap();
            for j in top..top + height {
                fabric[i][j] += 1;
                if fabric[i][j] == 2 {
                    answer += 1;
                }
            }
        }
    }

    println!("Overlapping elements are {}", answer);

    Ok(())
}