use days::day19::*;

use std::error::Error;
use std::collections::HashSet;

pub fn first_star() -> Result<(), Box<Error + 'static>> {
    let (reg_i, instructions) = prepare_input("./data/day21.txt");
    let mut regs: [usize; 6] = [0; 6];
    loop {
        let i_pointer = regs[reg_i];
        // Tricks mostly involve understanding the code: in my input there's only one place that read the reg[0] value and it's the exit condition, so we shortcut the code at that point and give the required input
        if i_pointer == 28 {
            break;
        }
        if let Some(operation) = instructions.get(i_pointer) {
            (operation.fun)(&mut regs, operation.arg_1, operation.arg_2, operation.arg_3);
            regs[reg_i] += 1;
        } else {
            break;
        }
    }
    println!("Reg 0 required value: {}", regs[3]);
    Ok(())
}

fn exec(instructions: &Vec<Operation>, reg_i: usize, start: usize) -> usize {
    let mut regs: [usize; 6] = [0; 6];
    regs[0] = start;
    let mut count = 0;
    loop {
        let i_pointer = regs[reg_i];
        if let Some(operation) = instructions.get(i_pointer) {
            (operation.fun)(&mut regs, operation.arg_1, operation.arg_2, operation.arg_3);
            regs[reg_i] += 1;
            count += 1;
        } else {
            break;
        }
    }
    count
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    let mut a: u64 = 0;
    let mut d: u64 = 10504829;
    let mut e: u64;
    let mut f: u64;
    let mut set = HashSet::<u64>::new();
    'main: loop {
        e = d | 65_536;
        d = 10_649_702;
        loop {
            f = e & 255;
            d = f + d;

            d = d & 16_777_215;
            d = d * 65_899;
            d = d & 16_777_215;

            if 256 > e {
                if !set.insert(d) {
                    // Not sure I do understand how it works: basically, once we get our first duplicate D, it means that the previous value was the last value to be unique as any further one could be a duplicate
                    // It's also the our lowest with the most instructions as any further ones could have halted the program earlier
                    // Probably involve some math magic regarding modulo but couldn't figure that out, in short, it was pure luck, sorry. 
                    break 'main;
                }
                if d == a {
                    break 'main;
                } else {
                    a = d;
                    continue 'main;
                }
            } else {
                e = e / 256;
            }
        }
    }
    println!("{}", a);
    Ok(())
}
// 1149790  too low
// 16777215 too high (MAX)
// 6339601
