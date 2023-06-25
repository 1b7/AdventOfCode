use md5;

fn main() {
    let prefix = load_input();
    let mut suffix = 0;
    let (mut i, mut j) = (0, 0);

    let mut p1_output = String::new();
    let mut p2_output = vec!['-'; 8];

    while i < 8 || j < 8 {
        let hash_str = format!("{}{}", prefix, suffix);
        let digest: String = md5::compute(hash_str).iter()
            .map(|&c| format!("{:02x}", c)).collect();

        if digest.starts_with("00000") {
            let c6 = digest.chars().nth(5).unwrap();
            if i < 8 {
                p1_output.push(c6);
                println!("[Door 1] {}", p1_output);
                i += 1;
            }

            if c6.is_digit(10) {
                let c6 = c6.to_digit(10).unwrap() as usize;
                if c6 < 8 {
                    let c7 = digest.chars().nth(6).unwrap();
                    if p2_output[c6] == '-' {
                        p2_output[c6] = c7;
                        println!("[Door 2] {}", p2_output.iter().collect::<String>());
                        j += 1;
                    }
                }
            }
        }
        suffix += 1;
    }

    println!("=== RESULT ===");
    println!("Part 1: {}", p1_output);
    println!("Part 2: {}", p2_output.iter().collect::<String>());
}

fn load_input() -> String {
    std::fs::read_to_string(
        std::env::args().nth(1).expect("Filepath not provided.")
    ).expect("Could not load file!")
}