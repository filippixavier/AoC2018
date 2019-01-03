use days::day19::*;

use std::error::Error;

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
    let a: u64 = 1;
    let mut d: u64 = 10504829;
    let mut e: u64;
    let mut f: u64;
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
                if d == a {
                    break 'main;
                } else {
                    continue 'main;
                }
            } else {
                e = e / 256;
            }
        }
    }
    Ok(())
}
// 1149790  too low
// 16777215 too high (MAX)
