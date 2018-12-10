extern crate regex;

use std::error::Error;
use std::fs;
use std::path::Path;

use self::regex::Regex;

pub fn first_star() -> Result<(), Box<Error + 'static>> {
	let input = fs::read_to_string(Path::new("./data/day9.txt")).unwrap();
    let reg = Regex::new(r"(\d+).+?(\d+)").unwrap();
    
    let mut num_of_players = 0;
    let _scores = Vec::<i32>::new();
    let mut max = 0;

    let mut marbles = vec!(0);
    let mut current_posi = 0;

    if let Some(capture) = 	reg.captures(&input) {
    	num_of_players = capture[1].parse::<i32>().unwrap();
    	max = capture[2].parse::<i32>().unwrap();
    	println!("{:?}", capture);	
    }


    for marble in 1..max {
    	current_posi = (current_posi + 2) % (marbles.len() + 1);
    	marbles.insert(current_posi, marble);
    }

    println!("{:?}", marbles);

    Ok(())
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
	Ok(())
}