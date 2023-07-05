fn main() {
    // Avoid redundant reprocessing of instructions by parsing them all first.
    let instructions = load_input();
    let instructions: Vec<_> = instructions.lines().enumerate().map(as_instr).collect();

    println!("Part 1: {}", run(&instructions, [0; 4]));

    let mut mem = [0; 4];
    mem[2] = 1;
    println!("Part 2: {}", run(&instructions, mem));
}

fn run(instructions: &[Instr], mut mem: [isize; 4]) -> isize {
    let mut ip = 0;
    while ip < instructions.len() {
        let mut jumped = false;
        match instructions[ip] {
            Instr::Put(a, b) => { mem[chr_to_reg(b)] = a }
            Instr::Cpy(a, b) => { mem[chr_to_reg(b)] = mem[chr_to_reg(a)] },
            Instr::Inc(a) => { mem[chr_to_reg(a)] += 1 },
            Instr::Dec(a) => { mem[chr_to_reg(a)] -= 1},
            Instr::MemJnz(a, b) => {
                if mem[chr_to_reg(a)] != 0 {
                    ip = b;
                    jumped = true;
                }
            },
            Instr::IntJnz(a, b) => {
                if a != 0 {
                    ip = b;
                    jumped = true;
                }
            }
        };
        if !jumped { ip += 1; }
    }
    mem[0]
}

fn chr_to_reg(c: char) -> usize { (c as u8 - b'a') as usize }

fn as_instr(s: (usize, &str)) -> Instr {
    let base = s.0;
    let words: Vec<_> = s.1.split_whitespace().collect();
    match words[0] {
        "cpy" => {
            if let Ok(n) = words[1].parse() {
                Instr::Put(n, words[2].chars().next().unwrap())
            } else {
                Instr::Cpy(words[1].chars().next().unwrap(), words[2].chars().next().unwrap())
            }
        },
        "inc" => Instr::Inc(words[1].chars().next().unwrap()),
        "dec" => Instr::Dec(words[1].chars().next().unwrap()),
        "jnz" => {
            if let Ok(n) = words[1].parse::<usize>() {
                Instr::IntJnz(words[1].parse().unwrap(), (base as isize + words[2].parse::<isize>().unwrap()) as usize)
            } else {
                Instr::MemJnz(words[1].chars().next().unwrap(), (base as isize + words[2].parse::<isize>().unwrap()) as usize)
            }
        }
        _ => panic!("Unrecognised Instruction")
    }
}

#[derive(Debug, Clone, Copy)]
enum Instr {
    Put(isize, char),
    Cpy(char, char),
    Inc(char),
    Dec(char),
    MemJnz(char, usize),
    IntJnz(usize, usize)
}

fn load_input() -> String {
    std::fs::read_to_string(
        std::env::args().nth(1).expect("Filepath not provided.")
    ).expect("Could not load file!")
}