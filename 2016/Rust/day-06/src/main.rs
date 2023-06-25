fn main() {
    let signals = load_input();
    let mut counts = [[0; 26]; 8];
    for signal in signals.lines() {
        for (i, c) in signal.char_indices() {
            counts[i][(c as u8 - b'a') as usize] += 1;
        }
    }

    let most_frequent = counts.map(|count| {
        count.iter().enumerate()
            .map(|(i, x)| (x, i))
            .max().unwrap().1
    }).iter().map(|&idx| (idx as u8 + b'a') as char)
        .collect::<String>();

    let least_frequent = counts.map(|count| {
        count.iter().enumerate()
            .map(|(i, x)| (x, i))
            .min().unwrap().1
    }).iter().map(|&idx| (idx as u8 + b'a') as char)
        .collect::<String>();

    println!("Part 1: {}\nPart 2: {}", most_frequent, least_frequent);
    
}

fn load_input() -> String {
    std::fs::read_to_string(
        std::env::args().nth(1).expect("Filepath not provided.")
    ).expect("Could not load file!")
}