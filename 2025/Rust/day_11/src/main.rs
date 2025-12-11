use std::collections::{HashMap, VecDeque};

fn parse_line(line: &str) -> (&str, Vec<&str>) {
    let (src, destinations) = line.split_once(':').unwrap();
    (src.trim(), destinations.trim().split_whitespace().collect())
}

fn main() {
    let graph: HashMap<_, _> = include_str!("../../../../input/2025/11")
        .lines()
        .map(parse_line)
        .collect();

    let p1 = path_to(&graph, "you", "out");

    let p2 = (
        path_to(&graph, "svr", "fft")
            * path_to(&graph, "fft", "dac")
            * path_to(&graph, "dac", "out")
    ) + (
        path_to(&graph, "svr", "dac")
            * path_to(&graph, "dac", "fft")
            * path_to(&graph, "fft", "out")
    );

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}


// This is a DAG so we don't need to worry about cycles.
fn path_to(graph: &HashMap<&str, Vec<&str>>, from: &str, to: &str) -> usize {
    let mut queue = VecDeque::from([(from, 1)]);
    let mut unique_paths = 0;

    while let Some((from, mut n)) = queue.pop_front() {
        queue.retain(|&path| {
            let retain = path.0 != from;
            if !retain {  n += path.1; }
            retain
        });

        for &option in graph.get(from).unwrap_or(&vec![]) {

            if option == to {
                unique_paths += n;
                continue;
            }

            queue.push_back((option, n));
        }
    }

    unique_paths
}