fn main() {
    let input = include_str!("../../../input/02");
    let (p1, p2) = compute(input);
    println!("Part 1: {p1}, Part 2: {p2}");
}

// Given a list of numbers, determine whether it is 'safe'
fn has_fault(nums: &[i32]) -> bool {
    let mut list_sign = None;
    for pair in nums.windows(2) {
        let sign = Some((pair[0] - pair[1]).signum());
        if list_sign.is_none() { list_sign = sign }

        let diff = pair[0].abs_diff(pair[1]);
        if diff == 0 || diff > 3 || sign != list_sign { return true }
    }
    false
}

// Given raw text input, calculates both the number of safe reports,
// and the total number of safe reports including dampening.
fn compute(input: &str) -> (i32, i32) {
    let (mut safe, mut dampened) = (0, 0);

    input.lines().for_each(|line| {
        let nums = line.split_whitespace()
            .map(|x| x.parse::<i32>().expect("ERROR: Not a number"))
            .collect::<Vec<_>>();

        if has_fault(&nums) {
            // Fault detected; try to dampen it.
            for i in 0..nums.len() {
                let mut sublist =  nums.clone();
                sublist.remove(i);
                if !has_fault(&sublist) {
                    dampened += 1;
                    break;
                }
            }
        } else {
            safe += 1;
        }
    });

    (safe, safe + dampened)
}