use std::error::Error;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<Error + 'static>> {
    // Well that was painful: the file contain space to trim before parsing along with a final empty character, hence the unwrap_or
    println!("{}", fs::read_to_string(Path::new("./data/day1.txt"))?.split('\n').map( |x| x.trim().parse::<i32>().unwrap_or(0)).fold(0, |acc, x| x + acc));
    Ok(())
}