use std::{time::Instant, iter::repeat};

fn main() {
    let input = include_str!("../../../input/22");
    let s = Instant::now();

    assert_eq!(Facing::East as usize, 0);
    assert_eq!(Facing::South as usize, 1);
    assert_eq!(Facing::West as usize, 2);
    assert_eq!(Facing::North as usize, 3);
    
    let p1 = p1(input);
    // let p2 = p2(input);

    let e = s.elapsed();
    println!("Part 1: {}", p1);
    // println!("Part 2: {}", p2);
    println!("Took: {}ms", e.as_millis());
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum Facing { East, South, West, North }
impl Facing {
    pub fn turn_clockwise(&mut self) {
        *self = match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North
        };
    }

    pub fn turn_anticlockwise(&mut self) {
        *self = match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile { Open, Wall, Void }
impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            ' ' => Tile::Void,
            '#' => Tile::Wall,
            '.' => Tile::Open,
            _   => panic!()
        }
    }
}

fn parse_map(s: &str) -> Vec<Vec<Tile>> {
    let mut map: Vec<Vec<Tile>> = s.lines()
        .map(|line| line.trim_matches('\n').chars().map(Tile::from_char).collect())
        .collect();
    
    // Making all row widths equal (avoids potential headache with indexing into rows)
    let max_width = map.iter().map(|x| x.len()).max().unwrap();
    for row in &mut map {
        while row.len() < max_width {
            row.push(Tile::Void)
        }
    }

    map
}

fn get_first_open(map: &Vec<Vec<Tile>>) -> (usize, usize) {
    for (r, row) in map.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if *col == Tile::Open { 
                return (r, c)
            }
        }
    }
    panic!("Not found");
}

#[derive(Debug, Clone, Copy)]
struct Position { x: usize, y: usize, f: Facing }
impl Position {
    fn up(&self, _w: usize, _h: usize) -> Option<(usize, usize)> { 
        if self.y > 0 { Some((self.y - 1, self.x )) } else { None }
    }
    fn down(&self, _w: usize, h: usize) -> Option<(usize, usize)> { 
        if self.y < (h - 1) { Some((self.y + 1, self.x)) } else { None }
    }
    fn left(&self, _w: usize, _h: usize) -> Option<(usize, usize)> { 
        if self.x > 0 { Some(( self.y, self.x - 1)) } else { None }
    }
    fn right(&self, w: usize, _h: usize) -> Option<(usize, usize)> { 
        if self.x < (w - 1) { Some(( self.y, self.x + 1)) } else { None }
    }

    pub fn step(&mut self, n: usize, map: &Vec<Vec<Tile>>) {
        let width = map[0].len();
        let height = map.len();

        let travel = match self.f {
            Facing::North => Self::up,
            Facing::East => Self::right,
            Facing::South => Self::down,
            Facing::West => Self::left
        };

        fn wrap(mut s: Position, row: usize, col: usize, height: usize, width: usize, map: &Vec<Vec<Tile>>) -> Option<Position> {
            let coords: Vec<_> = match s.f {
                Facing::North => ((row..height).rev().zip(repeat(col))).collect(),
                Facing::South => (0..row).zip(repeat(col)).collect(),
                Facing::East  => (repeat(row).zip(0..col)).collect(),
                Facing::West  => repeat(row).zip((col..width).rev()).collect()
            };

            for (y, x) in coords {
                match map[y][x] {
                    Tile::Open => {
                        s.x = x; 
                        s.y = y;
                        return Some(s);
                    }
                    Tile::Void => continue,
                    Tile::Wall => return None,
                }
            }
            panic!("Wrapping failed; did not find either a Wall or Open tile")
        }

        for _ in 0..n {
            let res = travel(self, width, height);

            if let Some((row, col)) = res {
                match map[row][col] {
                    Tile::Open => { self.x = col; self.y = row; },
                    Tile::Wall => return,
                    Tile::Void => {
                        if let Some(new_pos) = wrap(*self, row, col, height, width, map) {
                            *self = new_pos;
                        } else {
                            return
                        }
                    }
                }
            } else {
               if let Some(new_pos) = wrap(*self, self.y, self.x, height, width, map) {
                *self = new_pos
               } else {
                    return
               }
            }
        }

    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction { TurnLeft, TurnRight, Forward(usize) }

fn parse_password(pwd: &str) -> Vec<Instruction> {
    let mut ins = vec![];
    let chrs = &mut pwd.chars().peekable();
    while chrs.peek().is_some() {
        let next = chrs.peek().unwrap();
        let instr = match next {
            'L' => { chrs.next(); Instruction::TurnLeft },
            'R' => { chrs.next(); Instruction::TurnRight },
            _ => {
                let mut nums = String::new();
                while chrs.peek().is_some() && chrs.peek().unwrap().is_numeric() {
                    nums.push(chrs.next().unwrap());
                }
                Instruction::Forward(nums.parse().unwrap())
            }
        };
        ins.push(instr);
    }
    ins
}

fn p1(input: &str) -> usize {
    let (map_str, password) = input.split_once("\n\n").unwrap();
    let map = &parse_map(map_str);

    let (iy, ix) = get_first_open(map);
    let mut pos = Position { y: iy, x: ix, f:  Facing::East };
    let instructions = parse_password(password);

    for instruction in &instructions {
        match instruction {
            Instruction::Forward(n) => pos.step(*n, map),
            Instruction::TurnLeft => pos.f.turn_anticlockwise(),
            Instruction::TurnRight => pos.f.turn_clockwise()
        }
    }

    (1000 * (pos.y + 1)) + (4 * (pos.x + 1)) + pos.f as usize
}

fn p2(input: &str) -> usize {
    todo!()
}