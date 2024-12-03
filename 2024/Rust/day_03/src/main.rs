use regex::{Regex, Match};

fn main() {
    let input = include_str!("../../../input/03");
    let (p1, p2) = solve(input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve(input: &str) -> (i32, i32) {
    // Utility Function: Given a list of locations and an end location,
    // finds the most-recent instance of that location, which comes before this boundary.
    fn find_last_instruction(end: usize, xs: &[usize]) -> Option<&usize> {
        xs.iter().rev().find(|&&i| i < end)
    }

    // Set up Regular Expressions for text searching:
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Failed to compile regex");

    let re_do = Regex::new(r"do\(\)").expect("Failed to compile regex");
    let do_idxs = re_do.find_iter(input).map(|m| m.start()).collect::<Vec<_>>();

    let re_dont = Regex::new(r"don't\(\)").expect("Failed to compile regex");
    let dont_idxs = re_dont.find_iter(input).map(|m| m.start()).collect::<Vec<_>>();

    // Convert a matched integer substring to an actual integer:
    let as_int = |c: Option<Match>| -> i32 {
        c.unwrap().as_str().parse().unwrap()
    };

    // Search over the input string, find `mul` instructions and apply them as appropriate.
    let (mut p1, mut p2) = (0, 0);
    for m in re.captures_iter(input) {
        let mul_location = m.get(0).unwrap().start();
        let dont_idx = find_last_instruction(mul_location, &dont_idxs);
        let do_idx = find_last_instruction(mul_location, &do_idxs);

        let product = as_int(m.get(1)) * as_int(m.get(2));
        if dont_idx.is_none() || (!do_idx.is_none() && dont_idx.unwrap() <= do_idx.unwrap()) {
            p2 += product;
        }
        p1 += product;
    }
    (p1, p2)
}