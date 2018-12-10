use std::error::Error;
use std::fs;
use std::path::Path;

fn prepare_input() -> Vec<usize> {
    let file = fs::read_to_string(Path::new("./data/day8.txt")).unwrap();
    let input = file
        .trim()
        .split(' ')
        .map(|value| value.parse::<usize>().unwrap())
        .collect();

    input
}

pub fn first_star() -> Result<(), Box<Error + 'static>> {
    let input = prepare_input();
    let (answer, _) = deep_first_meta(&input, 0);
    println!("Sum is: {}", answer);
    Ok(())
}

fn deep_first_meta(array: &Vec<usize>, start: usize) -> (usize, usize) {
    let meta_num = array[start + 1];
    let mut node_num = array[start];
    let mut sub_start = start + 2;
    let mut meta = 0;

    while node_num > 0 {
        let (inter_meta, inter_start) = deep_first_meta(array, sub_start);
        sub_start = inter_start;
        meta += inter_meta;
        node_num -= 1;
    }

    for value in sub_start..sub_start + meta_num {
        meta += array[value];
    }
    (meta, sub_start + meta_num)
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    Ok(())
}
