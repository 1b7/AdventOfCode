use std::collections::HashSet;

fn main() {
    let s = load_input();
    let chunks = s.split(',').map(|s| {
        let o = s.trim().split_at(1);
        (o.0, o.1.parse::<i32>().expect("Could not interpret string as number"))
    }).collect::<Vec<_>>();

    let (p1, p2) = compute(&chunks);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);    
}

fn compute(chunks: &Vec<(&str, i32)>) -> (i32, i32) {
    let mut visited = HashSet::new();
    let mut first_repeat = None;
    
    let as_hor_range = |x1, x2, y| { (x1..=x2).map(move |xn| (xn, y)) };
    let as_ver_range = |x, y1, y2| { (y1..=y2).map(move |yn| (x, yn)) };

    let mut facing = Facing::North;
    let (mut x, mut y) = (0, 0);
    for &(turn, distance) in chunks {
        facing = if turn == "L" { facing.left() } else { facing.right() };

        let range: Vec<(i32, i32)> = match facing {
            Facing::North => as_ver_range(x, y + 1, y + distance).collect(),
            Facing::East  => as_hor_range(x + 1, x + distance, y).collect(),
            Facing::South => as_ver_range(x, y - distance, y - 1).rev().collect(),
            Facing::West  => as_hor_range(x - distance, x - 1, y).rev().collect()
        };
        (x, y) = (range[(distance - 1) as usize].0, range[(distance - 1) as usize].1);

        if first_repeat.is_none() {
            for item in range {
                if visited.contains(&item) { first_repeat =  Some(item.0.abs() + item.1.abs()); }
                visited.insert(item);
            }
        }
    }

    (x.abs() + y.abs(), first_repeat.expect("No duplicate location found!"))
}

#[derive(Debug, Clone, Copy)]
enum Facing { North, East, South, West }
impl Facing {
    fn right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East  => Self::South,
            Self::South => Self::West,
            Self::West  => Self::North,
        }
    }

    fn left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West  => Self::South,
            Self::South => Self::East,
            Self::East  => Self::North,
        }
    }
}

fn load_input() -> String {
    std::fs::read_to_string(
        std::env::args().nth(1).expect("Filepath not provided.")
    ).expect("Could not load file!")
}