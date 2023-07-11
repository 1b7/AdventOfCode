use std::collections::HashSet;
use day_12::interpreter::*;

fn main() {
    let instructions = load_input();
    let instructions: Vec<_> = instructions.lines()
        .map(|line| Instruction::from_str(line))
        .collect();
    clock_signal(instructions);
}

fn clock_signal(instructions: Vec<Instruction>) -> usize {
    for i in 0.. {
        let mut int = Interpreter::new([i, 0, 0, 0], instructions.clone());
        let mut seen = HashSet::new();
        let mut prev_ip = int.ip;
        loop {
            int.exec();
            if int.instructions[prev_ip].opcode == Opcode::Out {
                if seen.contains(&int.buffer) { 
                    println!("[Rejected {:06}]\t{:016b}", i, int.buffer); 
                    break;
                }
                seen.insert(int.buffer);
                if int.buffer == 0b0101010101010101 || int.buffer == 0b1010101010101010 {
                    println!("First valid pattern from: {}", i);
                    return i as usize
                }
            }
            prev_ip = int.ip;
        }
    }
    unreachable!()
}