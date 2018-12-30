extern crate regex;

use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::HashMap;

use self::regex::Regex;

type Output = (
    HashMap<String, Vec<String>>,
    HashMap<String, Vec<String>>,
    Vec<String>,
);

fn prepare_inputs() -> Output {
    let input = fs::read_to_string(Path::new("../data/day7.txt")).unwrap();
    let reg = Regex::new(r"Step (\w) .* step (\w)").unwrap();

    let mut parent_to_childs = HashMap::<String, Vec<String>>::new();
    let mut child_to_parents = HashMap::<String, Vec<String>>::new();

    for capture in reg.captures_iter(&input) {
        let (parent, child) = (capture[1].to_string(), capture[2].to_string());
        if parent_to_childs.contains_key(&parent) {
            if let Some(childs) = parent_to_childs.get_mut(&parent) {
                childs.push(child.to_string());
            }
        } else {
            parent_to_childs.insert(parent.to_string(), vec![child.to_string()]);
        }

        if !parent_to_childs.contains_key(&child) {
            parent_to_childs.insert(child.to_string(), Vec::<String>::new());
        }

        if child_to_parents.contains_key(&child) {
            if let Some(parents) = child_to_parents.get_mut(&child) {
                parents.push(parent.to_string());
            }
        } else {
            child_to_parents.insert(child.to_string(), vec![parent.to_string()]);
        }

        if !child_to_parents.contains_key(&parent) {
            child_to_parents.insert(parent.to_string(), Vec::<String>::new());
        }
    }

    let mut nodes_to_check = Vec::<String>::new();
    for (id, parents) in child_to_parents.iter() {
        if parents.is_empty() {
            nodes_to_check.push(id.clone());
        }
    }
    (parent_to_childs, child_to_parents, nodes_to_check)
}

pub fn first_star() -> Result<(), Box<Error + 'static>> {
    let (parent_to_childs, mut child_to_parents, mut nodes_to_check) = prepare_inputs();
    let mut answer = String::new();

    while !nodes_to_check.is_empty() {
        nodes_to_check.sort_by(|a, b| b.cmp(a));
        let node_id = nodes_to_check.pop().unwrap();
        answer.push_str(&node_id);
        for child_id in &parent_to_childs[&node_id] {
            if let Some(parents) = child_to_parents.get_mut(child_id) {
                let posi = parents.iter().position(|x| *x == node_id).unwrap();
                parents.remove(posi);
                if parents.is_empty() {
                    nodes_to_check.push(child_id.clone());
                }
            }
        }
    }

    println!("Order is: {}", answer);
    Ok(())
}

static MAX_WORKER: usize = 6;

#[derive(Debug)]
struct Worker {
    countdown: i32,
    id: String,
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    const MIN_DURATION: i32 = 60;

    let alpha = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();

    let mut answer = String::new();
    let mut id_to_duration = HashMap::<String, i32>::new();
    let mut count = 1;

    for letter in alpha.chars() {
        id_to_duration.insert(letter.to_string(), count);
        count += 1;
    }

    count = -1;

    let mut workers = Vec::<Worker>::new();
    let (parent_to_childs, mut child_to_parents, mut nodes_to_check) = prepare_inputs();

    while !(nodes_to_check.is_empty() && workers.is_empty()) {
        let mut workers_done = Vec::<usize>::new();
        for (index, worker) in workers.iter_mut().enumerate() {
            worker.countdown -= 1;
            if worker.countdown == 0 {
                workers_done.push(index);
            }
        }

        workers_done.reverse();

        for posi in workers_done {
            let worker = workers.remove(posi);
            for child_id in &parent_to_childs[&worker.id] {
                if let Some(parents) = child_to_parents.get_mut(child_id) {
                    let posi = parents.iter().position(|x| *x == worker.id).unwrap();
                    parents.remove(posi);
                    if parents.is_empty() {
                        nodes_to_check.push(child_id.clone());
                    }
                }
            }
            answer.push_str(&worker.id);
        }

        nodes_to_check.sort_by(|a, b| b.cmp(a));
        while !nodes_to_check.is_empty() && workers.len() < MAX_WORKER {
            let id = nodes_to_check.pop().unwrap();
            let countdown = id_to_duration[&id] + MIN_DURATION;
            workers.push(Worker { countdown, id });
        }
        count += 1;
    }

    println!("Order: {}, time: {}", answer, count);
    Ok(())
}
