use std::time::Instant;

fn main() {
    let s = Instant::now();
    
    let input = include_str!("../../../input/11");
    let map: Vec<Vec<bool>> = input.lines()
        .map(|line| line.chars().map(|c| c == '#' ).collect())
        .collect();

    // Locate expansion points:
    let mut expanded_rows = vec![];
    for r in (0..map.len()).rev() {
        if !map[r].iter().any(|&x| x) { 
            expanded_rows.push(r);
        }
    }
    let mut expanded_cols = vec![];
    for c in (0..map[0].len()).rev() {
        if !(0..map.len()).any(|r| map[r][c]) { 
            expanded_cols.push(c);
        }
    }

    // Calculate coordinates
    let mut galaxies = vec![];
    for (i, row) in map.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col { galaxies.push((i, j)) }
        }
    }
    
    let e = s.elapsed();
    println!("Part 1: {}", calc_dists(&galaxies, &expanded_rows, &expanded_cols, 1));
    println!("Part 2: {}", calc_dists(&galaxies, &expanded_rows, &expanded_cols, 999_999));
    println!("(Both parts: {}us)", e.as_micros());
}

fn calc_dists(galaxies: &[(usize, usize)], expanded_rows: &[usize], expanded_cols: &[usize], scaling: usize) -> usize {
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let (g_i, g_j) = (galaxies[i], galaxies[j]);
            
            let rows_crossed = expanded_rows.iter()
                .filter(|&&r| (r > g_i.0 && r < g_j.0) || (r > g_j.0 && r < g_i.0))
                .count();
            let cols_crossed = expanded_cols.iter()
                .filter(|&&c| (c > g_i.1 && c < g_j.1) || (c > g_j.1 && c < g_i.1))
                .count();
            let expansion = (rows_crossed + cols_crossed) * scaling;

            sum += expansion + g_i.0.abs_diff(g_j.0) + g_i.1.abs_diff(g_j.1);
        }
    }
    sum
}