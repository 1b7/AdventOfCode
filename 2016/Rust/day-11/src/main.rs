use std::{collections::{HashSet, BinaryHeap}, cmp::Reverse};

const TOP_FLOOR: usize = 3;

fn main() {
    let input = load_input();
    let mut floors: Vec<_> = input.lines().map(parse_line).collect();
    println!("Part 1: {}", search(floors.clone()));

    floors[0].extend_from_slice(
        &[Component::Gen(0), Component::Chip(0), Component::Gen(1), Component::Chip(1)]
    );
    println!("Part 2: {}", search(floors));
}

fn search(init: Vec<Vec<Component>>) -> usize {
    fn heuristic(moves: u16, floors: &[Vec<Component>]) -> u16 {
        moves + floors.iter().enumerate().map(|(i, floor)| {
            ((floors.len() - i) * floor.len()) as u16
        }).sum::<u16>()
    }
    
    fn is_safe(v: Component, floor: &[Component]) -> bool {
        match v {
            // Chip cannot be moved to floor with another RTG, if its own RTG is not there.
            Component::Chip(x) => floor.iter().all(|c| !c.is_gen())
                || floor.iter().any(|c| c.is_gen() && c.val() == x),
            // RTG cannot be moved to floor where other chip exists and is not
            // connected to its corresponding RTG.
            Component::Gen(x) => {
                for a in floor {
                    if !a.is_gen() && a.val() != x {
                        for b in floor {
                            if b.is_gen() && b.val() == a.val() { return true; }
                        }
                    }
                }
                false
            }
        }
    }

    fn expand(lift: usize, state: &[Vec<Component>]) -> Vec<(usize, Vec<Vec<Component>>)> {
        let mut combs = vec![];
        for c in 0..(&state[lift]).len() {
            for d in 0..(&state[lift]).len() {
                if !combs.contains(&(d, c)) { combs.push((c, d)); }
            }
        }

        let mut new_states = vec![];
        let not_low_floor = lift > 0;
        let not_top_floor = lift < TOP_FLOOR;

        for (m, n) in combs {
            let single_item = m == n;
            let safe_up_m = not_top_floor && is_safe(state[lift][m], &state[lift + 1]);
            let safe_dn_m = not_low_floor && is_safe(state[lift][m], &state[lift - 1]);

            // Avoid moving things back down to empty floors.
            let low_is_empty = lift == 0 || state[lift - 1].len() == 0;

            let mut append = |new_lift: usize| {
                let mut new = state.to_vec();
                let (m, n) = (m.min(n), m.max(n));

                new[lift].remove(n);
                if !single_item {
                    new[lift].remove(m);
                    new[new_lift].push(state[lift][m]);
                }

                new[new_lift].push(state[lift][n]);
                new_states.push((new_lift, new));
            };

            if single_item {
                if safe_up_m { append(lift + 1); }
                if !low_is_empty && safe_dn_m { append(lift - 1); }
            } else {
                let safe_up_n = not_top_floor && is_safe(state[lift][n], &state[lift + 1]);
                let safe_dn_n = not_low_floor && is_safe(state[lift][n], &state[lift - 1]);
                let same_type = state[lift][m].val() == state[lift][n].val();
                if (not_top_floor && same_type) || (safe_up_m && safe_up_n) { 
                    append(lift + 1); 
                }
                if !low_is_empty && ((not_low_floor && same_type) || (safe_dn_m && safe_dn_n)) { 
                    append(lift - 1); 
                }
            }
        }
        new_states
    }

    let total_components = init.iter().map(|floor| floor.len()).sum();
    let mut seen = HashSet::new();
    seen.insert((0, hash(&init)));

    let mut frontier = BinaryHeap::new();
    frontier.push((Reverse(heuristic(0, &init)), Reverse(0), 0, init));

    while !frontier.is_empty() {
        let (_h, steps, lift, state) = frontier.pop().unwrap();
        if state[TOP_FLOOR].len() == total_components {
            return steps.0 as usize 
        }

        for (nlift, nstate) in expand(lift, &state) {
            if !seen.contains(&(nlift, hash(&nstate))) {
                seen.insert((nlift, hash(&nstate)));
                let heuristic = Reverse(heuristic(steps.0 + 1, &nstate));
                frontier.push((heuristic, Reverse(steps.0 + 1), nlift, nstate));
            }
        }
    }

    panic!("No Solution Found");
}

// Hashes a state as some integer - does not care about the element of components,
// just the number of generators/chips on each floor, because type doesn't matter
// when it comes to pruning states.
fn hash(state: &[Vec<Component>]) -> usize {
    fn hash_floor(f: &[Component]) -> usize {
        f.iter().map(|c| if c.is_gen() { 1 } else { 1000 } ).sum()
    }
    state.iter().enumerate()
        .fold(0, |acc, (i, floor)| acc + (hash_floor(&floor) * i * i))
}

fn parse_line(s: &str) -> Vec<Component> {
    if s.contains("nothing relevant") { return vec![] }
    let (_, s) = s.split_once("contains ").unwrap();
    let chunks = s.split("a ");

    let hash = |st: &str| st.chars().take(3).enumerate()
        .fold(1, |acc, (i, c)| acc + ((i + 1) as u16) * (c as u16) );

    chunks.filter(|&s| s != "").map(|chunk| {
        if chunk.contains("generator") {
            Component::Gen((hash(
                chunk.split_whitespace().next().unwrap()
            ) / 3) as u8)
        } else {
            Component::Chip((hash(
                chunk.split('-').next().unwrap()
            ) / 3) as u8)
        }
    }).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Component { Gen(u8), Chip(u8) }
impl Component {
    pub fn val(self) -> u8 {
        match self {
            Component::Gen(x) => x,
            Component::Chip(x) => x
        }
    }

    pub fn is_gen(self) -> bool {
        match self {
            Component::Gen(_) => true,
            _ => false
        }
    }
}

fn load_input() -> String {
    std::fs::read_to_string(
        std::env::args().nth(1).expect("Filepath not provided.")
    ).expect("Could not load file!")
}