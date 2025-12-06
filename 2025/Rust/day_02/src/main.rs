fn main() {
    let input = include_str!("../../../../input/2025/02")
        .split(",")
        .map(|line| line.split_once('-').expect("Invalid range passed in"))
        .map(|range_str| (
            range_str.0.parse::<u64>().expect("Invalid number in range"),
            range_str.1.parse::<u64>().expect("Invalid number in range")
        ))
        .collect::<Vec<_>>();


    let p1 = input.iter().map(|&range| range).map(process_range_p1).sum::<u64>();
    let p2 = input.into_iter().map(process_range_p2).sum::<u64>();

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn process_range_p1(range: (u64, u64)) -> u64 {
    (range.0..=range.1).filter(|n| {
        let n_str = n.to_string();
        if n_str.len() % 2 != 0 { return false; }
        let (left, right) = n_str.split_at(n_str.len() / 2);

        left == right
    }).sum()
}

fn process_range_p2(range: (u64, u64)) -> u64 {
    (range.0..=range.1).filter(|n| {
        let n_str = n.to_string();

        for len in 1..n_str.len() {
            if n_str.len() % len != 0 { continue; }
            let segments = n_str.len() / len;

            let (prefix, _) = n_str.split_at(len);

            if prefix.repeat(segments) == n_str { return true }
        }

        false
    }).sum()
}
