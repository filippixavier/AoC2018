extern crate regex;

use std::error::Error;
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::cmp::Ordering;

use regex::Regex;
use regex::Captures;

fn first_star() -> Result<(), Box<Error + 'static>>{
    let file = fs::read_to_string(Path::new("./data/day4.txt"))?.trim().to_string();
    let mut input = file.lines().collect::<Vec<_>>();
    let id_reg = Regex::new(r"#(\d+)").unwrap();
    let date_reg = Regex::new(r"(?:\d+)-(\d+)-(\d+) (\d+):(\d+)").unwrap();
    let mut time_table: HashMap<String, [i32; 61]> = HashMap::new();
    input.sort_by(|&a, &b| {
        let date_a = date_reg.captures(a).unwrap();
        let date_b = date_reg.captures(b).unwrap();
        compare(&date_a, &date_b)
    });

    let mut duration: [usize; 2] = [0, 0];
    let mut index = 0;
    let mut current_id = "0";
    for line in &input {
        if id_reg.is_match(line) {
            if index != 0 {
                index = 0;
                fill_timeline(duration[0], 60, current_id, &mut time_table);
            }
            // Fighting the borrow checker here: the other access (&capture[x]) would throw an outliving reference error since it's a deref access
            current_id = id_reg.captures(line).unwrap().get(1).unwrap().as_str();
            continue;
        }
        if date_reg.is_match(line) {
            duration[index] = date_reg.captures(line).unwrap().get(4).unwrap().as_str().parse::<usize>().unwrap();
            index = (index + 1) % 2;

            if index == 0 {
                fill_timeline(duration[0], duration[1], current_id, &mut time_table);
            }
        }
    }

    let mut max = 0;
    let mut id = "0";
    for (key, val) in time_table.iter() {
        max = if max < val[60] {
            id = key;
            val[60]
        } else {
            max
        }
    }

    let sleepy_guard = time_table.get(id).unwrap();
    max = 0;
    let mut hour = 0;

    for i in 0..60 {
        max = if max < sleepy_guard[i] {
            hour = i;
            sleepy_guard[i]
        } else {
            max
        }
    }
    
    println!("\n{}", hour * id.parse::<usize>().unwrap());

    Ok(())
}

fn compare(a: &Captures, b: &Captures) -> Ordering {
    let month = [a.get(1).unwrap().as_str().parse::<usize>().unwrap(), b.get(1).unwrap().as_str().parse::<usize>().unwrap()];
    let day = [a.get(2).unwrap().as_str().parse::<usize>().unwrap(), b.get(2).unwrap().as_str().parse::<usize>().unwrap()];
    let hour = [a.get(3).unwrap().as_str().parse::<usize>().unwrap(), b.get(3).unwrap().as_str().parse::<usize>().unwrap()];
    let minute = [a.get(4).unwrap().as_str().parse::<usize>().unwrap(), b.get(4).unwrap().as_str().parse::<usize>().unwrap()];
    return if month[0] > month[1] ||
        month[0] == month[1] && day[0] > day[1] ||
        month[0] == month[1] && day[0] == day[1] && hour[0] > hour[1] || 
        month[0] == month[1] && day[0] == day[1] && hour[0] == hour[1] && minute[0] > minute[1]{
        Ordering::Greater
    } else {
        Ordering::Less
    }
}

fn fill_timeline(start: usize, end: usize, id: &str, time_table: &mut HashMap<String, [i32; 61]>) {
    let mut table: [i32; 61];
    match time_table.get(id) {
        Some(expr) => {
            table = *expr;
        },
        None => {
            table = [0; 61];
        }
    }

    for i in start..end {
        table[i] += 1;
        table[60] += 1;
    }

    time_table.insert(id.to_string(), table);
}