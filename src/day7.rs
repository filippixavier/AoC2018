extern crate regex;

use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::HashMap;

use self::regex::Regex;


pub fn first_star() -> Result<(), Box<Error + 'static>> {
    let input = fs::read_to_string(Path::new("./data/day7.txt"))?;
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
            parent_to_childs.insert(parent.to_string(), vec!(child.to_string()));
        }

        if !parent_to_childs.contains_key(&child) {
            parent_to_childs.insert(child.to_string(), Vec::<String>::new());
        }

        if child_to_parents.contains_key(&child) {
            if let Some(parents) = child_to_parents.get_mut(&child) {
                parents.push(parent.to_string());
            }
        } else {
            child_to_parents.insert(child.to_string(), vec!(parent.to_string()));
        }

        if !child_to_parents.contains_key(&parent) {
            child_to_parents.insert(parent.to_string(), Vec::<String>::new());
        }
    }

    let mut nodes_to_check = Vec::<String>::new();
    let mut answer = String::new();
    for (id, parents) in child_to_parents.iter() {
        if parents.is_empty() {
            nodes_to_check.push(id.clone());
        }
    }

    while !nodes_to_check.is_empty() {
        nodes_to_check.sort_by(|a, b| b.cmp(a));
        let node_id = nodes_to_check.pop().unwrap();
        answer.push_str(&node_id);
        for child_id in parent_to_childs.get(&node_id).unwrap() {
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

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    Ok(())
}