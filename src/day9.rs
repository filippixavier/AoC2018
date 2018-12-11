extern crate regex;

use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

use self::regex::Regex;

pub fn first_star() -> Result<(), Box<Error + 'static>> {
    let input = fs::read_to_string(Path::new("./data/day9.txt")).unwrap();
    let reg = Regex::new(r"(\d+).+?(\d+)").unwrap();

    let mut num_of_players = 0;
    let mut scores = Vec::<i32>::new();
    let mut max = 0;

    let mut marbles = vec![0];
    let mut current_posi = 0;

    if let Some(capture) = reg.captures(&input) {
        num_of_players = capture[1].parse::<i32>().unwrap();
        max = capture[2].parse::<i32>().unwrap();
        for _ in 0..num_of_players {
            scores.push(0);
        }
    }

    for marble in 1..max {
        if marble % 23 == 0 {
            if let Some(score) = scores.get_mut(((marble - 1) % num_of_players) as usize) {
                *score += marble;
                current_posi = (current_posi + (marbles.len() - 7)) % marbles.len();
                let temp = marbles.remove(current_posi);
                // println!("Removed: {}", temp);
                *score += temp;
            }
        } else {
            current_posi = (((current_posi + 1) % marbles.len()) + 1) % (marbles.len() + 1);
            marbles.insert(current_posi, marble);
        }
    }

    if let Some(highscore) = scores.iter().max() {
        println!("highscore is: {}", highscore);
    }

    Ok(())
}

#[derive(Debug)]
struct Marble {
    previous: i32,
    next: i32,
}

// Build it on release to speed up the process
// Note: Apparently faster using a VecDeque and push/shift our way through
pub fn second_star() -> Result<(), Box<Error + 'static>> {
    let input = fs::read_to_string(Path::new("./data/day9.txt")).unwrap();
    let reg = Regex::new(r"(\d+).+?(\d+)").unwrap();

    let mut num_of_players = 0;
    let mut scores = Vec::<u32>::new(); // Too big for standard integer
    let mut player: usize = 0;

    let mut max = 0;

    let mut fake_cyclic_linked_list = HashMap::<i32, Marble>::new();
    let mut current_id = 0;

    if let Some(capture) = reg.captures(&input) {
        num_of_players = capture[1].parse::<usize>().unwrap();
        max = capture[2].parse::<i32>().unwrap() * 100;
        for _ in 0..num_of_players {
            scores.push(0);
        }
    }

    fake_cyclic_linked_list.insert(
        0,
        Marble {
            previous: 0,
            next: 0,
        },
    );

    for i in 1..max {
        if i % 23 != 0 {
            let mut new_node = Marble {
                previous: 0,
                next: 0,
            };
            {
                let node = &fake_cyclic_linked_list[&current_id];
                new_node.previous = node.next;
            }
            {
                let node = fake_cyclic_linked_list.get_mut(&new_node.previous).unwrap();
                new_node.next = node.next;
                node.next = i;
            }
            {
                let node = fake_cyclic_linked_list.get_mut(&new_node.next).unwrap();
                node.previous = i;
            }
            current_id = i;
            fake_cyclic_linked_list.insert(i, new_node);
        } else {
            let mut sixth_node_id;
            let mut eigth_node_id;
            let mut removed_node_id;
            {
                let mut node = &fake_cyclic_linked_list[&current_id];
                for _ in 0..5 {
                    node = &fake_cyclic_linked_list[&node.previous];
                }
                sixth_node_id = node.previous;
                for _ in 0..2 {
                    node = &fake_cyclic_linked_list[&node.previous];
                }
                eigth_node_id = node.previous;
            }
            {
                let node = fake_cyclic_linked_list.get_mut(&sixth_node_id).unwrap();
                removed_node_id = node.previous;
                node.previous = eigth_node_id;
            }
            {
                let node = fake_cyclic_linked_list.get_mut(&eigth_node_id).unwrap();
                node.next = sixth_node_id;
            }
            current_id = sixth_node_id;
            // println!("Removed2: {}", removed_node_id);
            scores[player] += i as u32 + removed_node_id as u32;
            fake_cyclic_linked_list.remove(&removed_node_id);
        }
        player = (player + 1) % num_of_players;
    }

    if let Some(highscore) = scores.iter().max() {
        println!("highscore is: {}", highscore);
    }

    Ok(())
}
