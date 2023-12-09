use std::time::Instant;

fn main() {
    let p1_start = Instant::now();
    let input = include_str!("../../../input/09").lines().map(|line|
        line.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>()
    ).collect::<Vec<Vec<_>>>();

    let mut p1 = 0;
    for seq in input.clone() {
        let mut subseqs = subsequences(seq, false);

        let max_idx = subseqs.len() - 1;
        subseqs[max_idx].push(0);

        for i in (0..max_idx).rev() {
            let new_n = subseqs[i + 1][subseqs[i + 1].len() - 1] + subseqs[i][subseqs[i].len() - 1];
            subseqs[i].push(new_n);
        }
        p1 += subseqs[0][subseqs[0].len() - 1];
    }

    let p1_time = p1_start.elapsed().as_micros();
    println!("Part 1: {p1:10} ({p1_time})us");

    let p2_start = Instant::now();
    let mut p2 = 0;
    for seq in input {
        let mut subseqs = subsequences(seq, true);

        let max_idx = subseqs.len() - 1;
        subseqs[max_idx].insert(0, 0);

        for i in (0..max_idx).rev() {
            let new_n = subseqs[i + 1][0] + subseqs[i][0];
            subseqs[i].insert(0, new_n);
        }
        p2 += subseqs[0][0];
    }
    let p2_time = p2_start.elapsed().as_micros();
    println!("Part 2: {p2:10} ({p2_time})us");
}

fn subsequences(seq: Vec<i32>, backward: bool) -> Vec<Vec<i32>> {
    let window_fn = if backward { |w: &[i32]| w[0] - w[1] } else { |w: &[i32]| w[1] - w[0] };
    let mut subseqs = vec![seq];
    for i in 0.. {
        let new_seq = subseqs[i].windows(2).map(window_fn).collect::<Vec<_>>();
        let all_zero = new_seq.iter().all(|&x| x == 0);
        subseqs.push(new_seq);
        if all_zero { break }
    }
    subseqs
}