fn main() {
    let input = include_str!("../../../input/23");
    let (p1, p2) = compute(input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn as_grid(s: &str) -> Vec<(isize, isize)> {
    s.lines().enumerate().flat_map(|(i, line)| {
        line.char_indices().filter_map(|(j, c)| {
            if c == '#' { Some((i as isize, j as isize)) } else { None }
        }).collect::<Vec<_>>()
    }).collect()
}

fn propose_move(p: (isize, isize), grid: &[(isize, isize)], rotate: usize) -> (isize, isize) {
    let (x, y) = (p.1, p.0);
    let checks = [
        (y - 1, x - 1), (y - 1, x), (y - 1, x + 1),
        (y,     x - 1),             (y,     x + 1),
        (y + 1, x - 1), (y + 1, x), (y + 1, x + 1)
    ];

    let results = [
        (!grid.contains(&checks[0]) && !grid.contains(&checks[1]) && !grid.contains(&checks[2]), (y - 1, x)),
        (!grid.contains(&checks[5]) && !grid.contains(&checks[6]) && !grid.contains(&checks[7]), (y + 1, x)),
        (!grid.contains(&checks[0]) && !grid.contains(&checks[3]) && !grid.contains(&checks[5]), (y, x - 1)),
        (!grid.contains(&checks[2]) && !grid.contains(&checks[4]) && !grid.contains(&checks[7]), (y, x + 1))
    ];

    if !checks.iter().any(|pair| grid.contains(pair)) {
        return (y, x)
    } else {
        for n in rotate..(rotate + results.len()) {
            let (result, proposal) = results[n % results.len()];
            if result { return proposal }
        }
    }
    (y, x)
}

fn compute(input: &str) -> (usize, usize) {
    let mut current = as_grid(input);
    let mut proposed = vec![];
    let mut resets = vec![];
    let mut rounds = 0;
    let mut p1 = 0;

    loop {
        for &pair in &current {
            proposed.push(propose_move(pair, &current, rounds));
        }
        
        for (i, proposal) in proposed.iter().enumerate() {
            if proposed.iter().filter(|&&p| p == *proposal).count() > 1 {
                resets.push(i);
            }
        }
        for &r in &resets { proposed[r] = current[r]; }
        resets.clear();

        if rounds == 10 {
            let min_y = current.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
            let min_x = current.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
            let max_y = current.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
            let max_x = current.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
        
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    if !current.contains(&(y, x)) {
                        p1 += 1;
                    }
                }
            }
        }

        if proposed == current { break; }
        current.clear();
        current.append(&mut proposed);
        rounds += 1;
    }
    (p1, rounds + 1)
}