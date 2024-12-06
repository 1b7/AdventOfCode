use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Direction { Up, Down, Left, Right }

impl Direction {
    fn movement(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum State { Free, Occupied }

fn main() {
    let input = include_str!("../../../input/06");

    let mut map: Vec<Vec<State>> = vec![];
    let mut start = (0, 0);
    for (row, line) in input.lines().enumerate() {
        let mut new_row = vec![State::Free; line.len()];
        for (col, c) in line.chars().enumerate() {
            match c {
                '^' => { start = (col as i32, row as i32); },
                '#' => { new_row[col] = State::Occupied; },
                _ => ()
            }
        }
        map.push(new_row);
    }

    let (p1, p2) = compute(start, &mut map);
    println!("Part 1: {p1}\nPart 2: {p2}");

}

/// Simulates the guard's movements from some starting point.
/// Returns a boolean indicating the presence of a cycle, and the distinct locations visited.
fn run(start: (i32, i32), map: &[Vec<State>]) -> (bool, HashSet<(i32, i32)>) {
    let mut direction = Direction::Up;
    let mut pos = start;
    let mut visited: HashSet<((i32, i32), Direction)> = HashSet::new();

    while (pos.0 >= 0 && pos.0 < map[0].len() as i32 && pos.1 >= 0 && pos.1 < map.len() as i32) {
        // If we're revisiting the same cell while travelling in the same direction,
        // we are cycling.
        if visited.contains(&(pos, direction)) {
            return (true, visited.into_iter().map(|x| x.0).collect());
        }
        visited.insert((pos, direction));

        let option = direction.movement();
        let new_pos = (pos.0 + option.0, pos.1 + option.1);

        // The guard has gone out of bounds:
        if new_pos.0 < 0 || new_pos.0 >= map[0].len() as i32 || new_pos.1 < 0 || new_pos.1 >= map.len() as i32 {
            break;
        }

        // If the guard will run into an obstacle, turn instead.
        if map[new_pos.1 as usize][new_pos.0 as usize] == State::Occupied {
            direction = direction.turn_right();
        } else {
            pos = new_pos;
        }
    }

    (false, visited.into_iter().map(|x| x.0).collect())
}

fn compute(start: (i32, i32), map: &mut Vec<Vec<State>>) -> (usize, usize) {
    // Part 1: Find the unique positions visited from the start position of the base map:
    let (_, positions) = run(start, map);

    // Part 2: Try adding obstacles to each of those paths in turn;
    // Count the paths which now end in a cycle.
    let mut cycles = 0;
    for &position in positions.iter() {
        if position == start { continue; }
        map[position.1 as usize][position.0 as usize] = State::Occupied;
        if run(start, map).0 { cycles += 1; }
        map[position.1 as usize][position.0 as usize] = State::Free;
    }

    (positions.len(), cycles)
}