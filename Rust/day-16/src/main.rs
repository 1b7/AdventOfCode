use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("../../../input/16");
    let graph = process_graph(&get_graph(input));

    let p1 = travel(&graph, 1, 30);
    // let p2 = travel(&graph, 2, 26);
    println!("Part 1: {}", p1);
    // println!("Part 2: {}", p2);
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
 
fn process_graph<'a> (g: &HashMap<&'a str, (usize, Vec<&'a str>)>) -> HashMap<&'a str, (usize, Vec<(&'a str, u8)>)> {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum State { Free, Moving(u8) }
impl State {
    pub fn tick(self) -> State {
        match self {
            Self::Free => Self::Free,
            Self::Moving(1) => Self::Free,
            Self::Moving(x) => Self::Moving(x - 1)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Traveller {
    node: String,
    path: Vec<String>,
    state: State,
    opening_valve: Option<String>
}

impl Traveller {
    pub fn new(node: String, path: Vec<String>, state: State) -> Self { 
        Traveller { node, path, state, opening_valve: None } 
    } 
    // pub fn tick(&mut self) -> State { self.state = self.state.tick(); self.state }
}

fn travel(graph: &HashMap<&str, (usize, Vec<(&str, u8)>)>, actors: usize, time: usize) -> usize {
    let template = Traveller::new("AA".to_string(), vec!["AA".to_string()], State::Free);
    let travellers = vec![template; actors];

    let mut states: VecDeque<_> = VecDeque::new();
    states.push_back((travellers, HashSet::<String>::new(), time, 0));

    let mut pressure_max = 0;

    // Process a State:
    while !states.is_empty() {
        let (mut travellers, mut opened_valves, mut time_left, mut pressure_lifted) = states.pop_front().unwrap();
        if time_left == 0 { continue }

        // Turning valves will now be open:
        for actor in &mut travellers {
            if let Some(valve) = &actor.opening_valve {
                opened_valves.insert(valve.to_owned());
                actor.opening_valve = None;
            }
        }

        // Include Pressure Lifted:
        pressure_lifted += opened_valves.iter()
            .map(|valve| graph.get(valve.as_str()).unwrap().0)
            .sum::<usize>();
        pressure_max = pressure_max.max(pressure_lifted);
        time_left -= 1;

        let mut outgoing_travellers: Vec<Vec<Traveller>> = vec![vec![]; 2];

        // 'Act' for each traveller:
        for (n, traveller) in travellers.iter().enumerate() {
            let new_state = traveller.state.tick();
            match new_state {
                State::Moving(_) => {
                    let mut new_traveller = traveller.clone();
                    new_traveller.state = new_traveller.state.tick();
                    outgoing_travellers[n].push(new_traveller);
                },
                State::Free => {
                    let pos = &traveller.node;
                    if !opened_valves.contains(pos) && pos != "AA" {
                        let mut new_traveller = traveller.clone();
                        new_traveller.state = State::Moving(1);
                        new_traveller.opening_valve = Some(pos.to_string());
                        outgoing_travellers[n].push(new_traveller);
                    } else {
                        let (_, options) = graph.get(traveller.node.as_str()).unwrap();

                        // Only consider travelling to unopened valves:
                        let options = options.into_iter().filter(|(node, dist)| 
                            !opened_valves.contains(*node)
                            && *dist as usize <= time_left
                        );

                        // Create new state, pass it forward:
                        for option in options {
                            let mut new_traveller = traveller.clone();
                            new_traveller.state = State::Moving(option.1);
                            new_traveller.node = option.0.to_string();
                            new_traveller.path.push(option.0.to_string());
                            outgoing_travellers[n].push(new_traveller);
                        }
                    }
                }
            };
        }

        let remaining =  opened_valves.iter()
            .map(|valve| graph.get(valve.as_str()).unwrap().0)
            .sum::<usize>() * time_left as usize;
        pressure_max = pressure_max.max(pressure_lifted + remaining);

        if actors == 2 {
            // Tes all possible combinations of choices for each actor:
            for t_a in outgoing_travellers[0].iter() {
                for t_b in outgoing_travellers[1].iter() {
                    states.push_back((
                        vec![t_a.clone()], //, t_b.clone()], 
                        opened_valves.clone(), 
                        time_left, 
                        pressure_lifted
                    ));
                }
            }
        } else {
            for t_a in outgoing_travellers[0].iter() {
                states.push_back((
                    vec![t_a.clone()], //, t_b.clone()], 
                    opened_valves.clone(), 
                    time_left, 
                    pressure_lifted
                ));
            }
        }
    }
    pressure_max
}