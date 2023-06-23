use std::{time::{Instant, Duration}, collections::HashSet, thread::sleep, fmt::Write};
use colored::*;

fn main() {
    let input = include_str!("../../../input/24");

    let s = Instant::now();
    let width = input.lines().next().unwrap().trim().len() as u8;
    let height = input.lines().count() as u8;

    let walls = create_walls(width, height);
    let bliz_states = generate_blizzard_states(get_grid(input), &walls, width, height);

    let p1 = search(input, &bliz_states, &walls, false, 0, false);
    let e = s.elapsed();
    let p2_a = p1 + search(input, &bliz_states, &walls, false, p1, true);
    let p2_b = p2_a + search(input, &bliz_states, &walls, true, p2_a, false);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2_b);
    println!("Took: {}ms", e.as_millis());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point { y: u8, x: u8 }
impl Point {
    pub fn new(y: u8, x: u8) -> Self { Point { y, x } }
    pub fn up (self) -> Option<Self> {
        if self.y > 0 {
            Some(Point { y: self.y - 1, x: self.x })
        } else { None }
    }
    pub fn down (self, mh: u8) -> Option<Self> {
        if self.y < mh {
            Some(Point { y: self.y + 1, x: self.x })
        } else { None }
    }
    pub fn left  (self) -> Option<Self> { 
        if self.x > 0 {
            Some(Point { y: self.y, x: self.x - 1 })
        } else { None }
    }
    pub fn right (self, mw: u8) -> Option<Self> { 
        if self.x < mw {
            Some(Point { y: self.y, x: self.x + 1 })
        } else { None }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction { Up, Right, Down, Left }

fn get_grid(s: &str) -> Vec<(Point, Direction)> {
    Vec::from_iter(
        s.lines().enumerate().flat_map(|(y, line)| {
            line.char_indices().filter_map(|(x, c)| {
                let point = Point::new(y as u8, x as u8);
                match c {
                    '>' => Some((point, Direction::Right)),
                    '<' => Some((point, Direction::Left)),
                    '^' => Some((point, Direction::Up)),
                    'v' => Some((point, Direction::Down)),
                    _   => None
                }
            }).collect::<Vec<_>>()
        })
    )
}

fn draw(path: &[Point], bs: &HashSet<Point>, ws: &HashSet<Point>, w: u8, h: u8) {
    let mut out = String::new();
    for y in 0..h {
        for x in 0..w {
            let p = Point::new(y, x);
            let c = if path.contains(&p) {
                if *path.last().unwrap() == p {
                    "█".bold().bright_cyan()
                } else {
                    "█".dimmed().cyan()
                }
            } else if bs.contains(&p) {
                "*".dimmed()
            } else if ws.contains(&p) {
                "█".dimmed()
            } else {
                " ".normal()
            };
            write!(&mut out, "{}", c).unwrap();
        }
        writeln!(&mut out).unwrap();
    }
    println!("{}", out);
}

fn create_walls(width: u8, height: u8) -> HashSet<Point> {
    let mut walls = HashSet::new();
    for x in 0..width {
        if x != 1 {
            walls.insert(Point::new(0, x));
        }
        if x != width - 2 {
            walls.insert(Point::new(height - 1, x));
        }
    }
    for y in 0..height {
        walls.insert(Point::new(y, 0));
        walls.insert(Point::new(y, width - 1));
    }
    walls
}

fn generate_blizzard_states(init: Vec<(Point, Direction)>, walls: &HashSet<Point>, width: u8, height: u8) -> Vec<HashSet<Point>> {
    let mut blizzards = init;
    let mut blizzard_states: Vec<HashSet<Point>> = vec![];
    blizzard_states.push(blizzards.clone().into_iter().map(|(p, _)| p).collect());
    for _ in 1..(width as usize * height as usize) {
        for (point, direction) in &mut blizzards {
            let mut new_point = match direction {
                Direction::Up => point.up(),
                Direction::Down => point.down(height),
                Direction::Left => point.left(),
                Direction::Right => point.right(width)
            }.unwrap();
            
            if walls.contains(&new_point) {
                match direction {
                    Direction::Up => new_point.y = height - 2,
                    Direction::Down => new_point.y = 1,
                    Direction::Left => new_point.x = width - 2,
                    Direction::Right => new_point.x = 1
                }
            }
            *point = new_point;
        }
        let condensed = blizzards.clone().into_iter().map(|(p, _)| p).collect();
        blizzard_states.push(condensed);
    }
    blizzard_states
}

fn search(s: &str, bliz_states: &[HashSet<Point>], walls: &HashSet<Point>, draw_path: bool, offset: usize, switch_direction: bool) -> usize {
    let width = s.lines().next().unwrap().trim().len() as u8;
    let height = s.lines().count() as u8;
    
    let mut start = Point::new(0, 1);
    let mut goal = Point::new((height - 1) as u8, width - 2);
    if switch_direction { std::mem::swap(&mut start, &mut goal); }
    
    fn manhattan (f: Point, t: Point) -> usize { 
        f.x.abs_diff(t.x) as usize + f.y.abs_diff(t.y) as usize 
    }
    
    let mut paths: Vec<(usize, Vec<Point>)> = vec![];
    paths.push((manhattan(start, goal), vec![start]));

    // BinaryHeap seems to be slower, but is an interesting alternative method.
    // let mut paths: BinaryHeap<(isize, Vec<Point>)> = BinaryHeap::new();
    // paths.push((-(mhtn(start, goal) as isize), vec![start]));

    let mut starts = HashSet::new();
    while !paths.is_empty() {
        let (_, path) = paths.pop().unwrap();
        let next_board = &bliz_states[(path.len() + offset) % bliz_states.len()];
        let node = *path.last().unwrap();
        
        if node == goal {
            if draw_path {
                for i in 0..path.len() {
                    sleep(Duration::from_millis(150));
                    println!("\n{}", i);
                    draw(&path[0..i], &bliz_states[i], walls, width, height);
                }
                println!("\nFinal");
                draw(&path, &bliz_states[path.len()], walls, width, height);
            }
            return path.len() - 1
        }

        if starts.contains(&(node, path.len())) { continue; }
        starts.insert((node, path.len()));

        [Some(node), node.down(height), node.right(width), node.left(), node.up()]
            .into_iter()
            .flatten()
            .for_each(|option| {
                if !next_board.contains(&option) && !walls.contains(&option) {
                    let mut new_path = path.clone();
                    new_path.push(option);
                    let new_h = manhattan(node, goal) + new_path.len();
                    let res = match paths.binary_search_by(|o| new_h.cmp(&o.0)) {
                        Ok(n) => n,
                        Err(n) => n
                    };
                    paths.insert(res, (new_h, new_path));
                    // paths.push(((-(new_h as isize)), new_path));
                }
            });
    }
    panic!("No path found");
}