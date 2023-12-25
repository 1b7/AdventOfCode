use std::collections::{HashMap, HashSet};

fn main() {
    let mut input: HashMap<_, _> = include_str!("../../../input/25")
        .lines()
        .map(|s| {
            let (src, dests) = s.split_once(": ").unwrap();
            (src, dests.split(" ").collect::<Vec<_>>())
        }).collect();

    let copy = input.clone();

    // Populate bi-directional links
    for (src, dests) in &copy {
        for d in dests {
            if let Some(v) = input.get_mut(d) {
                if !v.contains(&src) { v.push(src); }
            } else {
                input.insert(d, vec![src]);
            }
        }
    }
    
    // Generate lookup table to support graph HashMap -> Vec conversion.
    let lut = input.keys().enumerate().map(|(i, &k)| (k, i)).collect::<HashMap<&str, usize>>();

    let mut graph: Vec<Vec<usize>> = vec![vec![]; lut.len()];
    for (src, dests) in input {
        let dests = dests.iter().map(|d| lut.get(d).unwrap());
        graph[*lut.get(src).unwrap()].extend(dests);
    }

    // let removals = [("abc", "def"), ("geh", "ijk"), ("lmn", "opq")];
    panic!("Must first set `removals` variable; find these edges by analysing the graph");

    for (l, r) in removals {
        let a = *lut.get(l).unwrap();
        let b = *lut.get(r).unwrap();

        let node = graph.get_mut(a).unwrap();
        let loc = node.iter().enumerate().find(|(_i, &x)| x == b).unwrap().0;
        node.remove(loc);

        let node = graph.get_mut(b).unwrap();
        let loc = node.iter().enumerate().find(|(_i, &x)| x == a).unwrap().0;
        node.remove(loc);
    }

    let (_, s) = is_connected(&graph, &[]);
    println!("Answer: {}", s * (graph.len() - s));
}

/// Test if a graph is connected by trying to reach all other nodes; 
/// if any are unreachable then the number of nodes visited will be less than
/// the number in the graph as a whole.
fn is_connected(g: &[Vec<usize>], ignore_edges: &[(usize, usize)]) -> (bool, usize) {
    let mut frontier = vec![0];
    let mut visited = HashSet::new();
    while !frontier.is_empty() {
        let node = frontier.pop().unwrap();
        if visited.contains(&node) { continue; }
        visited.insert(node);

        for &dest in &g[node] {
            if !ignore_edges.contains(&(node, dest)) && !ignore_edges.contains(&(dest, node)) {
                frontier.push(dest);
            }
        }
    }
    (visited.len() == g.len(), visited.len())
}