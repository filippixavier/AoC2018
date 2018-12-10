extern crate regex;

use std::error::Error;
use std::fs;
use std::path::Path;

use self::regex::Regex;

pub fn first_star() -> Result<(), Box<Error + 'static>> {
    let input = fs::read_to_string(Path::new("./data/day9.txt")).unwrap();
    let reg = Regex::new(r"(\d+).+?(\d+)").unwrap();

    let mut num_of_players = 0;
    let mut scores = Vec::<i32>::new();
    let mut max = 0;

    let mut marbles = vec![0];
    let mut current_posi = 0;

    if let Some(capture) = reg.captures(&input) {
        num_of_players = capture[1].parse::<i32>().unwrap();
        max = capture[2].parse::<i32>().unwrap();
        for _ in 0..num_of_players {
            scores.push(0);
        }
    }

    for marble in 1..max {
        if marble % 23 == 0 {
            if let Some(score) = scores.get_mut(((marble - 1) % num_of_players) as usize) {
                *score += marble;
                current_posi = (current_posi + (marbles.len() - 7)) % marbles.len();
                *score += marbles.remove(current_posi);
            }
        } else {
            current_posi = (((current_posi + 1) % marbles.len()) + 1) % (marbles.len() + 1);
            marbles.insert(current_posi, marble);
        }
    }

    if let Some(highscore) = scores.iter().max() {
        println!("highscore is: {}", highscore);
    }

    Ok(())
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    Ok(())
}
