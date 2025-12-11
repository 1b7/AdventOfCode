use std::collections::VecDeque;

const WIDTH: usize = 15;

fn main() {
    let rows = include_str!("../../../../input/2025/10")
        .lines()
        .map(parse_line)
        .collect::<Vec<_>>();

    println!("Part 1: {}", rows.iter().map(|row| resolve_state(row.0, &row.1)).sum::<usize>());
}

fn resolve_state(target: u16, buttons: &[Vec<usize>]) -> usize {
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

fn parse_line(line: &str) -> (u16, Vec<Vec<usize>>, Vec<usize>) {
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