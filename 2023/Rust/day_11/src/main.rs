fn main() {
    let input = include_str!("../../../input/11");
    let mut map: Vec<Vec<bool>> = input.lines()
        .map(|line| line.chars().map(|c| c == '#' ).collect())
        .collect();

    // Apply Expansions
    let mut expanded_rows = vec![];
    for r in (0..map.len()).rev() {
        if !map[r].iter().any(|&x| x) { 
            expanded_rows.push(r);
        }
    }
    dbg!(&expanded_rows);

    let mut expanded_cols = vec![];
    for c in (0..map[0].len()).rev() {
        if !(0..map.len()).any(|r| map[r][c]) { 
            expanded_cols.push(c);
        }
    }
    dbg!(&expanded_cols);

    // Calculate coordinates
    let mut galaxies = vec![];
    for (i, row) in map.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col { galaxies.push((i, j)) }
        }
    }

    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let (g_i, g_j) = (galaxies[i], galaxies[j]);
            
            let rows_crossed = expanded_rows.iter().filter(|&&r| r > g_i.0 && r < g_j.0).collect::<Vec<_>>();
            let cols_crossed = expanded_cols.iter().filter(|&&c| c > g_i.1 && c < g_j.1).collect::<Vec<_>>();
            let bounds = (rows_crossed.len() + cols_crossed.len());

            let dif = g_i.0.abs_diff(g_j.0) + g_i.1.abs_diff(g_j.1);
            let dist = dif + bounds;
            
            println!("[{} {} : {:2}]\t{:?}\t{} {}\t({:?} | {:?})", i+1, j+1, dist, (galaxies[i], galaxies[j]), rows_crossed.len(), cols_crossed.len(), rows_crossed, cols_crossed);

            sum += dist;
        }
    }
    println!("{}", sum);

}
