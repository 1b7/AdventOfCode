use std::collections::{HashMap, HashSet};

fn main() {
    let garden = include_str!("../../../input/12")
        .lines()
        .map(|line| line.chars().map(|c| c as u8).collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();


    let (p1, p2) = compute(&garden);
    println!("Part 1: {p1}\nPart 2: {p2}");

}

fn compute(garden: &[Vec<u8>]) -> (usize, usize) {
    let mut visited = HashSet::new();

    let mut sum = 0;
    let mut bulk_sum = 0;
    for row in 0..garden.len() {
        for col in 0..garden[0].len() {
            if visited.contains(&(row as i32, col as i32)) { continue; }

            let old_visited = visited.clone();
            let (p, a) = search(&garden, row as i32, col as i32, &mut visited);
            sum += p * a;


            let region = visited.difference(&old_visited).collect::<HashSet<_>>();
            let sides = count_sides(&region);
            bulk_sum += a * sides;
        }
    }

    (sum, bulk_sum)
}

fn count_sides(region: &HashSet<&(i32, i32)>) -> usize {
    let mut exterior = 0;
    let mut interior = 0;

    // Find corners; if we're at a corner, then there must be a wall!
    for (r, c) in region {
        let (row, col) = (*r, *c);
        if !region.contains(&(row - 1, col)) && !region.contains(&(row, col - 1)) {
            exterior += 1;
        }
        if !region.contains(&(row - 1, col)) && !region.contains(&(row, col + 1)) {
            exterior += 1;
        }
        if !region.contains(&(row + 1, col)) && !region.contains(&(row, col - 1)) {
            exterior += 1;
        }
        if !region.contains(&(row + 1, col)) && !region.contains(&(row, col + 1)) {
            exterior += 1;
        }

        if region.contains(&(row - 1, col)) && region.contains(&(row, col - 1)) && !region.contains(&(row - 1, col - 1)) {
            interior += 1;
        }
        if region.contains(&(row - 1, col)) && region.contains(&(row, col + 1)) && !region.contains(&(row - 1, col + 1)) {
            interior += 1;
        }
        if region.contains(&(row + 1, col)) && region.contains(&(row, col - 1)) && !region.contains(&(row + 1, col - 1)) {
            interior += 1;
        }
        if region.contains(&(row + 1, col)) && region.contains(&(row, col + 1)) && !region.contains(&(row + 1, col + 1)) {
            interior += 1;
        }
    }

    exterior + interior
}

fn search(garden: &[Vec<u8>], from_row: i32, from_col: i32, visited: &mut HashSet<(i32, i32)>) -> (usize, usize) {
    let plant = garden[from_row as usize][from_col as usize];

    let mut frontier = vec![(from_row, from_col)];
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut perimeter = 0;
    let mut area = 0;

    while frontier.len() > 0 {
        let (r, c) = frontier.pop().unwrap();
        visited.insert((r, c));
        area += 1;

        for (dr, dc) in directions {
            let (new_r, new_c) = (r + dr, c + dc);

            // Bounds Checks
            if new_r < 0 || new_c < 0 || new_r >= garden.len() as i32 || new_c >= garden[0].len() as i32 {
                perimeter += 1;
                continue;
            }

            // Plant Match Check
            if garden[new_r as usize][new_c as usize] != plant {
                perimeter += 1;
            } else if !visited.contains(&(new_r, new_c)) && !frontier.contains(&(new_r, new_c)) {
                frontier.push((new_r, new_c));
            }
        }
    }

    (perimeter, area)
}
