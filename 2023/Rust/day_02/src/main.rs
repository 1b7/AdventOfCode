use std::time::Instant;

fn main() {
    let s = include_str!("../../../input/02");
    
    let t = Instant::now();
    let (p1, p2) = cubes(s);
    let e = t.elapsed().as_micros();

    println!("Part 1: {}\nPart 2:{}", p1, p2);
    println!("Took {}us", e);
}

fn cubes(s: &str) -> (u32, u32) {
    let mut sum = 0;
    let mut sum_id = 0;
    let n_red = 12; let n_green = 13; let n_blue = 14;

    for line in s.lines() {
        let (id, rest) = line.split_once(':').unwrap();
        let (_, id) = id.split_once(' ').unwrap();
        let id = id.parse::<u32>().unwrap();
        let mut min_red = 0; let mut min_blue = 0; let mut min_green = 0;
        let mut valid_game = true;

        for set in rest.split(';') {
            let pairs = set.split(',');
            pairs.for_each(|pair| {
                let pair = pair.trim();
                let (count, colour) = pair.split_once(' ').unwrap();
                let count = count.parse::<u32>().unwrap();

                valid_game = valid_game && (
                    colour == "green" && count <= n_green 
                    || colour == "red" && count <= n_red
                    || colour == "blue" && count <= n_blue);

                match colour {
                    "green" => { min_green = min_green.max(count); }
                    "red" => { min_red = min_red.max(count); }
                    "blue" => { min_blue = min_blue.max(count); }
                    _ => panic!("Unexpected Colour")
                };
            });
        }
        if valid_game { sum_id += id; }
        sum += min_red * min_blue * min_green;
    }
    (sum_id, sum)
}