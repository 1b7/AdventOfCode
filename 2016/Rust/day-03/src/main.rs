fn main() {
    let s = load_input();
    println!("{}", possible_triangles(&s, read_rows));
    println!("{}", possible_triangles(&s, read_cols));
}

fn possible_triangles(s: &str, read_fn: fn(&str) -> Vec<[u16; 3]>) -> i32 {
    let mut n_possible = 0;
    for mut sides in read_fn(s) {
        sides.sort();
        if sides[0] + sides[1] > sides[2] { n_possible += 1; }
    }
    n_possible
}

fn read_rows(s: &str) -> Vec<[u16; 3]> {
    s.lines().map(|line| {
        let ns: Vec<_> = line.split_whitespace()
            .map(|n| n.parse().expect("Could not convert string to number"))
            .collect();
        [ns[0], ns[1], ns[2]]
    }).collect()
} 

fn read_cols(s: &str) -> Vec<[u16; 3]> {
    let mut tris = vec![];
    let (mut a, mut b, mut c) = ([0; 3], [0; 3], [0; 3]);

    let mut row_counter = 0;
    for line in s.lines() {
        let sides: Vec<u16> = line.split_whitespace()
            .map(|n| n.parse().expect("Could not convert string to number"))
            .collect();

        a[row_counter] = sides[0];
        b[row_counter] = sides[1];
        c[row_counter] = sides[2];
        
        row_counter += 1;
        if row_counter > 2 {
            tris.push(a);
            tris.push(b);
            tris.push(c);
            row_counter = 0;
            (a, b, c) = ([0; 3], [0; 3], [0; 3]);
        }
    }

    tris
}

fn load_input() -> String {
    std::fs::read_to_string(
        std::env::args().nth(1).expect("Filepath not provided.")
    ).expect("Could not load file!")
}