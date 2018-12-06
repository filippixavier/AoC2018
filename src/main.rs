// Note: look into build.rs to just rewrite mod day1 into mod dayx without all the cfg

#[cfg(all(feature = "day1", not(feature = "nday")))]
mod day1;
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


#[cfg(all(feature = "day1", not(feature = "nday")))]
use day1::*;
#[cfg(feature = "day2")]
use day2::*;
#[cfg(feature = "day3")]
use day3::*;
#[cfg(feature = "day4")]
use day4::*;
#[cfg(feature = "day5")]
mod day5;
#[cfg(feature = "day6")]
mod day6;


fn main() {
    match first_star() {
        Err(x) => { println!("Error: {:?}", x); }
        _ => { println!("First Star: Succeed!"); }
    }
    match second_star() {
        Err(x) => { println!("Error {:?}", x); }
        _ => { println!("Second Star: Succeed!"); }
    }
}
