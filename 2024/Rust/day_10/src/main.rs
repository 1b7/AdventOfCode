use std::collections::HashSet;

fn main() {
    let world = include_str!("../../../input/10")
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    dbg!(p1(&world));
}

fn find_trails(from: (i32, i32), world: &[Vec<u8>]) -> (usize, usize) {
    let mut frontier = vec![from];

    let mut visited = HashSet::new();
    let mut endpoints = HashSet::new();

    let moves = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut rating = 0;

    let outside_bounds = |p: (i32, i32)| {
        let (r, c) = p;
        r < 0 || r >= world.len() as i32 || c < 0 || c >= world[0].len() as i32
    };

    while frontier.len() > 0 {
        let (r, c) = frontier.pop().unwrap();

        visited.insert((r, c));
        if world[r as usize][c as usize] == 9 {
            rating += 1;
            endpoints.insert((r, c));
        }

        for (dr, dc) in moves {
            let (new_r, new_c) = (r + dr, c + dc);
            if outside_bounds((new_r, new_c)) { continue; }
            if world[new_r as usize][new_c as usize] == world[r as usize][c as usize] + 1 {
                frontier.push((r + dr, c + dc));
            }
        }
    }

    (endpoints.len(), rating)
}

fn p1(world: &[Vec<u8>]) -> (usize, usize) {

    let mut total_score = 0;
    let mut total_rating = 0;

    for r in 0..world.len() {
        for c in 0..world[0].len() {
            if world[r][c] == 0 {
                let (score, rating) = find_trails((r as i32, c as i32), world);
                total_score += score;
                total_rating += rating;
            }
        }
    }

    (total_score, total_rating)
}