use std::error::Error;
use std::fs;
use std::path::Path;

fn first_star() -> Result<(), Box<Error + 'static>> {
    // Well that was painful: the file contain space to trim before parsing along with a final empty character, hence the unwrap_or
    println!("{}", fs::read_to_string(Path::new("./data/day1.txt"))?.split('\n').map( |x| x.trim().parse::<i32>().unwrap_or(0)).fold(0, |acc, x| x + acc));
    Ok(())
}

fn second_star() -> Result<(), Box<Error + 'static>> {
    let mut found_frequency = HashSet::new();
    found_frequency.insert(0);
    // Note: if the data file was on the same level as the source file, juste use the include_str! macro
    // Note that this time we trim the input beforehand as the 0 would lead to a falsy result
    let input = fs::read_to_string(Path::new("./data/day1.txt"))?.trim().split('\n').map( |x| x.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut acc = 0;

    for frequency in input.iter().cycle() {
        acc += frequency;
        if !found_frequency.insert(acc) {
            break;
        }
    }

    println!("{}", acc);

    Ok(())
}