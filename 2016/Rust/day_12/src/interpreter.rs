pub struct Interpreter {
    pub memory: [isize; 4],
    pub instructions: Vec<Instruction>,
    pub ip: usize,
    pub buffer: u16
}

impl Interpreter {
    pub fn new(memory: [isize; 4], instructions: Vec<Instruction>) -> Self {
        Self { memory, instructions, ip: 0, buffer: 0 }
    }

    fn toggle(&mut self, n: usize) {
        if n >= self.instructions.len() { return; }
        match self.instructions[n].opcode {
            Opcode::Inc => self.instructions[n].opcode = Opcode::Dec,
            Opcode::Dec | Opcode::Tgl | Opcode::Out => 
                self.instructions[n].opcode = Opcode::Inc,
            Opcode::Jnz => self.instructions[n].opcode = Opcode::Cpy,
            Opcode::Cpy => self.instructions[n].opcode = Opcode::Jnz
        }
    }

    pub fn exec(&mut self) {
        let as_reg = |n| (n as u8 - b'a') as usize;
        
        let mut jumped = false;
        let inst = self.instructions[self.ip];
        match inst.opcode {
            Opcode::Cpy => {
                self.memory[as_reg(inst.ops[1])] = match inst.mode {
                    Mode::DirReg => inst.ops[0],
                    Mode::Register => self.memory[as_reg(inst.ops[0])],
                    _ => self.memory[as_reg(inst.ops[1])] // No Op If Not Right Mode.
                };
            },
            Opcode::Inc => self.memory[as_reg(inst.ops[0])] += 1,
            Opcode::Dec => self.memory[as_reg(inst.ops[0])] -= 1,
            Opcode::Jnz => {
                let cmp = match inst.mode {
                    Mode::Direct | Mode::DirReg => inst.ops[0],
                    Mode::Register | Mode::RegDir => self.memory[as_reg(inst.ops[0])],
                };

                let offset = match inst.mode {
                    Mode::Direct | Mode::RegDir => inst.ops[1],
                    Mode::Register | Mode::DirReg => self.memory[as_reg(inst.ops[1])],
                };

                if cmp != 0 {
                    self.ip = ((self.ip as isize) + offset) as usize;
                    jumped = true;
                }
            },
            Opcode::Tgl => {
                let offset = match inst.mode {
                    Mode::Direct | Mode::DirReg => inst.ops[0],
                    Mode::Register | Mode::RegDir => self.memory[as_reg(inst.ops[0])]
                };
                let target = ((self.ip as isize) + offset) as usize;

                self.toggle(target);
            },
            Opcode::Out => {
                let v = match inst.mode {
                    Mode::Direct | Mode::DirReg => inst.ops[0],
                    Mode::Register | Mode::RegDir => self.memory[as_reg(inst.ops[0])]
                };
                if v != 0 && v != 1 { return }

                self.buffer = self.buffer << 1;
                if v == 1 { self.buffer |= 1; }
            }
        }

        if !jumped { self.ip += 1; }
    }

    pub fn run(&mut self) {
        while self.ip < self.instructions.len() { self.exec(); }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub opcode: Opcode,
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
                    Instruction::new(Opcode::Cpy, Mode::DirReg, [n, register])
                } else {
                    Instruction::new(Opcode::Cpy, Mode::Register, [op as isize, register])
                }
            },
            "inc" => Instruction::new(Opcode::Inc, Mode::Register, [op as isize, 0]),
            "dec" => Instruction::new(Opcode::Dec, Mode::Register, [op as isize, 0]),
            "jnz" => {
                if let Ok(n) = words[1].parse() {
                    if let Ok(m) = words[2].parse() {
                        Instruction::new(Opcode::Jnz, Mode::Direct, [n, m])
                    } else {
                        Instruction::new(Opcode::Jnz, Mode::DirReg, [n, words[2].chars().next().unwrap() as isize])
                    }
                } else {
                    if let Ok(m) = words[2].parse() {
                        Instruction::new(Opcode::Jnz, Mode::RegDir, [op as isize, m])
                    } else {
                        Instruction::new(Opcode::Jnz, Mode::Register, [op as isize, words[2].chars().next().unwrap() as isize])
                    }
                }
            },
            "tgl" => {
                if let Ok(n) = words[1].parse() {
                    Instruction::new(Opcode::Tgl, Mode::Direct, [n, 0])
                } else {
                    Instruction::new(Opcode::Tgl, Mode::Register, [op as isize, 0])
                }
            },
            "out" => {
                if let Ok(n) = words[1].parse() {
                    Instruction::new(Opcode::Out, Mode::Direct, [n, 0])
                } else {
                    Instruction::new(Opcode::Out, Mode::Register, [op as isize, 0])
                }
            }
            _ => panic!("Unrecognised Instruction")
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Mode { Register, Direct, RegDir, DirReg }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode { Cpy, Inc, Dec, Jnz, Tgl, Out }

pub fn load_input() -> String {
    std::fs::read_to_string(
        std::env::args().nth(1).expect("Filepath not provided.")
    ).expect("Could not load file!")
}