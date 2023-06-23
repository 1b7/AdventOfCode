use std::collections::HashSet;

const ROW_WIDTH: usize = 7;
// Arbitrarily defined constant to circumvent weird issues with repetitions in data:
const DUP_CHECK: usize = 25; 

fn main() {
    let input = include_str!("../../../input/17");
    let s1 = p1(input, 2022);
    let s2 = p1(input, 1000000000000);
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);

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
    let mut ceil_off = 0;
    let mut settled = HashSet::new();
    let mut dir_cursor = 0;
    let mut states: Vec<(usize, usize, (usize, usize))> = vec![];

    let mut done = false;
    let mut i = 0;
    while i < total {
        let spawn_point = (2, ceiling + 3);
        let mut new_rock = blocks[i % blocks.len()].clone();
        for (x, y) in new_rock.iter_mut() {
            *x += spawn_point.0;
            *y += spawn_point.1;
        }

        // Simulate falling + wind effects on a per-rock basis:
        loop {
            let direction = directions[dir_cursor % directions.len()];
            shift(direction, &mut new_rock, &settled);
            dir_cursor += 1;
            let fell = fall(&mut new_rock, &settled, 0);

            if !fell {
                settled.extend(new_rock.iter());
                let &highest = new_rock.iter().map(|(_, y)| y).max().unwrap();
                ceiling = ceiling.max(highest + 1);

                let new_state = (i % blocks.len(), dir_cursor % directions.len());
                let counts = states.iter().filter(|(_, _, pos)| new_state == *pos);

                let mut len = 0;
                let mut last = None;
                for x in counts {
                    last = Some(x);
                    len += 1;
                }

                if last.is_some() && len > DUP_CHECK && !done {
                    done = true;
                    let &(cyc_ceil, cyc_idx, _) = last.unwrap();
                    let cycle_height = ceiling - cyc_ceil;
                    let rem_cycles = (total - i) / (i - cyc_idx);

                    i += (i - cyc_idx) * rem_cycles;
                    ceil_off = cycle_height * rem_cycles;
                }
                states.push((ceiling, i, new_state));
                break;
            }
        }
        i += 1;
    }
    ceiling + ceil_off
}