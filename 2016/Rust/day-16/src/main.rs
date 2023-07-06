use std::fmt::Display;

fn main() {
    println!("Part 1: {}", checksum(272));
    println!("Part 2: {}", checksum(35651584));
}

fn checksum(capacity: usize) -> String {
    let mut init = bits_from_str(&load_input());
    while init.len() < capacity { expand(&mut init); }

    let mut check: Vec<_> = init[..capacity].to_vec();
    while check.len() % 2 == 0 {
        check = check.chunks(2)
            .map(|chunk| Bit::xnor(chunk[0], chunk[1]))
            .collect();
    }
    
    String::from_iter(check.iter().map(|b| b.to_string()))
}

fn expand(bits: &mut Vec<Bit>) {
    let copy = bits.clone();
    let copy = copy.iter().rev().map(Bit::toggle);
    bits.push(Bit::Off);
    bits.extend(copy);
}

fn bits_from_str(s: &str) -> Vec<Bit> {
    s.chars().map(|c| match c {
        '0' => Bit::Off,
        '1' => Bit::On,
         _  => panic!()
    }).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bit { Off, On }

impl Bit {
    pub fn toggle(bit: &Self) -> Self {
        match bit {
            Bit::Off => Bit::On,
            Bit::On  => Bit::Off
        }
    }

    pub fn xnor(a: Self, b: Self) -> Self {
        if a == b { Bit::On } else { Bit::Off }
    }
}

impl Display for Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Bit::Off => '0',
            Bit::On => '1'
        };
        write!(f, "{}", c)
    }
}

fn load_input() -> String {
    std::fs::read_to_string(
        std::env::args().nth(1).expect("Filepath not provided.")
    ).expect("Could not load file!")
}