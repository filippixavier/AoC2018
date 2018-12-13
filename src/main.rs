// Note: look into build.rs to just rewrite mod day1 into mod dayx without all the cfg

#[cfg(all(feature = "day1", not(feature = "nday")))]
mod day1;
#[cfg(feature = "day10")]
mod day10;
#[cfg(feature = "day11")]
mod day11;
#[cfg(feature = "day12")]
mod day12;
#[cfg(feature = "day2")]
mod day2;
#[cfg(feature = "day3")]
mod day3;
#[cfg(feature = "day4")]
mod day4;
#[cfg(feature = "day5")]
mod day5;
#[cfg(feature = "day6")]
mod day6;
#[cfg(feature = "day7")]
mod day7;
#[cfg(feature = "day8")]
mod day8;
#[cfg(feature = "day9")]
mod day9;

#[cfg(all(feature = "day1", not(feature = "nday")))]
use day1::*;
#[cfg(feature = "day10")]
use day10::*;
#[cfg(feature = "day11")]
use day11::*;
#[cfg(feature = "day12")]
use day12::*;
#[cfg(feature = "day2")]
use day2::*;
#[cfg(feature = "day3")]
use day3::*;
#[cfg(feature = "day4")]
use day4::*;
#[cfg(feature = "day5")]
use day5::*;
#[cfg(feature = "day6")]
use day6::*;
#[cfg(feature = "day7")]
use day7::*;
#[cfg(feature = "day8")]
use day8::*;
#[cfg(feature = "day9")]
use day9::*;

fn main() {
    match first_star() {
        Err(x) => {
            println!("Error: {:?}", x);
        }
        _ => {
            println!("First Star: Succeed!");
        }
    }
    match second_star() {
        Err(x) => {
            println!("Error {:?}", x);
        }
        _ => {
            println!("Second Star: Succeed!");
        }
    }
}
