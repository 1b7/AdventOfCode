use std::{collections::HashSet, time::Instant};

fn main() {
    let t_start = Instant::now();
    let input: Vec<Vec<char>> = include_str!("../../../input/03").lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();

    let mut number_locs: HashSet<usize> = HashSet::new();
    let mut sum = 0;
    let mut ratio = 0;

    for (row, line) in input.iter().enumerate() {
        for (col, &chr) in line.iter().enumerate() {
            if chr != '.' && !chr.is_digit(10) {
                let is_gear = chr == '*';
                let mut found_nums = vec![];

                let neighbours = [(-1i32, -1i32), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
                for neighbour in neighbours {
                    let (offset_row, offset_col) = neighbour;
                    let (row, col) = (row as i32, col as i32);
                    let (r, c) = (
                        (row + offset_row).max(0).min((input.len() - 1 )as i32) as usize, 
                        (col + offset_col).max(0).min((input.len() - 1) as i32) as usize
                    );
                    
                    if input[r][c].is_digit(10) {
                        let mut n_start = c;
                        while n_start < line.len() - 1 && input[r][n_start + 1].is_digit(10) {
                            n_start += 1;
                        }
                        
                        if !number_locs.contains(&((r * line.len() + n_start))) {
                            number_locs.insert(r * line.len() + n_start);
                            let mut pow = 1;
                            let mut num = input[r][n_start].to_digit(10).unwrap();
                            while n_start > 0 && input[r][n_start - 1].is_digit(10) {
                                n_start -= 1;
                                num += input[r][n_start].to_digit(10).unwrap() * 10u32.pow(pow);
                                pow += 1;
                            }
                            found_nums.push(num);
                            sum += num;
                        }
                    }
                }
                if is_gear && found_nums.len() == 2 {
                    ratio += found_nums[0] * found_nums[1]
                }
            }
        }
    }
    let elapsed = t_start.elapsed().as_micros();
    println!("Part 1: {}\nPart 2: {}\nTook: {}us", sum, ratio, elapsed);
}
