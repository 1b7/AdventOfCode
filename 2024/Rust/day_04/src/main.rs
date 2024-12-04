fn main() {
    let wordsearch: Vec<Vec<char>> = include_str!("../../../input/04")
        .lines()
        .map(|line| line.chars().collect()).collect();

    println!("Part 1: {}", p1(&wordsearch, "XMAS"));
    println!("Part 2: {}", p2(&wordsearch, "MAS"));
}

fn p1(grid: &[Vec<char>], s: &str) -> usize {
    let target: Vec<_> = s.chars().collect();
    let reverse: Vec<_> = s.chars().rev().collect();

    // Rows
    let mut count = 0;
    for row in grid.iter() {
        for window in row.windows(target.len()) {
            if window == target || window == reverse {  count += 1; }
        }
    }

    // Columns
    for col in 0..grid.len() {
        for row in 0..(grid.len() - target.len() + 1) {
            let window: Vec<_> = (0..target.len()).map(|i| grid[row + i][col]).collect();
            if window == target || window == reverse {  count += 1; }
        }
    }

    // Leading Diagonals
    for col in 0..(grid.len() - target.len() + 1) {
        for row in 0..(grid.len() - target.len() + 1) {
            let window: Vec<_> = (0..target.len()).map(|i| grid[row + i][col + i]).collect();
            if window == target || window == reverse {  count += 1; }
        }
    }

    // Opposite Diagonals
    for col in (target.len() - 1)..grid.len() {
        for row in 0..(grid.len() - target.len() + 1) {
            let window: Vec<_> = (0..target.len()).map(|i| grid[row + i][col - i]).collect();
            if window == target || window == reverse {  count += 1; }
        }
    }

    count
}


fn p2(grid: &[Vec<char>], s: &str) -> usize {
    let target: Vec<_> = s.chars().collect();
    let reverse: Vec<_> = s.chars().rev().collect();

    let mut count = 0;
    for row in 0..(grid.len() - target.len() + 1) {
        for col in 0..(grid[row].len() - target.len() + 1) {
            let leading: Vec<_> = (0..target.len()).map(|i| grid[row + i][col + i]).collect();
            if leading != target && leading != reverse { continue; }
            
            let counter: Vec<_> = (0..target.len()).map(|i| grid[row + i][col + (target.len() - 1 - i)]).collect();
            if counter == target || counter == reverse { count += 1; }
        }
    }

    count
}