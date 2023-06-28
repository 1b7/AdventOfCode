const N_OUTS: usize = 25;
const N_BOTS: usize = 250;
const N_CYCLES: usize = 50;

fn main() {
    let input = load_input();
    let mut bots = vec![Bot::new(); N_BOTS];
    let mut outs = vec![0; N_OUTS];

    // Set up Bots.
    for line in input.lines() {
        let split: Vec<_> = line.split_whitespace().collect();
        match split[0] {
            "value" => {
                let val = split[1].parse::<usize>().unwrap();
                let dest = split[5].parse::<usize>().unwrap();
                bots[dest].receive(val).or_else(|| panic!("Tried to pass value to full bot."));
            },
            "bot" => {
                let from = split[1].parse::<usize>().unwrap();

                let type_low = split[5];
                let dest_low = split[6].parse::<usize>().unwrap();
                
                let type_high = split[10];
                let dest_high = split[11].parse::<usize>().unwrap();

                let compress = |o_type, dest| {
                    match o_type {
                        "bot" => Target::Bot(dest),
                        _ => Target::Output(dest)
                    }
                };

                bots[from].lo_target = compress(type_low, dest_low);
                bots[from].hi_target = compress(type_high, dest_high);
            },
            _ => panic!("Invalid Instruction!")
        }
    }

    // Execute Updates.
    for _ in 0..N_CYCLES {
        for i in 0..bots.len() {
            if !bots[i].is_done() && bots[i].is_full()  {
                let (low, high) = bots[i].get_values().unwrap();
                if low == 17 && high == 61 { println!("Part 1: {}", i); }

                match bots[i].lo_target {
                    Target::Bot(x) => { bots[x].receive(low); },
                    Target::Output(x) => { outs[x] = low; },
                    Target::None => { panic!(); }
                }
                
                match bots[i].hi_target {
                    Target::Bot(x) => { bots[x].receive(high); },
                    Target::Output(x) => { outs[x] = high; },
                    Target::None => { panic!(); }
                }
                bots[i].set_done();
            }
        }
    }
    println!("Part 2: {}", outs[0] * outs[1] * outs[2])
}

#[derive(Debug, Clone, Copy)]
enum Target { Bot(usize), Output(usize), None }

#[derive(Debug, Clone, Copy)]
struct Bot {
    lo_target: Target, hi_target: Target, values: [usize; 2], is_done: bool
}

impl Bot {
    pub fn new() -> Self {
        Self { lo_target: Target::None, hi_target: Target::None, values: [0; 2], is_done: false }
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn set_done(&mut self) { self.is_done = true; }

    pub fn receive(&mut self, x: usize) -> Option<usize> {
        if self.values[0] == 0 {
            self.values[0] = x;
            return Some(0);
        } else if self.values[1] == 0 {
            self.values[1] = x;
            return Some(1);
        } else {
            return None;
        }
    }

    /// Returns bot's values **in order**, or None if not full.
    pub fn get_values(&self) -> Option<(usize, usize)> {
        if !self.is_full() { return None }
        if self.values[0] < self.values[1] {
            Some((self.values[0], self.values[1]))
        } else {
            Some((self.values[1], self.values[0]))
        }
    }

    pub fn is_full(&self) -> bool { self.values[0] != 0 && self.values[1] != 0 }
}

fn load_input() -> String {
    std::fs::read_to_string(std::env::args().nth(1).expect("Filepath not provided."))
        .expect("Could not load file!")
}
