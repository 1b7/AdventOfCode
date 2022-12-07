use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../../input/07");
    println!("Part 1: {:?}", navigate(input));
}

fn cd(i: &str, cur:&mut Vec<String>) {
    match i {
        ".." => { cur.pop(); },
        "/" => { cur.clear(); },
        _ => {   cur.push(i.to_string()); }
    };
}

fn size(from: &str, dirs: &HashMap<String, HashSet<(&str, usize)>>, counted: &mut HashSet<String>) -> usize {
    let mut s = 0;
    for &(f, size) in dirs.get(from).unwrap() { 
        let mut new_s = from.to_owned(); new_s.push_str(f);
        if !counted.contains(&new_s) {
            s += size; 
            counted.insert(new_s);
        }
    }

    let ds: Vec<_> = dirs.keys()
        .filter(|l| l.starts_with(from))
        .filter(|&p| p != from)
        .collect();

    for d in ds { s += size(d, dirs, counted); }
    s
}


fn navigate(s: &str) -> (usize, usize) {
    let mut dir = vec![];
    let mut dirs: HashMap<String, HashSet<(&str, usize)>> = HashMap::new();
    dirs.insert("".to_string(), HashSet::new());

    for line in s.lines() {
        if line.starts_with("$") {
            let splits: Vec<_> = line.split_whitespace().take(3).collect();
            match splits[1] {
                "ls" => (),
                "cd" => cd(splits[2], &mut dir),
                _ => panic!()
            };
        } else if !line.starts_with("dir") { // is file
            let (size, name) =  line.split_once(" ").unwrap();
            let size = size.parse().unwrap();
            let current = dir.join("/");

            if !dirs.contains_key(&current) {
                dirs.insert(current.clone(), HashSet::new());
            }
            
            dirs.get_mut(&current).unwrap().insert((name, size));
        }
    }
    
    let sizes: Vec<_> = dirs.keys()
    .map(|k| 
        size(k, &dirs, &mut HashSet::new())
    ).collect();

    let free_space = 70_000_000 - sizes.iter().max().unwrap();
    let delta = 30_000_000  - free_space;

    (sizes.iter().filter(|&&n| n <= 100_000).sum(),
        *sizes.iter().filter(|&&n| n >= delta).min().unwrap())
}