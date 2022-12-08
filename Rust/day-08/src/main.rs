use std::iter;

fn main() {
    let input = include_str!("../../../input/08");
    let rows = process_input(input);
    let height: usize = rows.len();
    let width: usize = rows[0].len();
    
    println!("Part 1: {:?}", p1(&rows, height, width));
    println!("Part 2: {:?}", p2(&rows, height, width));
}

fn process_input(input: &str) -> Vec<Vec<u8>> {
    let mut rows = vec![];
    for line in input.lines() {
        let row: Vec<u8> = line
            .trim()
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        rows.push(row);
    }
    rows
}

fn p1(rows: &[Vec<u8>], height: usize, width: usize) -> usize {
    let mut count  = 0;
    for x in 1..(height - 1) {
        for y in 1..(width - 1) {
            fn checker(iter: &mut impl Iterator<Item = (usize, usize)>, x: usize, y: usize, rows: &[Vec<u8>]) -> bool {
                iter.all(|(r, c)|
                    if r == x && c == y { true } else { rows[r][c] < rows[x][y] }
                )
            }
            if  checker( &mut (0..=x).zip(iter::repeat(y)),            x, y, &rows) || // North
                checker( &mut (x..=(height - 1)).zip(iter::repeat(y)), x, y, &rows) || // South
                checker( &mut iter::repeat(x).zip(0..=y),              x, y, &rows) || // West
                checker( &mut iter::repeat(x).zip(y..=(width - 1)),    x, y, &rows)    // East
            {
                count += 1
            }
        }
    }
    count + (2 * (height - 1)) + (2 *  (width - 1))
}

fn p2(rows: &[Vec<u8>], height: usize, width: usize) -> usize {
    let mut view  = 0;
    for x in 1..(height - 1) {
        for y in 1..(width - 1) {
            fn finder(iter: &mut impl Iterator<Item = (usize, usize)>, x: usize, y: usize, rows: &[Vec<u8>]) -> usize {
                iter.take_while(|&(r, c)|
                    if r == x && c == y { true } else { rows[r][c] < rows[x][y] }
                ).count()
            }
            view = view.max(
                finder( &mut (1..=x).rev().zip(iter::repeat(y)),      x, y, &rows) * 
                finder( &mut (x..=(height - 2)).zip(iter::repeat(y)), x, y, &rows) * 
                finder( &mut iter::repeat(x).zip((1..=y).rev()),      x, y, &rows) * 
                finder( &mut iter::repeat(x).zip(y..=(width - 2)),    x, y, &rows));
            }
    }
    view 
}