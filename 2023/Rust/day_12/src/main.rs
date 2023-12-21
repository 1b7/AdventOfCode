use std::{time::Instant, fmt::Display, collections::HashMap};

use quick_cache::unsync::Cache;

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

    // rayon::ThreadPoolBuilder::new().num_threads(4).build_global().unwrap();

    let input = unfold(input.clone());

    let sum = input.iter().enumerate().map(|(i, (line, seqs))| { 
        let result = placements(line, seqs);
        println!("{i:4}: {result}");
        result
    }).sum::<usize>();
    
    println!("{sum}");
}

fn to_str(line: &[Option<Spring>]) -> String {
    line.iter().map(|&x| match x {
        None => '?',
        Some(Spring::Damaged) => '#',
        Some(Spring::Operational) => '.'
    }).collect()
}

fn placements(line: &[Option<Spring>], seqs: &[u8]) -> usize {
    // Try to identify all possible starting points for the first sequence,
    // then all starting points from *those* starting points,
    // and so on.

    // Utility function to find the blocks (as pairs of indices) of fixed damaged springs.
    let find_damaged = |from: usize| -> Vec<(usize, usize)> {
        let mut start = usize::MAX;
        let mut end = usize::MAX;
        let mut pairs = vec![];
        for i in from..line.len() {
            if line[i] == Some(Spring::Damaged) && start > line.len() {
                start = i;
                end = i;
            } else if line[i] != Some(Spring::Damaged) && start < line.len() {
                end = i - 1;
                pairs.push((start, end));
                start = usize::MAX;
                end = usize::MAX;
            }
        }
        if pairs.len() == 0 || start < usize::MAX  { pairs.push((start, end)) }
        pairs
    };

    // Initialise the starting points:
    let s = seqs[0] as usize;
    let mut starts = vec![];
    let blocks = find_damaged(0);
    let (stop_l, _) = blocks[0];
    for (i, w) in line.windows(s).enumerate() {
        if i > stop_l { break }
        let w_end = i + s - 1;
        if w_end + 1 < line.len() &&  line[w_end + 1] == Some(Spring::Damaged) { continue }
        if w.iter().all(|&c| c == None || c == Some(Spring::Damaged)) {
            starts.push(1u128 << i);
        }
    }

    // Convert a u128 back to a set of indices.
    const N_BITS: usize = 128;
    let bits_to_vec = |n: &u128| -> Vec<usize> {
        (0..N_BITS).filter(|i| (n & (1 << i)) > 0).collect()
    };

    let pprint = |v: &[u128]| {
        format!("{:?}", v.clone().iter().map(bits_to_vec).collect::<Vec<_>>())
    };

    // let mut cache: HashMap<(usize, usize), Vec<u128>> = HashMap::new();
    let mut cache: Cache<(usize, usize), Vec<u128>> = Cache::new(100_000);

    // Now iteratively search for valid placements of blocks.
    let empty = vec![];
    for seq_idx in 1..seqs.len() {
        let mut saved = vec![];
        let s = seqs[seq_idx] as usize;

        for &placement_set in &starts {
            let placement_vec = bits_to_vec(&placement_set);
            let st = placement_vec[placement_vec.len() - 1] + seqs[seq_idx - 1] as usize;

            let mut this_placement: Vec<u128> = vec![];
            if let Some(result) = cache.get(&(st, s)) {
                this_placement = result.to_owned();
            } else {
                // Find the first relevant block to the current window.
                let fst = (0..blocks.len()).find(|&i| blocks[i].0 >= st);
                let blocks = if let Some(f) = fst { &blocks[f..] } else { &empty };
                
                'window:
                for (i, w) in line.windows(s).enumerate() {
                    if i <= st { continue }
                    if blocks.len() > 0 && i > blocks[0].0 { break }
                    let w_end = i + s - 1;
                    if w_end + 1 < line.len() && line[w_end + 1] == Some(Spring::Damaged) { continue }
    
                    // Is there an overlap with a block that isn't fully consumed?
                    // Is there an overlap:
                    //   Start OR End is within the block.
                    // Is it fully consumed:
                    //   Starts before and Ends at or After,
                    for &(l, r) in blocks {
                        let overlap = (i >= l && i <= r) || (w_end >= l && w_end <= r);
                        let consumed = i <= l && w_end >= r;
                        if overlap && !consumed { continue 'window }
                    }
    
                    if w.iter().all(|&c| c == None || c == Some(Spring::Damaged)) {
                        this_placement.push(placement_set | (1 << i));
                    }
                }
                cache.insert((st, s), this_placement.clone());
            }
            saved.extend(this_placement);
        }
        dbg!(saved.len());
        starts = saved;
    }

    // Check that there aren't any blocks of damaged springs past the end of our placements:
    let last_block = blocks[blocks.len() - 1];
    let mut count = 0;
    for placement in &starts {
        let end_idx = ((N_BITS - 1) - placement.leading_zeros() as usize) + seqs[seqs.len() - 1] as usize - 1;
        if last_block.1 < usize::MAX && end_idx < last_block.1 { continue }
        count += 1;
    }

    count
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

fn parse_line(s: &str) -> (Vec<Option<Spring>>, Vec<u8>) {
    let (springs, seqs) = s.split_once(' ').unwrap();
    let springs = springs.chars()
        .map(|c| match c {
            '#' => Some(Spring::Damaged),
            '.' => Some(Spring::Operational),
            _  => None
        }).collect();

    let seqs = seqs.split(',').map(|n| n.parse().unwrap()).collect();
    (springs, seqs)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spring { Operational, Damaged }