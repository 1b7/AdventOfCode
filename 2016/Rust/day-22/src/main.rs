use std::{cmp::Reverse, collections::{HashSet, BinaryHeap}};

/*  Part Two Explanation:
    Instead of trying to solve this like a sliding-tile puzzle; we treat it as
    a pathfinding problem: how many steps does it take to 'move' the free space 
    until it lies left of the goal data?

    Once we have worked that out, we could use a repeated cycle of 5 moves
    to shift the goal data along a direct path towards the top-left node.
    This approach does assume that the path along the top two rows is clear.
    
    The total number of moves then must be that initial distance + the number
    of cycles needed to get back to the left side of the network multiplied by 
    a cycle's 5 steps. */

fn main() {
    let input = load_input();
    let nodes = input.lines().skip(2).map(parse_line).collect::<Vec<_>>();
    
    
    let mut sorted = nodes.clone();
    sorted.sort_by(|a, b| a.avail.cmp(&b.avail));
    println!("Part 1: {}", viable_pairs(&sorted));

    let (mut max_x, mut max_y) = (0, 0);
    for node in &nodes {
        max_x = max_x.max(node.px);
        max_y = max_y.max(node.py);
    }

    let dist = dist_to_goal((max_x + 1) as usize, (max_y + 1) as usize, nodes);
    println!("Part 2: {}", dist + (max_x as usize - 1) * 5);
}

fn dist_to_goal(width: usize, height: usize, nodes: Vec<Node>) -> usize {
    let target = (width * height) - height;
    let loc_empty = nodes.iter().enumerate()
        .find(|&n| n.1.used == 0)
        .unwrap().0;
    
    // Heuristic function used for A* implementation; uses manhattan distance
    // from target node to our empty space.
    let h = |from: usize| -> u16 {
        let d = from.abs_diff(width - 1);
        let d = (d / width + d % width) as u16;
        d * d
    };

    let mut visited = HashSet::new();
    visited.insert(loc_empty);

    let mut frontier = BinaryHeap::new();
    frontier.push((Reverse(h(loc_empty)), Reverse(0), loc_empty, nodes));

    while frontier.len() > 0 {
        let (_, steps, free_space, next) = frontier.pop().unwrap();
        let steps = steps.0;

        if free_space == (target) { return steps as usize }
        let mut expand = |x| {
            if let Some(new_state) = try_swap(x, free_space, &next) {
                if !visited.contains(&x) {
                    visited.insert(x);
                    let heuristic = steps + 1 + h(x);
                    // let heuristic = steps + 1;
                    frontier.push((Reverse(heuristic), Reverse(steps + 1), x, new_state));
                }
            }
        };

        if  next[free_space].px           > 0            { expand(free_space - height); } // Left 
        if (next[free_space].px as usize) < (width - 1)  { expand(free_space + height); } // Right
        if  next[free_space].py           > 0            { expand(free_space - 1);      } // Up
        // Moving down wouldn't make sense; so is intentionally not implemented.
    }
    panic!("No solution found.")
}


fn try_swap(f: usize, t: usize, nodes: &[Node]) -> Option<Vec<Node>> {
    if nodes[t].size < nodes[f].used { return None; }

    let mut copy = nodes.to_vec();
    copy[t].used += nodes[f].used;
    copy[f].avail = nodes[f].size;
    copy[f].used = 0;
    Some(copy)
} 

fn viable_pairs(nodes: &[Node]) -> usize {
    let mut fits = 0;
    for i in 0..nodes.len() {
        for j in 0..nodes.len() {
            if nodes[i].used != 0 && nodes[j].avail >= nodes[i].used {
                fits += nodes.len() - j;
                if i >= j { fits -= 1; } // Don't count self!
                break;
            }
        }
    }
    fits
}

fn parse_line(s: &str) -> Node {
    let s = s.replace("/dev/grid/node-x", "")
        .replace(&['x', 'y', 'T', '%'], "");
    let s = s.split_terminator(&[' ', '-'])
        .filter(|&s| s != " " && s != "")
        .map(|n| n.parse::<u16>().unwrap())
        .collect::<Vec<_>>();
    
    Node::new(s[0] as u8, s[1] as u8, s[2], s[3], s[4])
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node {
    px: u8,
    py: u8,
    size: u16,
    used: u16,
    avail: u16,
    uid: u16
}

fn as_uid(x: u8, y: u8) -> u16 {
    x as u16 + y as u16 * 100
}

impl Node {
    pub fn new(px: u8, py: u8, size: u16, used: u16, avail: u16) -> Self {
        Self { px, py, size, used, avail, uid: as_uid(px, py)}
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} / {}]", self.used, self.size)
    }
}

fn load_input() -> String {
    std::fs::read_to_string(
        std::env::args().nth(1).expect("Filepath not provided.")
    ).expect("Could not load file!")
}