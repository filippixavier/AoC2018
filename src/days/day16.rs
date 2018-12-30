extern crate regex;

use std::error::Error;

use std::fs;
use std::path::Path;

use std::collections::HashMap;
use std::collections::HashSet;

use self::regex::Regex;

pub fn addr(registers: &mut [usize], reg_a: usize, reg_b: usize, reg_c: usize) {
    registers[reg_c] = registers[reg_a] + registers[reg_b];
}

pub fn addi(registers: &mut [usize], reg_a: usize, value: usize, reg_c: usize) {
    registers[reg_c] = registers[reg_a] + value;
}

pub fn mulr(registers: &mut [usize], reg_a: usize, reg_b: usize, reg_c: usize) {
    registers[reg_c] = registers[reg_a] * registers[reg_b];
}

pub fn muli(registers: &mut [usize], reg_a: usize, value: usize, reg_c: usize) {
    registers[reg_c] = registers[reg_a] * value;
}

pub fn banr(registers: &mut [usize], reg_a: usize, reg_b: usize, reg_c: usize) {
    registers[reg_c] = registers[reg_a] & registers[reg_b];
}

pub fn bani(registers: &mut [usize], reg_a: usize, value: usize, reg_c: usize) {
    registers[reg_c] = registers[reg_a] & value;
}

pub fn borr(registers: &mut [usize], reg_a: usize, reg_b: usize, reg_c: usize) {
    registers[reg_c] = registers[reg_a] | registers[reg_b];
}

pub fn bori(registers: &mut [usize], reg_a: usize, value: usize, reg_c: usize) {
    registers[reg_c] = registers[reg_a] | value;
}

pub fn setr(registers: &mut [usize], reg_a: usize, _unused: usize, reg_c: usize) {
    registers[reg_c] = registers[reg_a];
}

pub fn seti(registers: &mut [usize], value: usize, _unused: usize, reg_c: usize) {
    registers[reg_c] = value;
}

pub fn gtir(registers: &mut [usize], value: usize, reg_b: usize, reg_c: usize) {
    registers[reg_c] = if value > registers[reg_b] { 1 } else { 0 }
}

pub fn gtri(registers: &mut [usize], reg_a: usize, value: usize, reg_c: usize) {
    registers[reg_c] = if registers[reg_a] > value { 1 } else { 0 }
}

pub fn gtrr(registers: &mut [usize], reg_a: usize, reg_b: usize, reg_c: usize) {
    registers[reg_c] = if registers[reg_a] > registers[reg_b] {
        1
    } else {
        0
    }
}

pub fn eqir(registers: &mut [usize], value: usize, reg_b: usize, reg_c: usize) {
    registers[reg_c] = if value == registers[reg_b] { 1 } else { 0 }
}

pub fn eqri(registers: &mut [usize], reg_a: usize, value: usize, reg_c: usize) {
    registers[reg_c] = if registers[reg_a] == value { 1 } else { 0 }
}

pub fn eqrr(registers: &mut [usize], reg_a: usize, reg_b: usize, reg_c: usize) {
    registers[reg_c] = if registers[reg_a] == registers[reg_b] {
        1
    } else {
        0
    }
}

pub type OpCode = fn(&mut [usize], usize, usize, usize);

fn prepare_input() -> (Vec<[usize; 12]>, Vec<[usize; 4]>) {
    let input_1 = fs::read_to_string(Path::new("../data/day16_1.txt")).unwrap();
    let input_2 = fs::read_to_string(Path::new("../data/day16_2.txt")).unwrap();

    let mut first_part = Vec::new();
    let mut second_part = Vec::new();

    let first_part_reg = Regex::new(r"(\d+).*?(\d+).*?(\d+).*?(\d+).*?\s.*?(\d+).*?(\d+).*?(\d+).*?(\d+).*?\s.*?(\d+).*?(\d+).*?(\d+).*?(\d+)").unwrap();
    let second_part_reg = Regex::new(r"(\d+).*?(\d+).*?(\d+).*?(\d+)").unwrap();

    for cap in first_part_reg.captures_iter(&input_1) {
        let (dig_0, dig_1, dig_2, dig_3, dig_4, dig_5, dig_6, dig_7, dig_8, dig_9, dig_10, dig_11) = (
            &cap[1].parse::<usize>().unwrap(),
            &cap[2].parse::<usize>().unwrap(),
            &cap[3].parse::<usize>().unwrap(),
            &cap[4].parse::<usize>().unwrap(),
            &cap[5].parse::<usize>().unwrap(),
            &cap[6].parse::<usize>().unwrap(),
            &cap[7].parse::<usize>().unwrap(),
            &cap[8].parse::<usize>().unwrap(),
            &cap[9].parse::<usize>().unwrap(),
            &cap[10].parse::<usize>().unwrap(),
            &cap[11].parse::<usize>().unwrap(),
            &cap[12].parse::<usize>().unwrap(),
        );
        first_part.push([
            *dig_0, *dig_1, *dig_2, *dig_3, *dig_4, *dig_5, *dig_6, *dig_7, *dig_8, *dig_9,
            *dig_10, *dig_11,
        ]);
    }

    for cap in second_part_reg.captures_iter(&input_2) {
        let (dig_0, dig_1, dig_2, dig_3) = (
            &cap[1].parse::<usize>().unwrap(),
            &cap[2].parse::<usize>().unwrap(),
            &cap[3].parse::<usize>().unwrap(),
            &cap[4].parse::<usize>().unwrap(),
        );
        second_part.push([*dig_0, *dig_1, *dig_2, *dig_3]);
    }

    (first_part, second_part)
}

pub fn first_star() -> Result<(), Box<Error + 'static>> {
    let (first_part, _) = prepare_input();
    let fns = [
        addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri,
        eqrr,
    ];

    let mut count = 0;

    for code in first_part {
        let mut same_result = 0;
        let before = [code[0], code[1], code[2], code[3]];
        let op = &code[4..8];
        let after = &code[8..12];

        for fun in fns.iter() {
            let mut before_clone = before;
            fun(&mut before_clone, op[1], op[2], op[3]);

            if before_clone == after {
                same_result += 1;
            }
        }

        if same_result >= 3 {
            count += 1;
        }
    }

    println!("Result: {}", count);

    Ok(())
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    let (first_part, second_part) = prepare_input();
    let fns = [
        addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri,
        eqrr,
    ];

    let mut opcode = HashMap::<usize, OpCode>::new();

    {
        let mut tentative_opcode = HashMap::<usize, Vec<usize>>::new();
        let mut recovered_opcode = HashSet::<usize>::new();
        for code in first_part {
            let before = [code[0], code[1], code[2], code[3]];
            let op = &code[4..8];
            let after = &code[8..12];
            let mut matches = Vec::new();

            for (index, fun) in fns.iter().enumerate() {
                let mut before_clone = before;
                fun(&mut before_clone, op[1], op[2], op[3]);

                if before_clone == after {
                    matches.push(index);
                }
            }

            let matching_to_op = if let Some(tentative) = tentative_opcode.get_mut(&op[0]) {
                tentative.append(&mut matches);
                tentative.sort();
                tentative.dedup();
                tentative.clone()
            } else {
                matches
            };

            if matching_to_op.len() == 1 {
                recovered_opcode.insert(matching_to_op[0]);
                opcode.insert(op[0], fns[matching_to_op[0]]);
            }

            tentative_opcode.insert(op[0], matching_to_op);
        }

        let mut loop_breaker = 0;
        loop {
            if tentative_opcode.len() == opcode.len() {
                break;
            }
            for (op, list) in tentative_opcode.iter_mut() {
                if opcode.contains_key(&op) {
                    continue;
                }
                *list = list
                    .iter()
                    .cloned()
                    .filter(|x| !recovered_opcode.contains(x))
                    .collect();

                if list.len() == 1 {
                    recovered_opcode.insert(list[0]);
                    opcode.insert(*op, fns[list[0]]);
                }
            }

            loop_breaker += 1;
            if loop_breaker == 1000 {
                break;
            }
        }
    }

    let mut regs = [0, 0, 0, 0];

    for line in second_part {
        opcode[&line[0]](&mut regs, line[1], line[2], line[3]);
    }

    println!("{:?}", regs);

    Ok(())
}
