use day_12::interpreter::*;

fn main() {
    let instructions = load_input();
    let instructions: Vec<_> = instructions.lines()
        .map(|line| Instruction::from_str(line))
        .collect();

    let mut interpreter = Interpreter::new([7, 0, 0, 0], instructions.clone());
    interpreter.run();
    println!("Part 1: {}", interpreter.memory[0]);

    interpreter.instructions = instructions;
    interpreter.memory = [12, 0, 0, 0];
    interpreter.ip = 0;
    interpreter.run();
    println!("Part 2: {}", interpreter.memory[0]);
}

