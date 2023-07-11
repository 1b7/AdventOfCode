fn main() {
    // Avoid redundant reprocessing of instructions by parsing them all first.
    let instructions = load_input();
    let instructions: Vec<_> = instructions.lines()
        .map(|line| Instruction::from_str(line))
        .collect();

    let mut interpreter = Interpreter::new([0; 4], instructions);
    interpreter.run();
    println!("Part 1: {}", interpreter.memory[0]);

    interpreter.memory = [0, 0, 1, 0];
    interpreter.ip = 0;
    interpreter.run();
    println!("Part 2: {}", interpreter.memory[0]);
}

struct Interpreter {
    memory: [isize; 4],
    instructions: Vec<Instruction>,
    ip: usize
}

impl Interpreter {
    pub fn new(memory: [isize; 4], instructions: Vec<Instruction>) -> Self {
        Self { memory, instructions, ip: 0 }
    }

    fn exec(&mut self) {
        let as_reg = |n| (n as u8 - b'a') as usize;
        
        let mut jumped = false;
        let inst = self.instructions[self.ip];
        match inst.opcode {
            Opcode::Cpy => {
                self.memory[as_reg(inst.ops[1])] = match inst.mode {
                    Mode::Direct => inst.ops[0],
                    Mode::Register => self.memory[as_reg(inst.ops[0])]
                };
            },
            Opcode::Inc => self.memory[as_reg(inst.ops[0])] += 1,
            Opcode::Dec => self.memory[as_reg(inst.ops[0])] -= 1,
            Opcode::Jnz => {
                let cmp = match inst.mode {
                    Mode::Direct => inst.ops[0],
                    Mode::Register => self.memory[as_reg(inst.ops[0])]
                };

                if cmp != 0 {
                    self.ip = ((self.ip as isize) + inst.ops[1]) as usize;
                    jumped = true;
                }
            }
        }

        if !jumped { self.ip += 1; }
    }

    pub fn run(&mut self) {
        while self.ip < self.instructions.len() { self.exec(); }
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    opcode: Opcode,
    mode: Mode,
    ops: [isize; 2]
}

impl Instruction {
    pub fn new(opcode: Opcode, mode: Mode, ops: [isize; 2]) -> Self {
        Self { opcode, mode, ops }
    }

    pub fn from_str(s: &str) -> Self {
        let words: Vec<_> = s.split_whitespace().collect();
        let op = words[1].chars().next().unwrap();
        match words[0] {
            "cpy" => {
                let register = words[2].chars().next().unwrap() as isize;
                if let Ok(n) = words[1].parse() {
                    Instruction::new(Opcode::Cpy, Mode::Direct, [n, register])
                } else {
                    Instruction::new(Opcode::Cpy, Mode::Register, [op as isize, register])
                }
            },
            "inc" => Instruction::new(Opcode::Inc, Mode::Register, [op as isize, 0]),
            "dec" => Instruction::new(Opcode::Dec, Mode::Register, [op as isize, 0]),
            "jnz" => {
                let destination = words[2].parse().unwrap();
                if let Ok(n) = words[1].parse() {
                    Instruction::new(Opcode::Jnz, Mode::Direct, [n, destination])
                } else {
                    Instruction::new(Opcode::Jnz, Mode::Register, [op as isize, destination])
                }
            },
            // "tgl" => Instr::Toggle(words[1].parse().unwrap()),
            _ => panic!("Unrecognised Instruction")
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Mode { Register, Direct }

#[derive(Debug, Clone, Copy)]
enum Opcode { Cpy, Inc, Dec, Jnz }

fn load_input() -> String {
    std::fs::read_to_string(
        std::env::args().nth(1).expect("Filepath not provided.")
    ).expect("Could not load file!")
}