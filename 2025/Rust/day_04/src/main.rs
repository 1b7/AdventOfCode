fn main() {
    let mut warehouse: Vec<Vec<bool>> = include_str!("../../../../input/2025/04")
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect();

    let part_one = find_accessible(&warehouse).len();
    let part_two = remove_all(&mut warehouse);

    dbg!(part_one, part_two);
}

fn remove_all(warehouse: &mut [Vec<bool>]) -> usize {
    let mut total_removed = 0;

    loop {
        let removed = remove(warehouse);
        if removed == 0 { break; }
        total_removed += removed;
    }

    total_removed
}

fn remove(warehouse: &mut [Vec<bool>]) -> usize {
    let targets = find_accessible(&warehouse);

    for &(row, col) in &targets {
        warehouse[row][col] = false;
    }

    targets.len()
}

fn find_accessible(warehouse: &[Vec<bool>]) -> Vec<(usize, usize)> {
    let mut accessible = vec![];

    for row in 0..warehouse.len() {
        for col in 0..warehouse[0].len() {
            if !warehouse[row][col] { continue; }
            if check_adjacent(warehouse, row, col) < 4 { accessible.push((row, col)); }
        }
    }

    accessible
}

fn check_adjacent(warehouse: &[Vec<bool>], row: usize, col: usize) -> usize {
    let mut neighbours = 0;

    if row > 0 && col > 0 && warehouse[row - 1][col - 1] { neighbours += 1; } // Up & Left
    if row > 0 && warehouse[row - 1][col] { neighbours += 1; } // Up
    if row > 0 && col < warehouse[0].len() - 1 && warehouse[row - 1][col + 1] { neighbours += 1; } // Up & Right

    if col > 0 && warehouse[row][col - 1] { neighbours += 1; } //  Left
    if col < warehouse[0].len() - 1 && warehouse[row][col + 1] { neighbours += 1; } // Right

    if row < warehouse.len() - 1 && col > 0 && warehouse[row + 1][col - 1] { neighbours += 1; } // Down & Left
    if row < warehouse.len() - 1 && warehouse[row + 1][col] { neighbours += 1; } // Down
    if row < warehouse.len() - 1 && col < warehouse[0].len() - 1 && warehouse[row + 1][col + 1] { neighbours += 1; } // Down & Right


    neighbours
}