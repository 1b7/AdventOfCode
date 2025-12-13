use std::collections::VecDeque;

use z3;
use z3::{Solver};
use z3::ast::{Int};

const WIDTH: usize = 15;

fn main() {
    let rows = include_str!("../../../../input/2025/10")
        .lines()
        .map(parse_line)
        .collect::<Vec<_>>();

    println!("Part 1: {}", rows.iter().map(|row| resolve_state(row.0, &row.1)).sum::<u64>());

    let part_2: u64 = rows.iter()
        .map(|row| solve_joltage(&row.2, &row.1))
        .sum();

    println!("Part 2: {part_2}");
}

fn resolve_state(target: u16, buttons: &[Vec<usize>]) -> u64 {
    let initial_state = 0;

    let mut queue = VecDeque::from([(initial_state, 0)]);

    let buttons = buttons.iter().map(|btn| {
        btn.iter().fold(0, |acc, &btn| acc | 1_u16 << (WIDTH - btn))
    }).collect::<Vec<_>>();

    while let Some((state, presses)) = queue.pop_front() {
        if state == target { return presses }

        buttons.iter().for_each(|button| {
            queue.push_back((state ^ button, presses + 1));
        });
    }

    unreachable!()
}

fn solve_joltage(target: &[u64], buttons: &[Vec<usize>]) -> u64 {
    let mut connections = vec![vec![]; target.len()];
    let mut consts = vec![];
    let solver = Solver::new();

    // Define our constants -- these are our 'buttons'.
    let prefixes: Vec<_>  = (0..buttons.len()).map(|i| format!("btn_{i}")).collect();
    for b in 0..buttons.len() {
        consts.push(Int::fresh_const(&prefixes[b]));
    }

    // For each button, assert that it is pressed 0 or more times, additionally store which machines
    // this button is connected to.
    for b in 0..buttons.len() {
        let btn_i = &consts[b];

        solver.assert(btn_i.ge(0));

        for &connection in &buttons[b] {
            connections[connection].push(btn_i);
        }
    }

    // Iteratively declare assertions for each machine that the sum of presses of its connected buttons
    // must equal the target joltage of that machine.
    for i in 0..connections.len() {
        solver.assert(Int::add(&connections[i]).eq(&Int::from_u64(target[i])));
    }

    // Run the solver and take the solution with minimal presses.
    solver.solutions(&consts, false)
        .map(|solve| solve.iter().fold(0, |acc, x| acc + x.as_u64().unwrap()))
        .min()
        .unwrap()
}

fn parse_line(line: &str) -> (u16, Vec<Vec<usize>>, Vec<u64>) {
    let chunks: Vec<_> = line.split_whitespace().collect();

    let desired_state = chunks[0]
        .chars()
        .filter(|&c| c == '#' || c == '.')
        .map(|c| c == '#')
        .enumerate()
        .fold(0, |acc, (i, b)| if b { acc | (1 << WIDTH - i) } else { acc });

    let buttons = chunks[1..(chunks.len() - 1)]
        .iter()
        .map(|&btn| btn.trim_matches(['(', ')']).split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    let joltages = chunks[chunks.len() - 1]
        .trim_matches(['{', '}'])
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    (desired_state, buttons, joltages)
}