fn main() {
    let dial_range = 100;
    let initial_position = 50;

    let mut zeros_landed_on = 0;
    let mut zeros_passed = 0;

    include_str!("../../../../input/2025/01")
        .lines()
        .map(parse_line)
        .fold(initial_position, |pos, dist| {
            let offset = dist % dial_range;
            let new_pos = (pos + offset).rem_euclid(dial_range);

            if new_pos == 0 {
                zeros_landed_on += 1;
            }

            if pos != 0 && (pos + offset < 0 || pos + offset > dial_range) {
                zeros_passed += 1
            }
            zeros_passed += dist.abs() / dial_range;

            new_pos
        });

    println!("Part 1: {}", zeros_landed_on);
    println!("Part 2: {}", zeros_passed + zeros_landed_on);
}

fn parse_line(line: &str) -> i32 {
    let (direction, distance) = line.split_at(1);

    let distance = distance
        .parse()
        .expect("Got non-numeric value for distance");

    match direction {
        "R" => distance,
        "L" => -distance,
        _ => panic!("Invalid direction passed in"),
    }
}
