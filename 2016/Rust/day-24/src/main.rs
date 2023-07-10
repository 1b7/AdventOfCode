use std::{collections::{VecDeque, HashSet, BinaryHeap}, cmp::Reverse};

fn main() {
    let map = load_input();
    let g = parse(&map);
    println!("Part 1: {}", uniform_cost(&g, true));
    println!("Part 2: {}", uniform_cost(&g, false));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile { Node(u8), Space, Wall }
impl Tile {
    pub fn is_node(&self) -> bool { matches!(self, Tile::Node(_)) }
    pub fn node_val(&self) -> Option<u8> { 
        if let Tile::Node(n) = self { Some(*n) }
        else { None }
    }
}

fn uniform_cost(g: &[Vec<u16>], p1: bool) -> usize {
    let mut front = BinaryHeap::new();
    front.push((Reverse(0), 0, vec![0]));

    while !front.is_empty() {
        let (cost, node, visited) = front.pop().unwrap();

        if (p1 && visited.len() == g.len()) || (!p1 && visited.len() == (g.len() + 1)) { 
            return cost.0 as usize 
        }

        for i in 0..g[node].len() {
            let return_to_start = !p1 && visited.len() == g.len() && i == 0;
            if !visited.contains(&i) || return_to_start {
                let mut new_visited = visited.clone();
                new_visited.push(i);
                front.push((Reverse(cost.0 + g[node][i]), i, new_visited));
            }
        }
    }

    panic!("No solution found.")
}

fn parse(s: &str) -> Vec<Vec<u16>> {
    let width = s.lines().next().unwrap().len(); 
    let mut node_count = 0;
    let map: Vec<_> = s.lines()
        .flat_map(|line| line.chars())
        .map(|c| {
            match c {
                '.' => Tile::Space,
                '#' => Tile::Wall,
                x  => {
                    node_count += 1;
                    Tile::Node(x as u8 - b'0')
                }
            }
        }).collect();

    let mut all = map.iter().enumerate().filter_map(|(i, x)| {
        if let Tile::Node(n) = x {
            let mut dests = vec![u16::MAX; node_count];
            locate_nodes(node_count - 1, (i % width, i / width), &map, width, &mut dests);
            Some((n, dests))
        } else { None }
    }).collect::<Vec<_>>();

    all.sort();
    all.into_iter().map(|(_, b)| b).collect::<Vec<_>>()
}

fn locate_nodes(limit: usize, start: (usize, usize), map: &[Tile], width: usize, dests: &mut [u16]) {
    let height = map.len() / width;
    let flatten_idx = |x: usize, y: usize| x + (y * width);

    let mut options = VecDeque::new();
    options.push_back(vec![start]);
    let mut found = 0;

    let mut seen = HashSet::new();
    seen.insert(flatten_idx(start.0, start.1));

    while found < limit && !options.is_empty() {
        let path = options.pop_front().unwrap();
        let dist = (path.len() - 1).try_into().unwrap();
        let (x, y) = path[dist as usize];

        // Test to see if current cell is node, and if we have 
        // a new shortest-path to it.
        if (x, y) != start && map[flatten_idx(x, y)].is_node() {
            let i = map[flatten_idx(x, y)].node_val().unwrap();
            if dests[i as usize] == u16::MAX {
                found += 1;
                dests[i as usize] = dist;
            }
        } else {
            let mut append = |nx, ny| {
                let i = flatten_idx(nx, ny);
                if map[flatten_idx(nx, ny)] == Tile::Wall || seen.contains(&i) { 
                    return 
                }
                seen.insert(i);
                let mut new_path = path.clone();
                new_path.push((nx, ny));
                options.push_back(new_path)
            };
    
            if x > 0            { append(x - 1, y); }
            if x < (width  - 1) { append(x + 1, y); }
            if y > 0            { append(x, y - 1); }
            if y < (height - 1) { append(x, y + 1); }
        }
    }
}

fn load_input() -> String {
    std::fs::read_to_string(
        std::env::args().nth(1).expect("Filepath not provided.")
    ).expect("Could not load file!")
}