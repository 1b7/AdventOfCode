fn main() {
    let mut spheres: Vec<_> = load_input().lines().map(Sphere::from_str).collect();
    println!("Part 1: {}", solve(&spheres));
    spheres.push(Sphere {total: 11, init: 0});
    println!("Part 2: {}", solve(&spheres));
}

fn solve(spheres: &[Sphere]) -> usize {
    let first_alignment = spheres[0].total - spheres[0].init;
    for t in (first_alignment..).step_by(spheres[0].total) {
        if spheres[1..].iter().enumerate().all(|(i, sphere)| {
            (sphere.init + 1 + t + i) % sphere.total == 0
        }) { return t - 1 }
    }
    0
}

struct Sphere { total: usize, init: usize }
impl Sphere {
    pub fn from_str(s: &str) -> Self {
        let s: Vec<_> = s.split_whitespace().collect();
        Sphere {
            total: s[3].parse().unwrap(),
            init: s[11].trim_end_matches('.').parse().unwrap()
        }
    }
}

fn load_input() -> String {
    std::fs::read_to_string(
        std::env::args().nth(1).expect("Filepath not provided.")
    ).expect("Could not load file!")
}