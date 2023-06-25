fn main() {
    let s = load_input();

    let count_tls: i32 = s.lines().map(|line| if supports_tls(line) { 1 } else { 0 }).sum();
    let count_sls: i32 = s.lines().map(|line| if supports_sls(line) { 1 } else { 0 }).sum();
        
    dbg!(count_tls, count_sls);
}

fn supports_tls(s: &str) -> bool {
    let c_vec = s.chars().collect::<Vec<_>>();
    let mut in_hypernet = false;
    let mut has_abba = false;
    for window in c_vec.windows(4) {
        if window[0] == '[' { in_hypernet = true  }
        if window[0] == ']' { in_hypernet = false }
        if window[0] == window[3] && window[1] == window[2] && window[0] != window[1] {
            if in_hypernet {
                return false
            } else {
                has_abba = true;
            }
        }
    }
    has_abba
}

fn supports_sls(s: &str) -> bool {
    let c_vec = s.chars().collect::<Vec<_>>();
    let mut super_abas: Vec<&[char]> = vec![];
    let mut hyper_abas: Vec<&[char]> = vec![];
    let mut in_hypernet = false;

    for window in c_vec.windows(3) {
        if window[0] == '[' { in_hypernet = true  }
        if window[0] == ']' { in_hypernet = false }
        if window[1] == '[' || window[1] == ']' { continue }
        if window[0] == window[2] && window[0] != window[1] {
            if in_hypernet {
                hyper_abas.push(&window[0..=2])
            } else {
                super_abas.push(&window[0..=2])
            }
        }
    }

    for aba in &super_abas {
        let search_pattern = vec![aba[1], aba[0], aba[1]];
        if hyper_abas.contains(&&search_pattern[..]) {
            println!("{}", true);
            return true 
        }
    }
    
    false
}

fn load_input() -> String {
    std::fs::read_to_string(
        std::env::args().nth(1).expect("Filepath not provided.")
    ).expect("Could not load file!")
}