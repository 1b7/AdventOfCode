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

fn dir_size(from: &str, dirs: &HashMap<String, HashSet<(&str, usize)>>, counted: &mut HashSet<String>) -> usize {
    let mut s = dirs.get(from).unwrap().iter().filter_map(|&(f, size)| {
        let full_path = [from, f].join("");
        let n = (!counted.contains(&full_path)).then(|| size);
        counted.insert(full_path);
        n
    }).sum();

    dirs.keys()
        .filter(|l| l.starts_with(from))
        .filter(|&p| p != from)
        .for_each(|d| s += dir_size(d, dirs, counted));
    s
}


fn navigate(s: &str) -> (usize, usize) {
    let mut dir = vec![];
    let mut dirs: HashMap<String, HashSet<(&str, usize)>> = HashMap::new();

    for line in s.lines() {
        if line.starts_with("$") {
            let splits: Vec<_> = line.split_whitespace().collect();
            match splits[1] {
                "ls" => (),
                "cd" => cd(splits[2], &mut dir),
                _ => panic!()
            };
        } else {
            let current_dir = dir.join("/");
            if !dirs.contains_key(&current_dir) {
                dirs.insert(current_dir.clone(), HashSet::new());
            }

            if !line.starts_with("dir") {
                let (fsize, fname) =  line.split_once(' ').unwrap();
                let fsize = fsize.parse().unwrap();
                dirs.get_mut(&current_dir).unwrap().insert((fname, fsize));
            }
        }
    }
    
    let sizes: Vec<_> = dirs.keys()
        .map(|k| dir_size(k, &dirs, &mut HashSet::new()) )
        .collect();

    let delta = 30_000_000 + sizes.iter().max().unwrap() - 70_000_000;
    (sizes.iter().filter(|&&n| n <= 100_000).sum(),
        *sizes.iter().filter(|&&n| n >= delta).min().unwrap())
}