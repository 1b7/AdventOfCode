use std::collections::{HashMap, HashSet};

fn main() {
    let (rules, pages) = include_str!("../../../input/05")
        .split_once("\n\n")
        .expect("Error splitting input into rules and pages");

    let mut precedence = HashMap::new();

    rules.lines().for_each(|line| {
        let nums = line.split('|')
            .map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        precedence.entry(nums[0]).or_insert(HashSet::new())
            .insert(nums[1]);
    });

    let pages: Vec<Vec<i32>> = pages.lines()
        .map(|page| page.split(',').map(|x| x.parse().unwrap()).collect::<Vec<i32>>())
        .collect();

    let (correct_rule_sum, broken_rules) = identify_broken_rules(pages, &precedence);
    println!("Part 1: {correct_rule_sum}");

    let p2: i32 = broken_rules.iter().map(|rule| fix_rule(&rule, &mut precedence.clone())).sum();
    println!("Part 2: {p2}");
}

// Identify pages with broken rules, according to the precedence graph.
fn identify_broken_rules(pages: Vec<Vec<i32>>, precedence: &HashMap<i32, HashSet<i32>>) -> (i32, Vec<Vec<i32>>) {
    let mut correct_rule_sum = 0;
    let mut broken_rules = vec![];
    for page in pages {
        let mut seen = HashSet::new();

        let mut is_broken = false;
        for &n in &page {
            seen.insert(n);

            if let Some(preceding) = precedence.get(&n) {
                let intersection: Vec<_> = preceding.intersection(&seen).collect();
                if intersection.len() != 0 {
                    is_broken = true;
                    break;
                }
            }
        }

        if !is_broken {
            correct_rule_sum += page[page.len() / 2]
        } else {
            broken_rules.push(page);
        }
    }
    (correct_rule_sum, broken_rules)
}

// Fix page ordering by applying Kahn's algorithm (topological sorting).
// This is a modified version to account for the fact that we only care about edges
// relating to elements of the current 'page'.
fn fix_rule(page: &[i32], precedence: &mut HashMap<i32, HashSet<i32>>) -> i32 {
    // Utility function; determines whether a given node has any incoming edges
    // (from the current page) remaining.
    fn has_incoming_edges(n: i32, nodes: &HashSet<i32>, precedence: &HashMap<i32, HashSet<i32>>) -> bool {
        for &key in nodes {
            if key == n { continue; }
            if let Some(entry) = precedence.get(&key) {
                if entry.contains(&n) { return true; }
            }
        }
        false
    };

    let ps: HashSet<_> = page.iter().map(|&x| x).collect();
    let mut free_nodes: HashSet<i32> = HashSet::from_iter(
        page.iter().filter(|&&x| !has_incoming_edges(x, &ps, &precedence)).map(|&x| x)
    );

    let mut remaining: HashSet<i32> =  HashSet::from_iter(
        page.iter().filter(|x| !free_nodes.contains(x)).map(|&x| x)
    );

    let mut sorted_page: Vec<i32> = vec![];

    // Since we only care about the middle page, we can exit when this is found.
    let (mut i, sort_iters) = (0, page.len() / 2);
    while free_nodes.len() > 0 && i <= sort_iters  {
        let selection = *free_nodes.iter().next().unwrap();
        free_nodes.remove(&selection);

        sorted_page.push(selection);
        precedence.remove(&selection);

        let mut new_remaining = HashSet::new();
        for &r in &remaining {
            if !has_incoming_edges(r, &remaining, &precedence) {
                free_nodes.insert(r);
            } else {
                new_remaining.insert(r);
            }
        }
        remaining = new_remaining;
        i += 1;
    }

    sorted_page[sort_iters]
}
