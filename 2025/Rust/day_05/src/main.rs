fn main() {
    let (ranges, ingredients) = include_str!("../../../../input/2025/05")
        .split_once("\n\n")
        .expect("Failed to split input");


    let ranges = ranges.lines()
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
