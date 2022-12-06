fn main() {
    let input = include_str!("../../../input/06");
    println!("Part 1: {}", marker(input, 4));
    println!("Part 2: {}", marker(input, 14));
}

fn marker(s: &str, window_size: usize) -> usize {
    let chars = s.char_indices().collect::<Vec<_>>();
    chars.windows(window_size)
        .find(|&xs| 
            xs.iter().all(|x| {
                xs.len() - xs.iter().filter(|&y| y.1 == x.1).count() == xs.len() - 1
            })
        )
        .map_or_else(|| 0, |w| w[w.len() - 1].0 + 1)
}