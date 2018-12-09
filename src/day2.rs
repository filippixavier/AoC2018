use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn first_star() -> Result<(), Box<Error + 'static>> {
    // Need to put the file opening in another variable else we would get a lifetime error
    let file = fs::read_to_string(Path::new("./data/day2.txt"))?;
    let input = file.trim().split('\n');
    let mut double = 0;
    let mut triple = 0;

    for id in input {
        let mut counter_hash = HashMap::new();
        for key in id.chars() {
            let value = counter_hash.entry(key).or_insert(0);
            *value += 1;
        }
        // Because of the borrow checker, values return a reference to the hashmap values and the filter function give a reference to the iterator value to the closure, meaning we have a ref to a ref
        // Also, I know I could probably write cleaner code, still getting my hand on basics through.
        double += if counter_hash.values().filter(|&x| *x == 2).count() > 0 {
            1
        } else {
            0
        };
        triple += if counter_hash.values().filter(|&x| *x == 3).count() > 0 {
            1
        } else {
            0
        }
    }

    println!("Checksum: {}", double * triple);

    Ok(())
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    // Need to put the file opening in another variable else we would get a lifetime error
    let file = fs::read_to_string(Path::new("./data/day2.txt"))?;
    let input: Vec<&str> = file.trim().split('\n').map(|x| x.trim()).collect();

    for i in 0..input.len() {
        // Because all strings are encoded into utf8, we can't directly access a char by index (Rust would throw an error), so instead we turn a string into a vector of chars
        let first_code: Vec<_> = input[i].chars().collect();
        for j in i + 1..input.len() {
            let mut count = 0;
            let answer = input[j]
                .chars()
                .filter(|&x| {
                    let cmp = first_code[count] == x;
                    count += 1;
                    cmp
                })
                .collect::<String>();
            if first_code.len() - answer.len() == 1 {
                println!(
                    "Id is: {}\n original are:\n{}\n{}",
                    answer, input[i], input[j]
                );
                break;
            }
        }
    }

    Ok(())
}
