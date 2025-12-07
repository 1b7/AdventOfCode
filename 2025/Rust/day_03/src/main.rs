use std::collections::HashMap;
use rayon::prelude::*;

fn main() {
    let banks = include_str!("../../../../input/2025/03");

    let process_input = || banks.lines()
        .par_bridge()
        .map(parse_bank);

    let with_exponent = |exp| process_input()
        .map(|bank | {
            calc_max_joltage(&bank, exp, &mut HashMap::new())
        })
        .sum::<usize>();

    println!("Part One: {}", with_exponent(1));
    println!("Part Two: {}", with_exponent(11));
}

fn parse_bank(bank: &str) -> Vec<usize> {
    bank.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

fn calc_max_joltage(bank: &[usize], exp: u32, memo_cache: &mut HashMap<(Vec<usize>, u32), usize>) -> usize {
    if memo_cache.contains_key(&(bank.to_vec(), exp)) { return memo_cache[&(bank.to_vec(), exp)]; }

    let result = (0..(bank.len() - exp as usize))
        .map(|i| {
            bank[i] *  10_usize.pow(exp) + if exp == 0 { 0 } else { calc_max_joltage(&bank[(i + 1)..], exp - 1, memo_cache) }
        })
        .max()
        .unwrap();

    memo_cache.insert((bank.to_vec(), exp), result);
    result
}