use std::collections::HashSet;

fn main() {
    let (ranges, ingredients) = include_str!("../../../../input/2025/05")
        .split_once("\n\n")
        .expect("Failed to split input");


    let mut ranges = ranges.lines()
        .map(|range| {
            let (low, high) = range.split_once('-').expect("Failed to split range");
            (
                low.parse::<usize>().expect("Failed to parse low side of range"),
                high.parse::<usize>().expect("Failed to parse high side of range")
            )
        })
        .collect::<Vec<_>>();

    let ingredients = ingredients.lines()
        .map(|ingredient| ingredient.parse::<usize>().expect("Failed to parse ingredient"))
        .collect::<Vec<_>>();

    println!("Part 1: {}", test_ingredients(&ingredients, &ranges));
    println!("Part 2: {}", list_fresh_ingredients(&mut ranges));
}

fn test_ingredients(ingredients: &[usize], ranges: &[(usize, usize)]) -> usize {
    let mut fresh_ingredients = 0;

    for &ingredient in ingredients {
        for &range in ranges {
            if ingredient >= range.0 && ingredient <= range.1 {
                fresh_ingredients += 1;
                break;
            }
        }
    }

    fresh_ingredients
}

fn list_fresh_ingredients(ranges: &mut [(usize, usize)]) -> usize {
    for i in 0..ranges.len() {
        for j in 0..ranges.len() {
            if i == j { continue }

            // Subset range
            if ranges[i].0 <= ranges[j].0  && ranges[i].1 >= ranges[j].1 {
                ranges[j] = ranges[i];
                continue;
            }

            // Overlap on Low Side
            if ranges[i].0 < ranges[j].0 && ranges[i].1 >= ranges[j].0 {
                ranges[j].0 = ranges[i].0;
            }

            // Overlap on High Side
            if ranges[i].1 > ranges[j].1 && ranges[i].0 <= ranges[j].1 {
                ranges[j].1 = ranges[i].1;
            }
        }
    }

    ranges.iter()
        .collect::<HashSet<_>>() // Drop repeat ranges
        .iter()
        .map(|&(low, high)| high - low + 1)
        .sum()
}