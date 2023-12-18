use std::time::Instant;

use num_bigint::{BigInt, ToBigInt};
use num_traits::Signed;

fn main() {
    // Same functions are used for both parts - the only difference is in input
    // processing, so for efficiency, both are handled simultaneously in `run`.
    let instructions = include_str!("../../../input/18");
    let t = Instant::now();
    let (p1, p2) = run(instructions);
    let e = t.elapsed().as_micros();
    println!("Part 1: {p1}\nPart 2: {p2}\n({e}us)");
}

fn run(instructions: &str) -> (BigInt, BigInt) {
    fn from_hex(s: &str) -> (i32, i32) {
        let (n, d) = s.split_at(7);
        let n = i32::from_str_radix(&n.replace("(#", ""), 16).unwrap();
        let d = i32::from_str_radix(&d.replace(")", ""), 16).unwrap();
        (n, d)
    }

    let (mut p1_r, mut p1_c) = (0, 0);
    let mut p1_points = vec![(p1_r, p1_c)];

    let (mut p2_r, mut p2_c) = (0, 0);
    let mut p2_points = vec![(p2_r, p2_c)];

    instructions.lines().for_each(|line| {
        let mut coords = line.split_whitespace();
        // Part 1:
        let (dir, steps) = (coords.next().unwrap(), coords.next().unwrap().parse::<i32>().unwrap());
        let (dr, dc) = match dir {
            "R" => ( 0,  steps),
            "L" => ( 0, -steps),
            "D" => ( steps,  0),
            "U" => (-steps,  0),
             _  => panic!("Unrecognised Direction in input")
        };
        (p1_r, p1_c) = (p1_r + dr, p1_c + dc);
        p1_points.push((p1_r, p1_c));

        // Part 2:
        let hex = coords.next().unwrap();
        let (steps, dir) = from_hex(hex);
        let (dr, dc) = match dir {
            0 => ( 0,  steps),
            1 => ( steps,  0),
            2 => ( 0, -steps),
            3 => (-steps,  0),
             _  => panic!("Unrecognised Direction in input")
        };

        (p2_r, p2_c) = (p2_r + dr, p2_c + dc);
        p2_points.push((p2_r, p2_c));
    });

    (poly_area(&p1_points), poly_area(&p2_points))
}

fn poly_area(poly: &[(i32, i32)]) -> BigInt {
    area(&poly) + ((perimeter(&poly)/ 2) + 1)
}

fn area(coords: &[(i32, i32)]) -> BigInt {
    let mut area = 0.to_bigint().unwrap();
    for i in 0..coords.len() {
        let j = (i + 1) % coords.len();
        area += coords[i].1.to_bigint().unwrap() * coords[j].0.to_bigint().unwrap();
        area -= coords[i].0.to_bigint().unwrap() * coords[j].1.to_bigint().unwrap();
    }
    area / 2
}

fn perimeter(coords: &[(i32, i32)]) -> BigInt {
    let mut peri = 0.to_bigint().unwrap();
    for i in 0..coords.len() {
        let j = (i + 1) % coords.len();
        peri += (coords[i].0.to_bigint().unwrap() - coords[j].0.to_bigint().unwrap()).abs();
        peri += (coords[i].1.to_bigint().unwrap() - coords[j].1.to_bigint().unwrap()).abs();
    }
    peri
}