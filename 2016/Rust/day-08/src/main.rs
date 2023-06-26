use std::fmt::Display;
use regex::Regex;

fn main() {
    let s = load_input();
    run(&s);
}

fn run(instructions: &str) {
    let re = Regex::new(r"((?:rect)|(?:row)|(?:column))[a-z= ]*(\d+)\D+(\d+)").unwrap();
    let mut screen = Screen::new();
    for l in re.captures_iter(instructions) {
        let (code, a, b) = (l.get(1).unwrap().as_str(), l.get(2).unwrap().as_str(), l.get(3).unwrap().as_str());
        let (a, b) = (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap());
        screen = match code {
            "rect" => screen.rect(a, b),
            "row" => screen.rot_row(a, b),
            "column" => screen.rot_col(a, b),
            _ => panic!("Invalid instruction")
        };
    }
    
    println!("Part 1: {}", screen.count_lit());
    println!("Part 2:\n{}", screen);
}

const D_WIDTH: usize = 50;
const D_HEIGHT: usize = 6;

#[derive(Debug, Clone, Copy)]
struct Screen([[bool; D_WIDTH]; D_HEIGHT]);

impl Screen {
    pub fn new() -> Self { Self([[false; D_WIDTH]; D_HEIGHT])  }

    pub fn rect(&self, a: usize, b: usize) -> Self {
        let mut new = self.clone();
        for row in 0..b {
            for col in 0..a {
                new.0[row as usize][col as usize] = true;
            }
        }
        new
    }

    pub fn rot_row(&self, row: usize, by: usize) -> Self {
        let mut new = self.clone();
        for x in 0..D_WIDTH {
            new.0[row][(x + by) % D_WIDTH] = self.0[row][x]
        }
        new
    }

    pub fn rot_col(&self, col: usize, by: usize) -> Self {
        let mut new = self.clone();
        for y in 0..D_HEIGHT {
            new.0[(y + by) % D_HEIGHT][col] = self.0[y][col]
        }
        new
    }

    pub fn count_lit(&self) -> usize {
        let mut count = 0;
        for row in 0..D_HEIGHT {
            for col in 0..D_WIDTH {
                if self.0[row][col] { count += 1; }
            }
        }
        count
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for row in 0..D_HEIGHT {
            for col in 0..D_WIDTH {
                output.push(if self.0[row][col] { '█' } else { ' ' });
            }
            output.push('\n')
        }
        write!(f, "{}", output)
    }
}

fn load_input() -> String {
    std::fs::read_to_string(
        std::env::args().nth(1).expect("Filepath not provided.")
    ).expect("Could not load file!")
}