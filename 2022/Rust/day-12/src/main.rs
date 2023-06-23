use std::collections::VecDeque;

fn main() {
    let input = include_str!("../../../input/12");
    let p1 = p1(input);
    let p2 = p2(input);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn to_grid(txt: &str) -> Vec<Vec<u8>> {
    txt.lines().map(|line| line.bytes().collect()).collect()
}

fn get_start_end(grid: &[Vec<u8>]) -> ((usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut s_found = false;
    let mut e_found = false;

    for (r, row) in grid.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if *cell == b'E' {
                end = (r, c);
                e_found = true;
            }
            if *cell == b'S' {
                start = (r, c);
                s_found = true;
            }
            if e_found && s_found { break }
        }
    }

    (start, end)
}

fn p1(input: &str) -> usize {
    let grid = to_grid(input);
    let (s, _) = get_start_end(&grid);

    let mut paths = VecDeque::new();
    paths.push_back((1, vec![s]));
    let path = search(&grid,  b'E', paths).unwrap();
    path.len() + 1
}


fn p2(input: &str) -> usize {
    let grid = to_grid(input);
    let (_, e) = get_start_end(&grid);

    let mut paths = VecDeque::new();
    paths.push_back((1, vec![e]));
    let path = search(&grid,  b'a', paths).unwrap();
    path.len() + 1
}

fn search(grid: &[Vec<u8>], dest: u8, mut paths: VecDeque<(usize, Vec<(usize, usize)>)>) -> Option<Vec<(usize, usize)>> {
    while !paths.is_empty() {
        let start = paths.pop_front().unwrap().1;
        let s = start.last().unwrap();
        if grid[s.0][s.1] == dest { return Some(start) }
    
        let mut options = vec![];
        if s.0 > 0 { options.push((s.0 - 1, s.1)) }
        if s.0 < grid.len() - 1 { options.push((s.0 + 1, s.1)) } 
        if s.1 > 0 { options.push((s.0, s.1 - 1)) }
        if s.1 < grid[0].len() - 1 { options.push((s.0, s.1 + 1)) }

        for option in options {
            let condition =  match dest == b'a' {
                false => { grid[option.0][option.1] - 1 <= grid[s.0][s.1] || (grid[s.0][s.1]) == b'S' },
                true =>  { grid[option.0][option.1] + 1 >= grid[s.0][s.1] || (grid[s.0][s.1]) == b'E' }
            };

            if condition {
                if !start.contains(&option) {
                    let mut new = start.to_owned();
                    new.push(option);
                    let heuristic = start.len();

                    let i = match paths.binary_search_by(|a| (a.0).cmp(&heuristic)) {
                        Ok(x) => x,
                        Err(x) => x
                    };

                    let visited = paths.iter().map(|(_, p)| p).any(|p| p.contains(&option));

                    if !visited {
                        paths.insert(i, (heuristic, new));
                    }
                }
            }
        }
    }
    None
}