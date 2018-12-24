extern crate regex;

use std::error::Error;

use std::fs;
use std::path::Path;

use std::collections::HashMap;

use self::regex::Regex;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Sand,
    Mud,
    Fill,
    Drop,
}

fn prepare_input() -> (Vec<Tile>, usize, usize, usize) {
    let input = fs::read_to_string(Path::new("./data/day17.txt")).unwrap();
    let range_reg = Regex::new(r"(x|y)=(\d+).*?(?:x|y)=(\d+)\.+(\d+)").unwrap();

    let (mut min_x, mut min_y, mut max_x, mut max_y) = (None, None, None, None);
    let mut x_range = HashMap::<usize, Vec<(usize, usize)>>::new();
    let mut y_range = HashMap::<usize, Vec<(usize, usize)>>::new();

    for cap in range_reg.captures_iter(&input) {
        let (position, min, max) = (
            cap[2].parse::<usize>().unwrap(),
            cap[3].parse::<usize>().unwrap(),
            cap[4].parse::<usize>().unwrap(),
        );
        let (range, min_r, max_r, min_l, max_l) = match &cap[1] {
            "x" => (&mut y_range, &mut min_y, &mut max_y, &mut min_x, &mut max_x),
            "y" => (&mut x_range, &mut min_x, &mut max_x, &mut min_y, &mut max_y),
            _ => unreachable!(),
        };
        let vecs = if let Some(rans) = range.get(&position) {
            let mut new_vec = rans.clone();
            new_vec.push((min, max));
            new_vec
        } else {
            vec![(min, max)]
        };

        range.insert(position, vecs);
        *min_r = match min_r {
            Some(p_r) => {
                if *p_r < min {
                    Some(*p_r)
                } else {
                    Some(min)
                }
            }
            None => Some(min),
        };
        *max_r = match max_r {
            Some(p_r) => {
                if *p_r > max {
                    Some(*p_r)
                } else {
                    Some(max)
                }
            }
            None => Some(max),
        };
        *min_l = match min_l {
            Some(p_r) => {
                if *p_r < position {
                    Some(*p_r)
                } else {
                    Some(position)
                }
            }
            None => Some(position),
        };
        *max_l = match max_l {
            Some(p_r) => {
                if *p_r > position {
                    Some(*p_r)
                } else {
                    Some(position)
                }
            }
            None => Some(position),
        };
    }

    let line_size = max_x.unwrap() - min_x.unwrap() + 3;
    let n_of_lines = (max_y.unwrap() - min_y.unwrap()) + 1;
    let unwrap_min_y = min_y.unwrap();
    let unwrap_min_x = min_x.unwrap();

    let mut map = Vec::new();

    for _ in 0..(line_size * n_of_lines) {
        map.push(Tile::Sand);
    }

    for (key, list) in x_range.iter() {
        for (min, max) in list {
            let offset_y = key - unwrap_min_y;
            let offset_x_min = min - unwrap_min_x + 1;
            let offset_x_max = max - unwrap_min_x + 1;
            for tile in map
                .iter_mut()
                .skip(line_size * offset_y + offset_x_min)
                .take(offset_x_max - offset_x_min)
            {
                *tile = Tile::Mud;
            }
        }
    }

    for (key, list) in y_range.iter() {
        for (min, max) in list {
            let offset_x = key - unwrap_min_x + 1;
            let offset_y_min = min - unwrap_min_y;
            let offset_y_max = max - unwrap_min_y;

            for i in offset_y_min..offset_y_max + 1 {
                map[i * line_size + offset_x] = Tile::Mud;
            }
        }
    }

    (map, unwrap_min_y, unwrap_min_x, line_size)
}

fn visualize(map: &[Tile], line_size: usize) {
    let mut txt = "".to_string();
    for (index, i) in map.iter().enumerate() {
        let temp = match i {
            Tile::Sand => ".",
            Tile::Mud => "#",
            Tile::Fill => "~",
            Tile::Drop => "|",
        };
        txt.push_str(temp);
        if index % line_size == line_size - 1 {
            txt.push('\n');
        }
    }
    println!("{}", txt);
}

pub fn first_star() -> Result<(), Box<Error + 'static>> {
    let (mut map, min_y, min_x, line_size) = prepare_input();

    let start_pos = 500 - min_x;
    let mut dripping_water = vec![start_pos];
    let mut filling_water = Vec::new();

    while !dripping_water.is_empty() {
        let mut water_drop = dripping_water.pop().unwrap();

        while water_drop < map.len()
            && (map[water_drop] == Tile::Sand || map[water_drop] == Tile::Drop)
        {
            map[water_drop] = Tile::Drop;
            water_drop += line_size;
        }

        if water_drop - line_size < min_y {
            continue;
        }

        filling_water.push(water_drop - line_size);

        while !filling_water.is_empty() {
            let water_drop = filling_water.pop().unwrap();
            let mut left_fill = water_drop - 1;
            let mut right_fill = water_drop + 1;
            map[water_drop] = Tile::Fill;

            if map[left_fill] == Tile::Fill && map[right_fill] == Tile::Fill {
                continue;
            }

            while (map[left_fill] == Tile::Sand || map[left_fill] == Tile::Drop)
                && (map[left_fill + line_size] == Tile::Mud
                    || map[left_fill + line_size] == Tile::Fill)
            {
                map[left_fill] = Tile::Fill;
                left_fill -= 1;
            }
            if right_fill + line_size > map.len() {
                visualize(&map, line_size);
                return Ok(());
            }
            while (map[right_fill] == Tile::Sand || map[right_fill] == Tile::Drop)
                && (map[right_fill + line_size] == Tile::Mud
                    || map[right_fill + line_size] == Tile::Fill)
            {
                map[right_fill] = Tile::Fill;
                right_fill += 1;
            }

            if map[right_fill] == map[left_fill] && map[left_fill] == Tile::Mud {
                filling_water.push(water_drop - line_size);
            }

            if map[left_fill] == Tile::Sand {
                for i in left_fill..right_fill {
                    map[i] = match map[i] {
                        Tile::Fill => Tile::Drop,
                        _ => map[i],
                    };
                }
                dripping_water.push(left_fill);
            }

            if map[right_fill] == Tile::Sand {
                for i in left_fill..right_fill {
                    map[i] = match map[i] {
                        Tile::Fill => Tile::Drop,
                        _ => map[i],
                    };
                }
                dripping_water.push(right_fill);
            }
        }
    }
    visualize(&map, line_size);

    let count = map
        .iter()
        .filter(|tile| **tile == Tile::Fill || **tile == Tile::Drop)
        .collect::<Vec<_>>()
        .len();

    println!("Filled water: {}", count);

    // 35439 too high
    Ok(())
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    Ok(())
}
