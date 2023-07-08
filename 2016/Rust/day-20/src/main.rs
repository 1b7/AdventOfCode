fn main() {
    let mut ranges: Vec<(u32, u32)> = load_input().lines()
        .map(to_range)
        .collect();
    println!("Part 1: {}", lowest_valid(&ranges));
    
    ranges.sort();
    println!("Part 2: {}", count_allowed(&ranges));
}

fn lowest_valid(ranges: &[(u32, u32)]) -> u32 {
    let mut min = u32::MAX;
    for i in 0..ranges.len() {
        let n = ranges[i].1.saturating_add(1);
        if n < min && !&ranges.iter().any(|&(l, u)| n >= l && n <= u ) {
            min = n;
        }
    }
    min
}

fn count_allowed(ranges: &[(u32, u32)]) -> u32 {
    let mut combined = vec![];
    for range in ranges { combine(*range, &mut combined); }
    dbg!(combined.len());

    let mut sum = 0;
    for window in combined.windows(2) {
        if window[1].0 > window[0].1 { sum += window[1].0 - window[0].1 - 1; }
    }

    sum + (u32::MAX - combined[combined.len() - 1].1)
}

fn combine(range:(u32, u32), ranges: &mut Vec<(u32, u32)>) {
    let (l, u) = range;
    let mut is_new = true;
    for i in 0..ranges.len() {
        // Discard ranges which are already covered
        if l >= ranges[i].0 && u <= ranges[i].1 { return; }
        // Extend ranges which overlap the lower-bound of a range.
        if l <= ranges[i].0 && u >= ranges[i].0 {
            is_new = false;
            ranges[i].0 = l; 
        }
        // Extend ranges which overlap the upper-bound of a range.
        if l >= ranges[i].0 && l <= ranges[i].1 && u >= ranges[i].1 {
            is_new = false;
            ranges[i].1 = u; 
        }
    }
    if is_new { ranges.push((l, u)); }
}

fn to_range(line: &str) -> (u32, u32) {
    let (left, right) = line.split_once('-').unwrap();
    (left.parse().unwrap(), right.parse().unwrap())
}

fn load_input() -> String {
    std::fs::read_to_string(
        std::env::args().nth(1).expect("Filepath not provided.")
    ).expect("Could not load file!")
}