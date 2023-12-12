use std::{time::Instant, fmt::Display};

use rayon::prelude::*;

/*
    NOTE: This is a solution in principle; it works for Part 1,
    and would work for Part 2 given enough time and memory.
    At some point, I intend to rewrite this!
*/

fn main() {
    let input = include_str!("../../../input/12")
        .lines()
        .map(parse_line)
        .collect::<Vec<_>>();

    // let input = unfold(input.clone());

    // Naive approach, try every possible combination and check for validity.
    let t_s = Instant::now();
    let valid_arranges: usize = input.par_iter()
        .enumerate()
        .map(|(i, (springs, seq))| {
            // let r = permutations(&springs, &seq);
            // println!("Processed {}: {} ({}s)", i, r, t_s.elapsed().as_secs());
            permutations(&springs, &seq)
        })
        .sum();

    println!("{} {}", valid_arranges, t_s.elapsed().as_secs());
}

fn unfold(input: Vec<(Vec<Option<Spring>>, Vec<u8>)>) -> Vec<(Vec<Option<Spring>>, Vec<u8>)> {
    input.into_iter().map(|(springs, data)| {
        let mut new_spring = springs.clone();
        new_spring.push(None);
        let mut new_spring = new_spring.into_iter().cycle().take(4 * springs.len() + 4).collect::<Vec<_>>();
        new_spring.extend(springs.clone());

        let new_data = data.clone().into_iter().cycle().take(5 * data.len()).collect::<Vec<_>>();

        (new_spring, new_data)
    }).collect()
}

fn valid_line(b: &BinaryStack, expect: &[u8], strict: bool) -> bool {
    let mut bseq = b.as_seq();
    bseq.reverse();
    if strict { return bseq == expect }

    if bseq.len() > expect.len() { return false }
    for i in 0..(bseq.len() - 1) {
        if bseq[i] != expect[i] { return false }
    }
    bseq[bseq.len() - 1] <= expect[bseq.len() - 1]
}

fn permutations(line: &Vec<Option<Spring>>, seqs: &[u8]) -> usize {
    let mut buf: [Vec<BinaryStack>; 2] = [vec![BinaryStack::new(0)], vec![]];
    let mut toggle = 0;

    for spring in line {
        if spring.is_some() {
            if spring.unwrap() == Spring::Operational {
                for b in &mut buf[toggle] { b.push_zero(); }
            } else {
                for b in &mut buf[toggle] { b.push_one(); }
            }
            continue;
        }

        buf[(toggle + 1) % 2] = vec![];

        let ops = [BinaryStack::push_one, BinaryStack::push_zero];
        for stack in buf[toggle].clone() {
            for op in ops {
                let mut x = stack.clone();
                op(&mut x);
                if valid_line(&x, seqs, false) {
                    buf[(toggle + 1) % 2].push(x);
                }
            }
        }
        toggle = (toggle + 1) % 2;
    }

    buf[toggle].clone().into_iter().filter(|&l| {
        valid_line(&l, seqs, true)
    }).count()
}

fn parse_line(s: &str) -> (Vec<Option<Spring>>, Vec<u8>) {
    let (springs, seqs) = s.split_once(' ').unwrap();
    let springs = springs.chars().map(|c| match c {
        '#' => Some(Spring::Damaged),
        '.' => Some(Spring::Operational),
         _  => None
    }).collect();

    let seqs = seqs.split(',').map(|n| n.parse().unwrap()).collect();

    (springs, seqs)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spring { Operational, Damaged }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BinaryStack(u128);

impl BinaryStack {
    pub fn new(x: u128) -> Self { Self(x) }

    pub fn from_springs(springs: &[Spring]) -> Self {
        let mut out = 0;
        for s in springs.iter().rev() {
            match s {
                Spring::Damaged     => { out <<= 1; out |= 1;}
                Spring::Operational => { out <<= 1; }
            }
        }
        BinaryStack(out)
    }

    pub fn to_springs(&self, mut len: usize) -> Vec<Spring> {
        let mut out = vec![];
        let mut n = self.0;
        while len > 0 {
            let x = n & 1;
            n >>= 1;
            match x {
                1 => { out.push(Spring::Damaged) },
                0 => { out.push(Spring::Operational) },
                _ => unreachable!()
            };
            len -= 1;
        }
        out.reverse();
        out
    }

    pub fn as_seq(&self) -> Vec<u8> {
        let mut x = self.0;
        let mut out = vec![];
        let mut block = 0;
        while x > 0 {
            if x & 1 == 1 { block += 1 }
            else if block > 0  { out.push(block); block = 0 }
            x >>= 1;
        }
        if out.len() == 0 || block > 0 { out.push(block); }
        out
    }

    pub fn push_one(&mut self) { self.0 = (self.0 << 1) | 1; }
    pub fn push_zero(&mut self) { self.0 <<= 1; }

}

impl Display for BinaryStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0128b}", self.0)
    }
}