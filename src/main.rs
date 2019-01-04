#![allow(dead_code)]
use std::time::Instant;

// Note: look into build.rs to just rewrite mod day1 into mod dayx without all the cfg
mod days;

#[cfg(all(feature = "day1", not(feature = "nday")))]
use days::day1::*;
#[cfg(feature = "day10")]
use days::day10::*;
#[cfg(feature = "day11")]
use days::day11::*;
#[cfg(feature = "day12")]
use days::day12::*;
#[cfg(feature = "day13")]
use days::day13::*;
#[cfg(feature = "day14")]
use days::day14::*;
#[cfg(feature = "day15")]
use days::day15::*;
#[cfg(feature = "day16")]
use days::day16::*;
#[cfg(feature = "day17")]
use days::day17::*;
#[cfg(feature = "day18")]
use days::day18::*;
#[cfg(feature = "day19")]
use days::day19::*;
#[cfg(feature = "day2")]
use days::day2::*;
#[cfg(feature = "day20")]
use days::day20::*;
#[cfg(feature = "day21")]
use days::day21::*;
#[cfg(feature = "day22")]
use days::day22::*;
#[cfg(feature = "day3")]
use days::day3::*;
#[cfg(feature = "day4")]
use days::day4::*;
#[cfg(feature = "day5")]
use days::day5::*;
#[cfg(feature = "day6")]
use days::day6::*;
#[cfg(feature = "day7")]
use days::day7::*;
#[cfg(feature = "day8")]
use days::day8::*;
#[cfg(feature = "day9")]
use days::day9::*;

fn main() {
    let now = Instant::now();
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
    let end = now.elapsed();
    println!("{}.{}", end.as_secs(), end.subsec_millis());
}
