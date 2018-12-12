use std::error::Error;
use std::io::{self, Read};

use std::collections::HashMap;

fn get_cell_power(x: i32, y: i32, serial: i32) -> i32{
    let rack_id = x + 10;
    let temp_power = ((rack_id * y) + serial) * rack_id;
    temp_power.to_string().chars().rev().nth(2).unwrap_or('0').to_string().parse::<i32>().unwrap_or(0) - 5
}

pub fn first_star() -> Result<(), Box<Error + 'static>> {
    let mut max_power = 0;
    let serial;
    let mut memoize = HashMap::<(i32, i32), i32>::new();
    let mut buffer = String::new();
    let mut answer = [0; 2];
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {;
            serial = buffer.trim().parse::<i32>().unwrap_or(0);
        },
        Err(error) => {
            serial = 0;
            println!("Error: {}", error)
        },
    }

    for x in 1..298 {
        for y in 1..298 {
            let mut square_power = 0;
            for i in x..x+3 {
                for j in y..y+3 {
                    square_power += *memoize.entry((i, j)).or_insert_with(|| get_cell_power(i, j, serial));
                }
            }
            if square_power > max_power {
                max_power = square_power;
                answer[0] = x;
                answer[1] = y;
            }
        }
    }
    println!("Answer is: {:?} with power: {}", answer, max_power);
    Ok(())
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    Ok(())
}
