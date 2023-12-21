use std::collections::{VecDeque, HashSet};

fn main() {
    let input = include_str!("../../../input/21");

    let mut sp = (0, 0);

    let grid = input
        .lines()
        .enumerate()
        .map(|(r, line)| line.chars().enumerate().map(|(c, chr)| {
            if chr == 'S' { sp = (r as isize, c as isize); }
            chr == '.' || chr == 'S'
        }).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    dbg!(p1(sp, &grid));
}


fn p1(start: (isize, isize), grid: &Vec<Vec<bool>>) -> usize {
    let (rows, cols) = (grid.len() as isize, grid[0].len() as isize);
    let start = (start.0, start.1);
    let moves = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut starts = HashSet::new();
    starts.insert(start);

    for _ in 0..64 {
        let mut new_starts = HashSet::new();
        
        for &start in &starts {
            for m in moves {
                let npos = (start.0 + m.0, start.1 + m.1);
                if npos == start { continue }
                if npos.0 < 0 || npos.1 < 0 || npos.0 >= rows || npos.1 >= cols { continue }
                if grid[npos.0.rem_euclid(rows) as usize][npos.1.rem_euclid(cols) as usize] {
                    new_starts.insert(npos);
                }
            }
        }
        starts = new_starts;
    }
    dbg!(starts.len())
}