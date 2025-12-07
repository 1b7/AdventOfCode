use std::collections::{HashMap, HashSet};

type Point = (i64, i64);

fn main() {
    let mut start = (0, 0);
    let mut splitters = HashSet::new();

    include_str!("../../../../input/2025/07")
        .lines()
        .enumerate()
        .for_each(|(row, line)|
            line.chars().enumerate().for_each(|(col, chr)| {
                match chr {
                    'S' => { start = (row as i64, col as i64); },
                    '^' => { splitters.insert((row as i64, col as i64)); },
                    _ => ()
                };
            })
        );

    let (p1, p2) = calculate_splits(start, &splitters);
    println!("Part One: {p1}");
    println!("Part Two: {p2}");
}

fn calculate_splits(start: Point, splitters: &HashSet<Point>) -> (usize, usize) {
    let mut beams = HashSet::from([start]);
    let mut beam_counts = HashMap::from([(start, 1)]);

    let max_row = splitters.iter().max_by_key(|splitter| splitter.0).unwrap().0;

    let mut split_count = 0;

    for _ in 0..=max_row {
        let mut new_beams = HashSet::new();

        for beam in beams {
            let next_pos = (beam.0 + 1, beam.1);
            let splits_so_far = *beam_counts.get(&beam).unwrap_or(&1);

            let mut update_beam = |point| {
                new_beams.insert(point);
                beam_counts.entry(point).and_modify(|x| *x += splits_so_far ).or_insert(splits_so_far);
            };

            if splitters.contains(&next_pos) {
                split_count += 1;
                update_beam((beam.0 + 1, beam.1 - 1));
                update_beam((beam.0 + 1, beam.1 + 1));
            } else {
                update_beam(next_pos);
            }
        }

        beams = new_beams;
    }

    (split_count, beams.into_iter().map(|beam| beam_counts.get(&beam).unwrap_or(&0)).sum())
}