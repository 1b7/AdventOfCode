use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../../input/08");

    let mut antennae:HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    let max_x = input.lines().next().unwrap().chars().count() as i32;
    let max_y = input.lines().count() as i32;

    input.lines().enumerate().for_each(|(y, line)| {
        line.trim().chars().enumerate().for_each(|(x, chr)| {
            if chr != '.' {
                antennae.entry(chr).or_default().push((x as i32, y as i32));
            }
        })
    });

    let (p1, p2) = compute(&antennae, max_x, max_y);
    println!("Part 1: {p1}\nPart 2: {p2}");
}

fn find_antinodes (a: (i32, i32), b: (i32, i32), max_x: i32, max_y: i32) -> Vec<(i32, i32)> {
    let mut anodes = vec![];

    let dx = a.0 - b.0;
    let dy = a.1 - b.1;

    let anode_a = (a.0 + dx, a.1 + dy);
    let anode_b = (b.0 - dx, b.1 - dy);

    if anode_a.0 >= 0 && anode_a.1 >= 0 && anode_a.0 < max_x && anode_a.1 < max_y {
        anodes.push(anode_a)
    }

    if anode_b.0 >= 0 && anode_b.1 >= 0 && anode_b.0 < max_x && anode_b.1 < max_y {
        anodes.push(anode_b)
    }

    anodes
}
fn find_harmonics (a: (i32, i32), b: (i32, i32), max_x: i32, max_y: i32) -> Vec<(i32, i32)> {
    let mut harmonics = vec![a, b];

    let dx = a.0 - b.0;
    let dy = a.1 - b.1;

    let mut anode_a = (a.0 + dx, a.1 + dy);

    while anode_a.0 >= 0 && anode_a.1 >= 0 && anode_a.0 < max_x && anode_a.1 < max_y {
        harmonics.push(anode_a);
        anode_a = (anode_a.0 + dx, anode_a.1 + dy);
    }

    let mut anode_b = (b.0 - dx, b.1 - dy);

    while anode_b.0 >= 0 && anode_b.1 >= 0 && anode_b.0 < max_x && anode_b.1 < max_y {
        harmonics.push(anode_b);
        anode_b = (anode_b.0 - dx, anode_b.1 - dy);
    }

    harmonics
}

fn compute(antennae: &HashMap<char, Vec<(i32, i32)>>, max_x: i32, max_y:i32) -> (usize, usize) {
    let mut antinodes = HashSet::new();
    let mut harmonics = HashSet::new();
    
    for antenna in antennae.keys() {
        let ant = &antennae[antenna];
        for i in 0..(ant.len()) {
            for j in (i + 1)..(ant.len()) {
                let new_antinodes = find_antinodes(ant[i], ant[j], max_x, max_y);
                antinodes.extend(new_antinodes);

                let new_harmonics = find_harmonics(ant[i], ant[j], max_x, max_y);
                harmonics.extend(new_harmonics);
            }
        }
    }

    (antinodes.len(), harmonics.len())
}