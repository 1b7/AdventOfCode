use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("../../../input/16");
    let p1 = p1(input);
    println!("Part 1: {}", p1);
}

fn p1(s: &str) -> usize {
    let graph = get_graph(s);
    let mut paths = VecDeque::new();
    paths.push_front((vec!["AA".to_string()], HashSet::new(), 30, 0));
    let result = traverse(&mut paths, &graph);
    result
}

fn get_graph(s: &str) -> HashMap<&str, (usize, Vec<&str>)> {
    let mut map = HashMap::new();
    for line in s.lines() {
        let split: Vec<_> = line.split_whitespace().collect();
        let (label, rate, leads_to) = (split[1], split[4], &split[9..]);
        let rate: usize = rate.chars().filter(char::is_ascii_digit).collect::<String>().parse().unwrap();
        let leads_to: Vec<_> = leads_to.iter().map(|s| s.trim_end_matches(',')).collect();

        map.insert(label, (rate, leads_to));
    }
    map
}

fn bfs(g: &HashMap<&str, (usize, Vec<&str>)>, from: &str, to: &str) -> u8 {
    let mut paths = VecDeque::new();
    paths.push_front(vec![from]);
    while !paths.is_empty() {
        let path = paths.pop_front().unwrap();
        let tail = path[path.len() - 1];

        if tail == to { return (path.len() - 1) as u8 }

        let (_, neighbours) = g.get(tail).unwrap();
        for &neighbour in neighbours {
            let mut new_path = path.clone();
            new_path.push(neighbour);
            paths.push_back(new_path);
        }
    }
    0
}
 
fn process_graph <'a> (g: &HashMap<&'a str, (usize, Vec<&'a str>)>) -> HashMap<&'a str, (usize, Vec<(&'a str, u8)>)> {
    let keys: Vec<_> = g.clone().into_keys().collect();
    let mut dists: HashMap<&str, (usize, Vec<(&str, u8)>)> = HashMap::new();

    for k in keys.clone() {
        let copy = g.clone();
        let (_, (flow, _)) = copy.get_key_value(k).unwrap();
        if *flow == 0 && k != "AA"  { continue }
        dists.insert(k, (*flow, vec![]));

        for l in keys.clone() {
            let (flow, _) = &g.get(l).unwrap();
            if k == l || *flow == 0 { continue; }
            let dist = bfs(g, k, l);
            dists.get_mut(k).unwrap().1.push((l, dist));
        }
    }
    dists
}

fn traverse(
    paths: &mut VecDeque<(Vec<String>, HashSet<String>, usize, usize)>, 
    graph: &HashMap<&str, (usize, Vec<&str>)>
) -> usize {
    let mut max = 0;
    let mut max_path = None;

    let graph = process_graph(graph);

    while !paths.is_empty() {
        let front =  paths.pop_front();
        if front.is_none() { return 0 }
        let (path, open_valves, time_left, pressure_lifted) = front.unwrap();
    
        if time_left <= 0 { max = max.max(pressure_lifted); continue; } 
    
        let tail = &path[path.len() - 1];
        let &(_, options) = &graph.get(tail.as_str()).unwrap();
    
        options.iter().for_each(|&(option, distance)| {
            let distance = distance as usize;
            let option = String::from(option);

            let mut new_path = path.clone();
            new_path.push(option.to_string());

            if !open_valves.contains(&option) && time_left > distance {
                // Include pressure lifted during travel:
                let ppm = open_valves.iter().map(|valve| {
                    graph.get(valve.as_str()).unwrap().0
                }).sum::<usize>();

                // Opening a valve takes one minute:
                let mut new_open_valves = open_valves.clone();
                new_open_valves.insert(option);

                paths.push_back((
                    new_path.clone(),
                    new_open_valves, 
                    time_left - (distance + 1), 
                    pressure_lifted + (ppm * (distance + 1))
                ));

            } else {
                // Calculate current pressure lifted:
                let tmp_pressure = open_valves.iter().map(|valve| {
                    graph.get(valve.as_str()).unwrap().0
                }).sum::<usize>();

                let pressure_over_time = tmp_pressure * (time_left);
                let relieved = pressure_lifted + pressure_over_time;

                if relieved > max { 
                    max = relieved;
                    max_path = Some(path.to_owned());
                }
            }
        });
    }
    dbg!(max_path);
    max
}

// 2155 too high
