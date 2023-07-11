mod interpreter;
use interpreter::*;

fn main() {
    // Avoid redundant reprocessing of instructions by parsing them all first.
    let instructions = load_input();
    let instructions: Vec<_> = instructions.lines()
        .map(Instruction::from_str)
        .collect();

    let mut interpreter = Interpreter::new([0; 4], instructions);
    interpreter.run();
    println!("Part 1: {}", interpreter.memory[0]);

    interpreter.memory = [0, 0, 1, 0];
    interpreter.ip = 0;
    interpreter.run();
    println!("Part 2: {}", interpreter.memory[0]);
}