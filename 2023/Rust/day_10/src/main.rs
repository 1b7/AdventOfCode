use std::collections::HashSet;
use colored::*;

fn char_to_pipe(c: char) -> Option<Pipe> {
    match c {
        '|' => Some(Pipe::Vertical),
        '-' => Some(Pipe::Horizontal),
        'L' => Some(Pipe::NorthToEast),
        'J' => Some(Pipe::NorthToWest),
        '7' => Some(Pipe::SouthToWest),
        'F' => Some(Pipe::SouthToEast),
        'S' => Some(Pipe::Start),
        _  => None
    }
}

fn pipe_to_char(p: Pipe) -> char {
    match p {
        Pipe::Vertical => '│',
        Pipe::Horizontal => '─',
        Pipe::NorthToEast => '└',
        Pipe::NorthToWest => '┘',
        Pipe::SouthToWest => '┐',
        Pipe::SouthToEast => '┌',
        Pipe::Start => '█'
    }

}

fn main() {
    let input = include_str!("../../../input/10");
    let grid = input.lines().map(|line| {
        line.chars().map(char_to_pipe).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let (height, width) = (grid.len() as i32, grid[0].len() as i32);
    let mut start = (0, 0);
    for row in 0..height {
        for col in 0..width {
            if grid[row as usize][col as usize] == Some(Pipe::Start) { start = (row, col);  break; }
        }
    }

    let moves = [
        (-1,  0, [Pipe::Vertical, Pipe::SouthToEast, Pipe::SouthToWest, Pipe::Start  ]), 
        ( 0, -1, [Pipe::Horizontal, Pipe::NorthToEast, Pipe::SouthToEast, Pipe::Start]), 
        ( 0,  1, [Pipe::Horizontal, Pipe::NorthToWest, Pipe::SouthToWest, Pipe::Start]), 
        ( 1,  0, [Pipe::Vertical, Pipe::NorthToEast, Pipe::NorthToWest, Pipe::Start  ])
    ];
    
    // Use DFS to find longest path with no repetitions.
    // let mut visited = HashSet::new();
    let mut frontier = vec![];
    frontier.push(vec![start]);
    let mut max_journey = vec![];

    while !frontier.is_empty() {
        let journey = frontier.pop().unwrap();
        let pos = journey[journey.len() - 1];

        if journey.len() > max_journey.len() {
            max_journey = journey.clone();
        }

        if journey.len() > 1 && journey[journey.len() - 1] == start { panic!() }

        for mov in moves  {
            let cell = (pos.0 + mov.0, pos.1 + mov.1);
            if journey.contains(&cell) || cell.0 < 0 || cell.1 < 0 || cell.0 >= height || cell.1 >= width { continue }
            if let Some(pipe) = &grid[cell.0 as usize][cell.1 as usize] {
                if mov.2.contains(pipe) {
                    let mut new_journey = journey.clone();
                    new_journey.push((cell.0, cell.1));
                    frontier.push(new_journey);
                }
            }
        }
    }
    
    
    // Idea: We can use a flood fill from the outside edges of the map after adding
    // a buffer around the edge.
    // Any tiles which aren't reachable by treating the 'visited' nodes as walls,
    // must therefore be enclosed by those walls.
    // (In practice this doesn't quite work because pipes can run parallel to one another)

    let mut grid = grid.into_iter().map(|mut row| {
        row.insert(0, None);
        row.push(None);
        row
    }).collect::<Vec<_>>();
    let width = width + 2;
    let height = height + 2;
    grid.push(vec![None; width as usize]);
    grid.insert(0, vec![None; width as usize]);

    
    let mut flooded = HashSet::new();

    let directions: [(i32, i32); 4] = [ (-1,  0), (0, -1), (0,  1), (1,  0) ];

    let mut points = vec![(0, 0)];
    while points.len() > 0 {
        let point = points.pop().unwrap();
        flooded.insert(point);
        for d in directions {
            let p = (d.0 + point.0, d.1 + point.1);
            if p.0 < 0 || p.1 < 0 || p.0 >= height || p.1 >= width { continue; }

            if !max_journey.contains(&(p.0 - 1, p.1 - 1)) && !flooded.contains(&p) {
                points.push((p.0, p.1));
            }
        }
    }

    for row in 0..height {
        for col in 0..width {
            if flooded.contains(&&(row, col)) {
                print!("{}", String::from(" "));
            } else if max_journey.contains(&(row - 1, col - 1)) {
                print!("{}", String::from(pipe_to_char(grid[row as usize][col as usize].unwrap())).red());
            } else {
                print!("{}", String::from("#").blue());
            }
        }
        println!()
    }
    println!("Part 1: {}", max_journey.len() / 2);
    println!("Part 2: Counted from the map printed out!");
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pipe { Vertical, Horizontal, NorthToEast, NorthToWest, SouthToEast, SouthToWest, Start }