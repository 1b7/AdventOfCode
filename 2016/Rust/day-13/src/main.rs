use std::collections::HashSet;

fn main() {
    let favourite = load_input().parse().expect("Could not read favourite number.");
    println!("Part 1: {}", bfs(favourite, (31, 39), false));
    println!("Part 2: {}", bfs(favourite, (31, 39), true));
}

// Searches for a target pair of coordinates; if that target cannot be found (within
// a limit of 50 moves, if limit is set), instead returns the number of unique 
// reachable positions.
fn bfs(offset: usize, target: (usize, usize), limit: bool) -> usize {
    let mut depth = 0;
    let mut paths = vec![vec![(1, 1)]];
    let mut visited = HashSet::new();

    while !limit || depth < 50 {
        depth += 1;
        let mut i = 0;
        let end = paths.len();
        while i < end {
            let path = paths[i].clone();
            let pos = path[path.len() - 1];
            if pos == target { return path.len() - 1 }

            let mut insert = |nx, ny| {
                if visited.contains(&(nx, ny)) { return; }
                visited.insert((nx, ny));
                let mut new_path = path.clone();
                new_path.push((nx, ny));
                paths.push(new_path);
            };

            if is_space(pos.0 + 1, pos.1, offset) { insert(pos.0 + 1, pos.1) };
            if is_space(pos.0, pos.1 + 1, offset) { insert(pos.0, pos.1 + 1) };
            if pos.0 > 0 && is_space(pos.0 - 1, pos.1, offset) { insert(pos.0 - 1, pos.1) };
            if pos.1 > 0 && is_space(pos.0, pos.1 - 1, offset) { insert(pos.0, pos.1 - 1) };
            i += 1;
        }
        paths = paths[end..].to_vec();
    }

    visited.len()
}

fn is_space(x: usize, y: usize, offset: usize) -> bool {
    let sum = x * x + 3 * x + 2 * x * y + y + y * y + offset;
    sum.count_ones() % 2 == 0
}

fn load_input() -> String {
    std::fs::read_to_string(
        std::env::args().nth(1).expect("Filepath not provided.")
    ).expect("Could not load file!")
}