use std::{time::Instant, iter::repeat};

fn main() {
    let input = include_str!("../../../input/22");
    let s = Instant::now();

    // let p1 = p1(input);
    let p2 = p2(input);

    let e = s.elapsed();
    // println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
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
enum Instruction { TurnLeft, TurnRight, Forward }

fn parse_password(pwd: &str) -> Vec<Instruction> {
    let mut ins = vec![];
    let chrs = &mut pwd.chars().peekable();
    while chrs.peek().is_some() {
        let next = chrs.peek().unwrap();
        let instr = match next {
            'L' => { chrs.next(); ins.push(Instruction::TurnLeft); },
            'R' => { chrs.next(); ins.push(Instruction::TurnRight); },
            _ => {
                let mut nums = String::new();
                while chrs.peek().is_some() && chrs.peek().unwrap().is_numeric() {
                    nums.push(chrs.next().unwrap());
                }
                let n = nums.parse().unwrap();
                for _ in 0..n { ins.push(Instruction::Forward); }
            }
        };
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
            Instruction::Forward => pos.step(1, map),
            Instruction::TurnLeft => pos.f.turn_anticlockwise(),
            Instruction::TurnRight => pos.f.turn_clockwise()
        }
    }

    (1000 * (pos.y + 1)) + (4 * (pos.x + 1)) + pos.f as usize
}

fn move_face(idx: u8, facing: Facing, y: usize, x: usize) -> (u8, (usize, usize), Facing) {

    match idx {
        0 => match facing {
            Facing::East  => (1, (y, 0), Facing::East),
            Facing::South => (2, (0, x), Facing::South),
            Facing::West  => (3, (LENGTH - y, 0), Facing::East),
            Facing::North => (5, (x, 0), Facing::East)
        },

        1 => match facing {
            Facing::West =>  (0, (y, LENGTH - 1), Facing::East),
            Facing::South => (2, (x, LENGTH - 1), Facing::West),
            Facing::East  => (4, (LENGTH - y, LENGTH), Facing::West),
            Facing::North => (5, (LENGTH, x), Facing::South)
        },

        2 => match facing {
            Facing::North => (0, (LENGTH - 1, x), Facing::North),
            Facing::East  => (1, (LENGTH - 1, y), Facing::North),
            Facing::South => (4, (0, x), Facing::South),
            Facing::West  => (3, (0, y), Facing::South)
        },

        3 => match facing {
            Facing::East  => (4, (y, 0), Facing::East),
            Facing::South => (5, (0, x), Facing::South),
            Facing::North => (2, (x, 0), Facing::East),
            Facing::West  => (0, (LENGTH - y, 0), Facing::East)
        },

        4 => match facing {
            Facing::North => (2, (LENGTH - 1, x), Facing::North),
            Facing::West  => (3, (y, LENGTH - 1), Facing::West),
            Facing::South => (5, (x, LENGTH - 1), Facing::West),
            Facing::East  => (1, (LENGTH - y, LENGTH - 1), Facing::West)
        },

        5 => match facing {
            Facing::North => (3, (LENGTH - 1, x), Facing::North),
            Facing::East  => (4 ,(LENGTH - 1, y), Facing::North),
            Facing::South => (1, (0, x), Facing::South),
            Facing::West  => (0, (0, LENGTH - x), Facing::South),
        },
        _ => panic!()
    }
}

const LENGTH: usize = 50;

fn p2(input: &str) -> usize {
    let mut faces = vec![];
    let (map_str, password) = input.split_once("\n\n").unwrap();
    let map = &parse_map(map_str);

    let (dim, (iy, ix)) = (0, get_first_open(map));
    let pos = Position { y: iy, x: ix, f:  Facing::East };
    let mut pos = (dim, pos);
    let instructions = parse_password(password);

    let mut starts: Vec<(usize, usize)> = vec![];
    for y in (0..(LENGTH * 3)).step_by(LENGTH) {
        for x in (0..(LENGTH * 3)).step_by(LENGTH) {
            if map[y][x] != Tile::Void {
                let mut new_face = vec![];
                for dy in y..(y + LENGTH) {
                    let mut new_row = vec![];
                    for dx in x..(x + LENGTH) {
                        new_row.push(map[dy][dx])
                    }
                    starts.push((y, x));
                    new_face.push(new_row);
                }
                faces.push(new_face);
            }
        }
    }

    for instruction in instructions {
        match instruction {
            Instruction::TurnLeft => pos.1.f.turn_anticlockwise(),
            Instruction::TurnRight => pos.1.f.turn_clockwise(),
            Instruction::Forward => {
                let move_fn = match pos.1.f {
                    Facing::North => Position::up,
                    Facing::East => Position::right,
                    Facing::South => Position::down,
                    Facing::West => Position::left
                };

                let try_move = move_fn(&pos.1, LENGTH, LENGTH);
                if let Some(new_pos) = try_move {
                    pos.1.y = new_pos.0;
                    pos.1.x = new_pos.1;
                } else {
                    let (nd, (ny, nx), nf) = move_face(pos.0, pos.1.f, pos.1.y, pos.1.x);
                    pos = (nd, Position { x: nx, y: ny, f: nf });
                }

            }
        }
    }

    dbg!(pos);

    let final_x = pos.1.x + starts[pos.0 as usize].1;
    let final_y = pos.1.y + starts[pos.0 as usize].0;


    (1000 * (final_y + 1)) + (4 * (final_x + 1)) + pos.1.f as usize
}
// 32275 too low