use std::collections::HashSet;

const ROW_WIDTH: usize = 7;

fn main() {
    let input = include_str!("../../../input/17");
    let p1 = p1(input, 2022);
    // let p1 = p1(input, 1000000000000);
    println!("Part 1: {}", p1);

}

fn shift(c: char, block: &mut Vec<(usize, usize)>, grid: &HashSet<(usize, usize)>) -> bool {
    fn adjust(x: usize, c: char) -> usize {
        match c {
            '>' => x + 1,
            '<' => x - 1,
            _ => panic!("Unrecognised char '{}'", c)
        }
    }

    for &(x, y) in block.iter() {
        if (x == 0 && c == '<') || (x == (ROW_WIDTH - 1) && c == '>') || grid.contains(&(adjust(x, c), y)) {
            return false;
        }
    }

    for (x, _) in block.iter_mut() { *x = adjust(*x, c); }
    true
}

fn fall(block: &mut Vec<(usize, usize)>, grid: &HashSet<(usize, usize)>, floor: usize) -> bool {
    if block.iter().any(|&(x, y)| y <= floor || grid.contains(&(x, y - 1))) {
        return false;
    }

    for (_, y) in block.iter_mut() {  *y -= 1; }
    true
}

fn draw_grid(settled: &HashSet<(usize, usize)>) {
    let mins = (0, 0);
    let mut maxes = (0, 0);
    for &(x, y) in settled {
        maxes.0 = maxes.0.max(x);
        maxes.1 = maxes.1.max(y);
    }

    for y in (mins.1..=maxes.1).rev() {
        for x in mins.0..=maxes.0 {
            if settled.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn p1(input: &str, total: usize) -> usize {
    let blocks = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (0, 1), (1, 0), (1, 1)]
    ];

    let directions: Vec<char> = input.trim().chars().collect();
    let mut ceiling = 0;
    let mut floor = 0;
    let mut settled = HashSet::new();
    let mut i = 0;

    let mut dir_cursor = 0;

    while i < total {
        let spawn_point = (2, ceiling + 3);
        let mut new_rock = blocks[i % blocks.len()].clone();
        for (x, y) in new_rock.iter_mut() {
            *x += spawn_point.0;
            *y += spawn_point.1;
        }

        loop {
            let direction = directions[dir_cursor % directions.len()];

            shift(direction, &mut new_rock, &settled);
            dir_cursor += 1;

            let fell = fall(&mut new_rock, &settled, floor);
            if !fell {
                settled.extend(new_rock.iter());
                let &highest = new_rock.iter().map(|(_, y)| y).max().unwrap();
                ceiling = ceiling.max(highest + 1);
                break;
            }
        }

        // Trying to reduce space by deleting unreachable elements, i.e. where a
        // new floor has been created.
        let affected_rows: HashSet<_> = new_rock.iter().map(|(_, y)| *y).collect(); 
        let mut new_floors = [None; 5];

        for (i, &row) in affected_rows.iter().enumerate() {
            for col in 0..ROW_WIDTH {
                if !settled.contains(&(col, row)) {
                    new_floors[i] = None;
                    break;
                }
                new_floors[i] = Some(row);
            }
        }
        
        for nf in new_floors {
            if let Some(nf) = nf {
                floor = floor.max(nf);
                settled.retain(|&(_, y)| y >= nf);
            }
        }

        i += 1;
    }
    ceiling
}