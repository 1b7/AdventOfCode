use std::{collections::{BinaryHeap, HashSet, VecDeque, HashMap}, time::Instant, sync::LockResult};

fn main() {
    let mut input: Vec<_> = include_str!("../../../input/23")
        .lines()
        .map(|line| line.chars().map(Tile::from_char).collect::<Vec<_>>())
        .collect();

    println!("Part 1: {}", p1((0, 1), ((input.len() - 1 )as i16, (input[0].len() - 2) as i16), &input));
    
    // PART 2:
    // Replace start points of maze with walls for easier navigation.
    input[0][1] = Tile::Wall;
    let (nrow, ncol) = (input.len() - 1, input[0].len() - 2);
    input[nrow][ncol] = Tile::Wall;

    // Set up the target destinations for Part 2
    let (start, end) = ((1, 1), ((input.len() - 2) as i16, (input[0].len() - 2) as i16));

    // Condense the maze to a simple directed graph:
    let mut g = as_graph((1, 1), &input);

    // Add the start/end points as nodes, by using `find_branches` to work out
    // the nearest 'decision point' (i.e. node) to them and the distance to that node.
    let (start_node, start_dist, _) = find_branches(start, &input, start);
    let (end_node, end_dist, _) = find_branches(end, &input, end);
    g.insert(start, vec![(start_node, start_dist)]);
    g.get_mut(&end_node).unwrap().push((end, end_dist));

    // Use a similar algorithm as in Part 1 to find the longest path through it:
    println!("Part 2: {}", p2(start, end, g));

}

fn find_branches(from: (i16, i16), map: &[Vec<Tile>], ignore: (i16, i16)) -> 
    ((i16, i16), usize, Vec<(i16, i16)>) {
    let (mut r, mut c) = from;
    let mut prev = from;
    let mut dist = 0;

    let adj = loop {
        let options = [(r - 1, c), (r, c + 1), (r + 1, c), (r, c - 1)];
        let valid_opts = options.into_iter()
            .filter(|&o| o != prev && o != ignore
                    && o.0 >= 0 && o.1 >= 0 && o.0 < map.len() as i16 && o.1 < map[0].len() as i16
                    &&  map[o.0 as usize][o.1 as usize] != Tile::Wall)
            .collect::<Vec<_>>();

        dist += 1;
        if valid_opts.len() == 1 {
            prev = (r, c);
            (r, c) = valid_opts[0];
        } else {
            break valid_opts
        }
    };

    ((r,c), dist, adj)
}

fn as_graph(from: (i16, i16), map: &[Vec<Tile>]) -> HashMap<(i16, i16), Vec<((i16, i16), usize)>> {
    let mut frontier = VecDeque::from([(from, from)]);
    let mut edges = vec![];
    let mut branch_points = HashSet::new();

    while !frontier.is_empty() {
        let (from, ignore) = frontier.pop_front().unwrap();
        let (end, dist, dests) = find_branches(from, map, ignore);
        if branch_points.contains(&(ignore, end, dist)) || branch_points.contains(&(end, ignore, dist)) { continue }
        branch_points.insert((ignore, end, dist));

        edges.push((ignore, end, dist));

        for d in dests {
            frontier.push_back((d, end));
        }
    }

    let mut hm_edges: HashMap<(i16, i16), Vec<((i16, i16), usize)>> = HashMap::new();
    for (src, to, dist) in edges {
        if let Some(dests) = hm_edges.get_mut(&src) {
            dests.push((to, dist));
        } else {
            hm_edges.insert(src, vec![(to, dist)]);
        }

        if let Some(dests) = hm_edges.get_mut(&to) {
            dests.push((src, dist));
        } else {
            hm_edges.insert(to, vec![(src, dist)]);
        }
    }

    hm_edges
}

fn p2(start: (i16, i16), end: (i16, i16), g: HashMap<(i16, i16), Vec<((i16, i16), usize)>>) -> usize {
    let mut frontier = BinaryHeap::new();
    let hp = HeapPair(0, start, HashSet::<(i16, i16)>::from([start]));
    frontier.push(hp);

    let mut max_dist = 0;

    while frontier.len() > 0 {
        let hp = frontier.pop().unwrap();
        let (dist, (r, c), visited) = (hp.0, hp.1, hp.2);
        if (r,c) == end {
            max_dist = max_dist.max(dist);
            continue;
        }
        
        let expansions = g.get(&(r, c)).unwrap();

        for (option, distance) in expansions {
            if !visited.contains(option) {
                let mut new_visited= visited.clone();
                new_visited.insert(*option);
                frontier.push(HeapPair(dist + *distance as i16, *option, new_visited));
            }
        }
    }

    max_dist as usize
}

/// Use a brute-force depth-first-search until we find the longest path.
fn p1(start: (i16, i16), end: (i16, i16), map: &[Vec<Tile>]) -> usize {
    let mut frontier = BinaryHeap::new();
    let hp = HeapPair(end.0 + end.1, start, HashSet::<(i16, i16)>::from([start]));
    frontier.push(hp);

    let mut max_dist = 0;

    while frontier.len() > 0 {
        let hp = frontier.pop().unwrap();
        let (_v, (r, c), visited) = (hp.0, hp.1, hp.2);
        if (r,c) == end {
            max_dist = max_dist.max(visited.len());
            continue;
        }

        let tile = map[r as usize][c as usize];
        let (up, right, down, left) = ((r - 1, c), (r, c + 1), (r + 1, c), (r, c - 1));
        let expansion = match tile {
            Tile::Floor      => vec![up, left, right, down],
            Tile::SlopeUp    => vec![up],
            Tile::SlopeRight => vec![right],
            Tile::SlopeDown  => vec![down],
            Tile::SlopeLeft  => vec![left],
            Tile::Wall => { continue; },
        };

        for e in expansion {
            if e.0 >= 0 && e.1 >= 0 && e.0 < map.len() as i16 && e.1 < map[0].len() as i16
            && !visited.contains(&e) && map[e.0 as usize][e.1 as usize] != Tile::Wall {
                let mut new_visited= visited.clone();
                new_visited.insert(e);
                let new_v = (new_visited.len() as i16) + ((end.0 - e.0) + (end.1 - e.0));
                frontier.push(HeapPair(new_v, e, new_visited));
            }
        }
    }

    max_dist - 1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile { Floor, Wall, SlopeLeft, SlopeUp, SlopeRight, SlopeDown }
impl Tile {
    pub fn from_char(c: char) -> Self {
        match c {
            '#' => Tile::Wall,
            '.' => Tile::Floor,
            '>' => Tile::SlopeRight,
            '<' => Tile::SlopeLeft,
            '^' => Tile::SlopeUp,
            'v' => Tile::SlopeDown,
            _ => panic!("Invalid Input")
        }
    }
}

/// Utility type to let us put HashSets into our frontiers (when implemented as a BinaryHeap).
/// Effectively we're just wrapping it in a tuple-struct which then ignores the HashSet for
/// the sake of Ordering.
#[derive(Debug, Clone, PartialEq, Eq)]
struct HeapPair(i16, (i16, i16), HashSet<(i16, i16)>);
impl PartialOrd for HeapPair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for HeapPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}