use std::time::Instant;

fn main() {
    let t_0 = Instant::now();

    let input = include_str!("../../../input/01")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let t_1 = Instant::now();
    let p1 = input.iter()
        .map(|line| {
            let first = line.iter().find_map(|c| c.to_digit(10));
            let last = line.iter().rev().find_map(|c| c.to_digit(10));
            return first.unwrap() * 10 + last.unwrap(); 
        }).sum::<u32>();
    let p1_t = t_1.elapsed();
    println!("Part 1: {}", p1);

    let t_2 = Instant::now();
    // The strange indexing below is a replication of the 'windows' iterator's behaviour,
    // but including the edge-windows smaller than the window size.
    let p2 = input.iter()
        .map(|line| {
            let first = ((-4isize)..line.len() as isize).find_map(|i| {
                let st = (i).max(0) as usize;
                let en = (i+5).min(line.len() as isize) as usize;
                find_digit(&line[st..en], true)
            });

            let last = (((-4isize)..line.len() as isize).rev()).find_map(|i| {
                let st = (i).max(0) as usize;
                let en = (i+5).min(line.len() as isize) as usize;
                find_digit(&line[st..en], true)
            });

            return first.unwrap() * 10 + last.unwrap(); 
        }).sum::<u32>();
    let p2_t = t_2.elapsed();
    println!("Part 2: {}", p2);
    println!(
        "\n  Time Taken\nPart 1:\t{:4}us\nPart 2:\t{:4}us\nTotal:\t{:4}us", 
        p1_t.as_micros(), p2_t.as_micros(), t_0.elapsed().as_micros()
    );
}

fn is_number(s: &str, rev: bool) -> Option<u32> {
    let starts = [
        ("zero" , Some(0)),
        ("one"  , Some(1)),
        ("two"  , Some(2)),
        ("three", Some(3)),
        ("four" , Some(4)),
        ("five" , Some(5)),
        ("six"  , Some(6)),
        ("seven", Some(7)),
        ("eight", Some(8)),
        ("nine" , Some(9)),
    ];

    if rev {
        starts.iter().find_map(|&(start, digit)| if s.ends_with(start) { digit } else { None })
    } else {
        starts.iter().find_map(|&(start, digit)| if s.starts_with(start) { digit } else { None })
    }
    
}

fn find_digit(w: &[char], accept_strings: bool) -> Option<u32> {
    if w[0].is_numeric() { return Some(w[0].to_digit(10).unwrap()) }

    if accept_strings { 
        for i in 0..w.len() {
            if let Some(n) = is_number(&(w[0..=i]).iter().collect::<String>(), false) {
                return Some(n)
            }
        }
    }
    None
}