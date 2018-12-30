use std::error::Error;
use std::io;

use std::collections::HashMap;

fn get_serial() -> i32 {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => buffer.trim().parse::<i32>().unwrap_or(0),
        Err(error) => {
            println!("Error: {}", error);
            0
        }
    }
}

fn get_cell_power(x: i32, y: i32, serial: i32) -> i32 {
    let rack_id = x + 10;
    let temp_power = ((rack_id * y) + serial) * rack_id;
    (temp_power / 100 % 10) - 5
}

fn get_max_area_moving(serial: i32, size: i32, memoize: &mut HashMap<(i32, i32), i32>) -> [i32; 3] {
    let mut answer = [0, 0, 0];
    for y in 0..301 - size {
        let mut square_power = 0;
        for i in 0..size {
            for j in y..y + size {
                square_power += *memoize
                    .entry((i, j))
                    .or_insert_with(|| get_cell_power(i + 1, j + 1, serial));
            }
        }
        if square_power > answer[2] {
            answer[0] = 1;
            answer[1] = y + 1;
            answer[2] = square_power;
        }
        for x in 0..301 - size {
            square_power = move_right(square_power, (x, y), size, serial, memoize);
            if square_power > answer[2] {
                answer[0] = x + 2;
                answer[1] = y + 1;
                answer[2] = square_power;
            }
        }
    }
    answer
}

fn move_right(
    power: i32,
    start: (i32, i32),
    size: i32,
    serial: i32,
    memoize: &mut HashMap<(i32, i32), i32>,
) -> i32 {
    let mut temp_power = power;
    for y in start.1..start.1 + size {
        temp_power -= *memoize
            .entry((start.0, y))
            .or_insert_with(|| get_cell_power(start.0 + 1, y + 1, serial));
        temp_power += *memoize
            .entry((start.0 + size, y))
            .or_insert_with(|| get_cell_power(start.0 + size + 1, y + 1, serial))
    }
    temp_power
}

pub fn first_star() -> Result<(), Box<Error + 'static>> {
    // So apparently I could just do it without the Hashmap memoization and use a Vec of vec instead and would have been juuuust fine ... (Also APL rocked that one)
    let mut memoize = HashMap::<(i32, i32), i32>::new();
    let serial = get_serial();
    let answer;
    answer = get_max_area_moving(serial, 3, &mut memoize);

    println!(
        "Answer is: {}, {} with power: {}",
        answer[0], answer[1], answer[2]
    );
    Ok(())
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    let serial = get_serial();
    // I failed hard: this is a solution took from the MegaThread (in C++)
    let mut sum_array = [[0; 301]; 301];
    let mut answer = (0, 0, 0);
    let mut true_size = 1;

    for x in 1..301 {
        for y in 1..301 {
            sum_array[x][y] = get_cell_power(x as i32, y as i32, serial)
                + sum_array[x][y - 1]
                + sum_array[x - 1][y]
                - sum_array[x - 1][y - 1];
        }
    }

    for size in 1..301 {
        for x in size..301 {
            for y in size..301 {
                let total = sum_array[x][y] - sum_array[x - size][y] - sum_array[x][y - size]
                    + sum_array[x - size][y - size];
                if total > answer.2 {
                    true_size = size;
                    answer = (x - size + 1, y - size + 1, total);
                }
            }
        }
    }

    println!(
        "Answer is: {}, {} with power: {} and size: {}",
        answer.0, answer.1, answer.2, true_size
    );

    Ok(())
}
