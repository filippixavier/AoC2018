extern crate regex;

use days::day16::*;

use std::error::Error;

use std::fs;
use std::path::Path;

use self::regex::Regex;

struct Operation {
    fun: OpCode,
    arg_1: usize,
    arg_2: usize,
    arg_3: usize,
}

fn prepare_input() -> (usize, Vec<Operation>) {
    let input = fs::read_to_string(Path::new("../data/day19.txt")).unwrap();
    let id_reg = Regex::new(r"#ip (\d+)").unwrap();
    let ins_reg = Regex::new(r"(\w+) (\d+) (\d+) (\d+)").unwrap();
    let mut reg = 0;
    let mut instructions = Vec::new();
    if let Some(cap) = id_reg.captures(&input) {
        reg = cap[1].parse::<usize>().unwrap();
    }

    for cap in ins_reg.captures_iter(&input) {
        let (ins_name, arg_1, arg_2, arg_3) = (
            cap[1].to_string(),
            cap[2].parse::<usize>().unwrap(),
            cap[3].parse::<usize>().unwrap(),
            cap[4].parse::<usize>().unwrap(),
        );
        let new_instruction = match ins_name.as_str() {
            "addr" => Operation {
                fun: addr,
                arg_1,
                arg_2,
                arg_3,
            },
            "addi" => Operation {
                fun: addi,
                arg_1,
                arg_2,
                arg_3,
            },
            "mulr" => Operation {
                fun: mulr,
                arg_1,
                arg_2,
                arg_3,
            },
            "muli" => Operation {
                fun: muli,
                arg_1,
                arg_2,
                arg_3,
            },
            "banr" => Operation {
                fun: banr,
                arg_1,
                arg_2,
                arg_3,
            },
            "bani" => Operation {
                fun: bani,
                arg_1,
                arg_2,
                arg_3,
            },
            "borr" => Operation {
                fun: borr,
                arg_1,
                arg_2,
                arg_3,
            },
            "bori" => Operation {
                fun: bori,
                arg_1,
                arg_2,
                arg_3,
            },
            "setr" => Operation {
                fun: setr,
                arg_1,
                arg_2,
                arg_3,
            },
            "seti" => Operation {
                fun: seti,
                arg_1,
                arg_2,
                arg_3,
            },
            "gtir" => Operation {
                fun: gtir,
                arg_1,
                arg_2,
                arg_3,
            },
            "gtri" => Operation {
                fun: gtri,
                arg_1,
                arg_2,
                arg_3,
            },
            "gtrr" => Operation {
                fun: gtrr,
                arg_1,
                arg_2,
                arg_3,
            },
            "eqir" => Operation {
                fun: eqir,
                arg_1,
                arg_2,
                arg_3,
            },
            "eqri" => Operation {
                fun: eqri,
                arg_1,
                arg_2,
                arg_3,
            },
            "eqrr" => Operation {
                fun: eqrr,
                arg_1,
                arg_2,
                arg_3,
            },
            _ => unreachable!(),
        };
        instructions.push(new_instruction);
    }

    (reg, instructions)
}

pub fn first_star() -> Result<(), Box<Error + 'static>> {
    let (reg_i, instructions) = prepare_input();
    let mut regs: [usize; 6] = [0; 6];
    loop {
        let i_pointer = regs[reg_i];
        if let Some(operation) = instructions.get(i_pointer) {
            (operation.fun)(&mut regs, operation.arg_1, operation.arg_2, operation.arg_3);
            regs[reg_i] += 1;
        } else {
            break;
        }
    }
    println!("Reg 0 value: {}", regs[0]);
    Ok(())
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    let (reg_i, instructions) = prepare_input();
    let mut regs: [usize; 6] = [0; 6];
    regs[0] = 1;
    loop {
        let i_pointer = regs[reg_i];
        if let Some(operation) = instructions.get(i_pointer) {
            (operation.fun)(&mut regs, operation.arg_1, operation.arg_2, operation.arg_3);
            regs[reg_i] += 1;
        } else {
            break;
        }
        // println!("{}", regs[reg_i]);
    }
    println!("Reg 0 value: {}", regs[0]);
    Ok(())
}
