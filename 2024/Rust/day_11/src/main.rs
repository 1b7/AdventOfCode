use std::collections::HashMap;

fn main() {
    let stones = include_str!("../../../input/11")
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1: {}", blinks(&stones, 25));
    println!("Part 2: {}",blinks(&stones, 75));

}

fn blinks(stones: &[usize], n: usize) -> usize {
    let mut hm = HashMap::new();
    stones.iter().map(|&s| expand(s, 0, n, &mut hm)).sum()
}

fn split_stone(stone: usize) -> Vec<usize> {
    if stone == 0 { return vec![1] }
    let digits = stone.ilog10() + 1;
    if digits % 2 == 0 {
        let x = 10_usize.pow(digits / 2);
        vec![stone / x, stone % x]
    } else {
        vec![stone * 2024]
    }
}

fn expand(s: usize, depth: usize, limit: usize, lut: &mut HashMap<(usize, usize), usize>) -> usize {
    if lut.contains_key(&(depth, s)) { return *lut.get(&(depth, s)).unwrap(); }
    if depth >= limit { return 1; }

    let expanded = split_stone(s).iter()
        .map(|&t| expand(t, depth + 1, limit, lut))
        .sum();

    lut.insert((depth, s), expanded);
    expanded
}