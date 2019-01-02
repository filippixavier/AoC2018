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

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    // 1149790  too low
    // 16777215 too high (MAX)
    Ok(())
}
