use std::{collections::{BinaryHeap, HashMap}, cmp::Reverse, time::Instant};

fn main() {
    let input: Vec<Vec<u32>> = include_str!("../../../input/17")
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let t = Instant::now();
    let p1 = p1(&input);
    let e = t.elapsed().as_micros();
    println!("Part 1: {p1} ({e}us)");

    let t = Instant::now();
    let p2 = p2(&input);
    let e = t.elapsed().as_micros();
    println!("Part 1: {p2} ({e}us)");
}

fn p1(grid: &[Vec<u32>]) -> usize { search(grid, false) }
fn p2(grid: &[Vec<u32>]) -> usize { search(grid, true) }

// Using an A*-style search (with a frontier implemented as a min-heap),
// uses a heuristic (based on the manhattan-distance to the goal) to efficiently
// find a minimal-cost path to the goal. 
// This approach mitigates the need to explore sub-optimal paths.
fn search(grid: &[Vec<u32>], p2: bool) -> usize {
    let mut queue = BinaryHeap::new();
    let mut mins = HashMap::new();

    let dist = ((grid.len() - 1) + (grid[0].len() - 1)) as u32;

    let start = (Reverse(dist), 0, (0, 0), (Dir::S, 0));
    let dest = ((grid.len() - 1) as i32, (grid[0].len() - 1) as i32);
    queue.push(start);

    while queue.len() > 0 {
        let (_heuristic, loss, node, direction) = queue.pop().unwrap();

        // Adhere to bounds on movement in a straight line:
        if (p2 && direction.1 > 10) || (!p2 && direction.1 > 3) { continue; }

        if (!p2 || direction.1 >= 4) && node == dest { return loss as usize }

        for d in [(-1, 0, Dir::U), (0, 1, Dir::R), (1, 0, Dir::D), (0, -1, Dir::L)] {
            // Don't try to immediately move backwards:
            if Dir::is_opposite(direction.0, d.2) { continue; }

            // Test Part 2's minimal movement requirement:
            if p2 && direction.0 != Dir::S && direction.1 < 4 && direction.0 != d.2 { continue; }

            let (nx, ny) = (node.0 + d.0, node.1 + d.1);
            if nx < 0 || nx >= grid.len() as i32 || ny < 0 || ny >= grid[0].len() as i32 { continue; }
            let new_dir = (d.2, if direction.0 == d.2 { direction.1 + 1 } else { 1 });
            let new_loss = loss + grid[nx as usize][ny as usize];

            // Calculate heuristic as manhattan distance - which is admissible.
            let new_h = Reverse(new_loss + (dist - ((nx + 1) + (ny + 1)) as u32));

            // Test if this is a new shortest path, and if so, add it to the frontier:
            if !mins.get(&((nx, ny), new_dir)).and_then(|&v| (v <= new_loss).then_some(0)).is_some() {
                mins.insert(((nx, ny), new_dir), new_loss);
                queue.push((new_h, new_loss, (nx, ny), new_dir));
            }
        }
    }
    panic!("No path found!")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir { U, R, D, L, S }

impl Dir {
    pub fn is_opposite(x: Self, y: Self) -> bool {
        match x {
            Dir::L => y == Dir::R,
            Dir::R => y == Dir::L,
            Dir::U => y == Dir::D,
            Dir::D => y == Dir::U,
            Dir::S => false
        }
    }
}