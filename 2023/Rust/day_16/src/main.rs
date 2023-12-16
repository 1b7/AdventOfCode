use std::{collections::HashSet, time::Instant};

fn main() {
    let s = include_str!("../../../input/16");

    let grid: Vec<Vec<Tile>> = s.lines()
        .map(|l| l.chars().map(Tile::from_char).collect())
        .collect();
    
    let t = Instant::now();
    let p1 = simulate(((0, 0), Direction::Right), &grid);
    let e = t.elapsed().as_micros();
    println!("Part 1: {p1} ({e}us)");
    let t = Instant::now();

    let mut m = 0;
    
    let top_row =   (0, 0..grid[0].len(), Direction::Down);
    for col in top_row.1 {
        m = m.max(simulate(((top_row.0, col as i32), top_row.2), &grid));
    }

    let bot_row =   (grid.len() - 1, 0..grid[0].len(), Direction::Up);
    for col in bot_row.1 {
        m = m.max(simulate(((bot_row.0 as i32, col as i32), bot_row.2), &grid));
    }
    
    let left_col =  (0..grid.len(), 0, Direction::Right);
    for row in left_col.0 {
        m = m.max(simulate(((row as i32, left_col.1 as i32), left_col.2), &grid));
    }

    let right_col = (0..grid.len(), grid[0].len(), Direction::Left);
    for row in right_col.0 {
        m = m.max(simulate(((row as i32, right_col.1 as i32), right_col.2), &grid));
    }

    let e = t.elapsed().as_micros();
    println!("Part 2: {m} ({e}us)");

}

fn simulate(init: ((i32, i32), Direction), grid: &[Vec<Tile>]) -> usize {
    let mut states: HashSet<((i32, i32), Direction)> = HashSet::new();
    let mut beams = vec![init];

    while beams.len() > 0 {
        let beam = beams.pop().unwrap();
        if beam.0.0 < 0 || beam.0.1  < 0  || beam.0.0 >= grid.len() as i32
            || beam.0.1 >= grid[0].len() as i32  || states.contains(&beam) { continue; }

        states.insert(beam);

        match &grid[beam.0.0 as usize][beam.0.1 as usize] {
            Tile::Empty => {
                let adj = beam.1.to_coords();
                beams.push(((beam.0.0 + adj.0, beam.0.1 + adj.1), beam.1));
            },
            Tile::HorSplit => {
                if beam.1 == Direction::Up || beam.1 == Direction::Down {
                    let (left, right) = (
                        ((beam.0.0, beam.0.1 - 1), Direction::Left), 
                        ((beam.0.0, beam.0.1 + 1), Direction::Right), 
                    );
                    beams.push(left);
                    beams.push(right);
                } else {
                    let adj = beam.1.to_coords();
                    beams.push(((beam.0.0 + adj.0, beam.0.1 + adj.1), beam.1));
                }
            },
            Tile::VerSplit => {
                if beam.1 == Direction::Left || beam.1 == Direction::Right {
                    let (up, down) = (
                        ((beam.0.0 - 1, beam.0.1), Direction::Up), 
                        ((beam.0.0 + 1, beam.0.1), Direction::Down), 
                    );
                    beams.push(up);
                    beams.push(down);
                } else {
                    let adj = beam.1.to_coords();
                    beams.push(((beam.0.0 + adj.0, beam.0.1 + adj.1), beam.1));
                }

            },
            Tile::MirrorInclined => {
                let reflected = match beam.1 {
                    Direction::Down  => ((beam.0.0, beam.0.1 - 1), Direction::Left),
                    Direction::Up    => ((beam.0.0, beam.0.1 + 1), Direction::Right),
                    Direction::Left  => ((beam.0.0 + 1, beam.0.1), Direction::Down),
                    Direction::Right => ((beam.0.0 - 1, beam.0.1), Direction::Up),
                };
                beams.push(reflected);
            },
            Tile::MirrorDeclined =>{
                let reflected = match beam.1 {
                    Direction::Down  => ((beam.0.0, beam.0.1 + 1), Direction::Right),
                    Direction::Up    => ((beam.0.0, beam.0.1 - 1), Direction::Left),
                    Direction::Left  => ((beam.0.0 - 1, beam.0.1), Direction::Up),
                    Direction::Right => ((beam.0.0 + 1, beam.0.1), Direction::Down),
                };
                beams.push(reflected);
            }
        };
    }

    let visited: HashSet<(i32, i32)> = HashSet::from_iter(states.iter().map(|x| x.0));
    visited.len()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction { Up, Right, Down, Left }
impl Direction {
    pub fn to_coords(&self) -> (i32, i32) {
        match self {
            Direction::Up    => (-1,  0),
            Direction::Down  => ( 1,  0),
            Direction::Left  => ( 0, -1),
            Direction::Right => ( 0,  1),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile { Empty, HorSplit, VerSplit, MirrorInclined, MirrorDeclined }
impl Tile {
    pub fn from_char(c: char) -> Self {
        match c {
            '.'  => Tile::Empty,
            '/'  => Tile::MirrorInclined,
            '\\' => Tile::MirrorDeclined,
            '-'  => Tile::HorSplit,
            '|'  => Tile::VerSplit,
            _    => panic!("Unrecognised Character")
        }
    }
}