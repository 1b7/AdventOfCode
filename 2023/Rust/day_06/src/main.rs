use std::time::Instant;

fn main() {
    let input = include_str!("../../../input/06");

    let s = Instant::now();
    let p1 = p1(input);
    let p1_t = s.elapsed().as_micros();
    println!("Part 1: {p1} ({p1_t}us)");

    let s = Instant::now();
    let p2 = p2(input);
    let p2_t = s.elapsed().as_micros();
    println!("Part 2: {p2} ({p2_t}us)");

}

fn p1(input: &str) -> usize {
    let mut input = input
        .lines()
        .map(|l| l.split_whitespace().skip(1).map(|n| n.parse::<usize>().unwrap()));

    let races = input.next().unwrap().zip(input.next().unwrap()).collect::<Vec<_>>();
    races.iter().map(|&(t, d)| wins(t, d)).product::<usize>()
}

fn p2(input: &str) -> usize {
    let mut input = input.lines().map(|l| {
        l.split_once(':').unwrap().1.trim().replace(' ', "").parse::<usize>().unwrap()
    });

    let (time, dist) = (input.next().unwrap(), input.next().unwrap());
    wins(time, dist)
}

fn wins(time: usize, dist: usize) -> usize {
    (0..time).fold(0, |acc, wind_time| acc + (wind_time * (time - wind_time) > dist) as usize)
}