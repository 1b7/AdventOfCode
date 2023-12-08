use std::{collections::HashMap, time::Instant};

fn main() {
    let input = include_str!("../../../input/08");

    let s_prep = Instant::now();

    let (directions, nodes) = input.split_once("\n\n").unwrap();

    let mut map_vec = nodes.lines().map(|line| {
        let (pos, connections) = line.split_once(" = ").unwrap();
        let connections = connections.replace(['(', ')'], "");
        let (l, r) = connections.split_once(", ").unwrap();
        (pos.to_string(), (l.to_string(), r.to_string()))
    }).collect::<Vec<_>>();
    map_vec.sort();
    
    let mut vmap = vec![(0, 0); map_vec.len()];
    for i in 0..map_vec.len() {
        let left = map_vec.binary_search_by(|a| a.0.cmp(&map_vec[i].1.0)).unwrap();
        let right = map_vec.binary_search_by(|a| a.0.cmp(&map_vec[i].1.1)).unwrap();
        vmap[i] = (left, right);
    }
    let e_prep = s_prep.elapsed().as_micros();
    println!("Pre-Processing time: {e_prep}us");

    let s_p1 = Instant::now();
    let p1 = p1(&vmap, &map_vec, directions);
    let e_p1 = s_p1.elapsed().as_micros();
    println!("Part 1: {p1}, elapsed: {e_p1}us");

    let s_p2 = Instant::now();
    let p2 = p2(&vmap, &map_vec, directions);
    let e_p2 = s_p2.elapsed().as_micros();
    println!("Part 2: {p2}, elapsed: {e_p2}us");
}

fn p1(vmap: &[(usize, usize)], map_vec: &[(String, (String, String))], directions: &str) -> usize {
    let start = map_vec.iter().enumerate()
        .filter_map(|(i, (k, _))| if k == &"AAA" { Some(i) } else { None })
        .collect::<Vec<_>>();

    let end = map_vec.iter().enumerate()
        .filter_map(|(i, (k, _))| if k == &"ZZZ" { Some(i) } else { None })
        .collect::<Vec<_>>();
    
    let mut pos = start[0];
    let mut steps = 0;
    for d in directions.chars().cycle() {
        if d == 'L' {
            pos = vmap[pos].0;
        } else {
            pos = vmap[pos].1;
        }
        steps += 1;
        if pos == end[0] { break }
    }
    steps
}

fn p2(vmap: &[(usize, usize)], map_vec: &[(String, (String, String))], directions: &str) -> usize {
    let starts = map_vec.iter().enumerate()
        .filter_map(|(i, (k, _))| if k.ends_with('A') { Some(i) } else { None })
        .collect::<Vec<_>>();

    let ends = map_vec.iter().enumerate()
        .filter_map(|(i, (k, _))| if k.ends_with('Z') { Some(i) } else { None })
        .collect::<Vec<_>>();


    let mut positions = starts.to_owned();
    let mut steps = 0;
    
    let mut phases = vec![0; positions.len()];
    let mut phase_count = 0;

    for d in directions.chars().cycle() {
        for i in 0..positions.len() {
            if phases[i] == 0 && ends.contains(&positions[i]) {
                phase_count += 1;
                phases[i] = steps;  
            }
            if d == 'L' {
                positions[i] = vmap[positions[i]].0;
            } else {
                positions[i] = vmap[positions[i]].1;
            }
        }
        steps += 1;
        if phase_count == phases.len() { break }
    }

    ((phases[0]..).step_by(phases[0])).find(|i| phases[1..].iter().all(|x| i % x == 0)).unwrap()
}