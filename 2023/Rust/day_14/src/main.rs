use std::time::Instant;

fn main() {
    let t = Instant::now();
    let mut dish = include_str!("../../../input/14")
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let e = t.elapsed();
    println!("Part 1: {} ({}us)", calc_load(&tilt_north(&dish)), e.as_micros());

    let t = Instant::now();
    // At some point, the repeated moves will result in a loop of positions.
    // So, keep track of the 'load', and when those values start to loop, we can
    // use this to calculate the state of the dish at the end of the 1bn repetitions.
    let mut buf = vec![0; 100];
    for n in 0..1_000_000_000 {
        tilt_cycle(&mut dish);

        let v = calc_load(&dish);
        buf.remove(0);
        buf.push(v);

        if let Some(cycle) = find_cycle(&buf) {
            if cycle.len() < 2 { continue }
            let i = (999_999_999 - n) % cycle.len() - 1;

            let e = t.elapsed();
            println!("Part 2: {} ({}us)", cycle[i], e.as_micros());
            break; 
        }
    }
}

/// Search for the shortest cycle in a vector, starting from the end and looking back.
fn find_cycle(xs: &[usize]) -> Option<Vec<usize>> {
    let tgt = xs[xs.len() - 1];
    // rev -> find shortest cycle, no rev -> find longest cycle
    for start in (0..xs.len() - 1).rev() {
        if xs[start] != tgt { continue }
        let len = xs.len() - (start + 1);

        if len > (xs.len() / 2)  { continue; }
        let mut cyclic = true;
        for left in ((start - (len - 1))..start).rev() {
            if xs[left] != xs[left + len] { cyclic = false; break }
        }
        if cyclic {
            return Some(xs[(start + 1)..(start + 1 + len)].to_vec())
        }
    }
    None
}

fn tilt_cycle(dish: &mut Vec<Vec<char>>) {
    for f in [tilt_north, tilt_west, tilt_south, tilt_east] { *dish = f(&dish); }
}

fn tilt_north(dish: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut output = vec![vec!['.'; dish[0].len()]; dish.len()];
    for col in 0..dish[0].len() {
        let mut leftmost = 0;
        let mut n_boulder = 0;
        for row in 0..dish.len() {
            if dish[row][col] == 'O' {
                let dest = (leftmost + n_boulder).max(0) as usize;
                output[dest][col] = 'O';
                n_boulder += 1;
            } else if dish[row][col] == '#' {
                output[row][col] = '#';
                leftmost = (row + 1) as i32;
                n_boulder = 0;
            }
        }
    }
    output
}

fn tilt_south(dish: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut output = vec![vec!['.'; dish[0].len()]; dish.len()];
    for col in 0..dish[0].len() {
        let mut rightmost = dish.len() as i32 - 1;
        let mut n_boulder = 0;
        for row in (0..dish.len()).rev() {
            if dish[row][col] == 'O' {
                let dest = (rightmost - n_boulder).max(0) as usize;
                output[dest][col] = 'O';
                n_boulder += 1;
            } else if dish[row][col] == '#' {
                output[row][col] = '#';
                rightmost = row as i32 - 1;
                n_boulder = 0;
            }
        }
    }
    output
}

fn tilt_west(dish: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut output = vec![vec!['.'; dish[0].len()]; dish.len()];
    for row in 0..dish.len() {
        let mut leftmost = 0;
        let mut n_boulder = 0;
        for col in 0..dish[0].len() {
            if dish[row][col] == 'O' {
                let dest = (leftmost + n_boulder).max(0) as usize;
                output[row][dest] = 'O';
                n_boulder += 1;
            } else if dish[row][col] == '#' {
                output[row][col] = '#';
                leftmost = (col + 1) as i32;
                n_boulder = 0;
            }
        }
    }
    output
}

fn tilt_east(dish: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut output = vec![vec!['.'; dish[0].len()]; dish.len()];
    for row in 0..dish.len() {
        let mut rightmost = dish[0].len() as i32 - 1;
        let mut n_boulder = 0;
        for col in (0..dish[0].len()).rev() {
            if dish[row][col] == 'O' {
                let dest = (rightmost - n_boulder).max(0) as usize;
                output[row][dest] = 'O';
                n_boulder += 1;
            } else if dish[row][col] == '#' {
                output[row][col] = '#';
                rightmost = (col) as i32 - 1;
                n_boulder = 0;
            }
        }
    }
    output
}

fn calc_load(dish: &[Vec<char>]) -> usize {
    let mut load = 0;
    for col in 0..dish[0].len() {
        for row in 0..dish.len() {
            if dish[row][col] == 'O' {
                load += dish.len() - row;
            }
        }
    }
    load
}

fn draw(dish: &[Vec<char>]) {
    for row in dish {
        for cell in row { print!("{cell}"); }
        println!();
    }
}