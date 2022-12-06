use std::time::Instant;

fn main() {
    let input = include_str!("../../../input/06");
    println!("Part 1: {}", marker(input, 4));
    println!("Part 2: {}", marker(input, 14));
}

fn marker(s: &str, window_size: usize) -> usize {
    let chars = s.char_indices().collect::<Vec<_>>();
    chars.windows(window_size)
        .find(|&xs| {
            let mut i = 0;
            xs.iter().all(|(_, x)| {
                i += 1;
                xs.iter().skip(i).filter(|&(_, y)| y == x).count() == 0
            })
        })
        .map_or_else(|| 0, |w| w[w.len() - 1].0 + 1)
}