fn main() {
    println!("Part 1: {}", run(40));
    println!("Part 2: {}", run(400000));
}

fn run(cycles: usize) -> usize {
    let mut prev: Vec<char> = load_input().chars().collect();
    let mut safe = prev.iter().filter(|&&c| c == '.').count();

    for _ in 1..cycles {
        // Insert safe buffer tiles.
        prev.insert(0, '.');
        prev.push('.');

        let mut next = vec!['^'; prev.len() - 2];
        for n in 1..(prev.len() - 1) {
            if !((prev[n - 1] == '^') ^ (prev[n + 1] == '^')) { // Left XNOR Right
                next[n - 1] = '.';
                safe += 1;
            }
        }
        prev = next;
    }
    safe
}

fn load_input() -> String {
    std::fs::read_to_string(
        std::env::args().nth(1).expect("Filepath not provided.")
    ).expect("Could not load file!")
}