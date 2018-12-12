use std::error::Error;
use std::io;

use std::collections::HashMap;

fn get_serial() -> i32 {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {;
            buffer.trim().parse::<i32>().unwrap_or(0)
        },
        Err(error) => {
            println!("Error: {}", error);
            0
        },
    }
}

fn get_cell_power(x: i32, y: i32, serial: i32) -> i32{
    let rack_id = x + 10;
    let temp_power = ((rack_id * y) + serial) * rack_id;
    temp_power.to_string().chars().rev().nth(2).unwrap_or('0').to_string().parse::<i32>().unwrap_or(0) - 5
}

fn get_max_area(serial: i32, size: i32, memoize: &mut HashMap<(i32, i32), i32>) -> [i32; 3] {
    let mut answer = [0, 0, 0];
    for x in 1..298 {
        for y in 1..298 {
            let mut square_power = 0;
            for i in x..x+size {
                for j in y..y+size {
                    square_power += *memoize.entry((i, j)).or_insert_with(|| get_cell_power(i, j, serial));
                }
            }
            if square_power > answer[2] {
                answer[0] = x;
                answer[1] = y;
                answer[2] = square_power;
            }
        }
    }
    answer
}

pub fn first_star() -> Result<(), Box<Error + 'static>> {
    let mut memoize = HashMap::<(i32, i32), i32>::new();
    let serial = get_serial();
    let answer;
    answer = get_max_area(serial, 3, &mut memoize);

    println!("Answer is: {}, {} with power: {}", answer[0], answer[1], answer[2]);
    Ok(())
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    let mut memoize = HashMap::<(i32, i32), i32>::new();
    let serial = get_serial();
    let mut answer = [0; 3];
    for size in 1..301 {
        let temp = get_max_area(serial, size, &mut memoize);
        if temp[2] > answer[2] {
            answer = temp;
        }
    }

    println!("Answer is: {}, {} with power: {}", answer[0], answer[1], answer[2]);

    Ok(())
}
