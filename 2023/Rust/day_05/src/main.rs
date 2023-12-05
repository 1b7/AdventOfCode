fn main() {
    let mut blocks = include_str!("../../../input/05").split("\n\n");

    let seeds: Vec<usize> = blocks.next().unwrap()
        .split_whitespace().skip(1)
        .map(|n| n.parse().unwrap())
        .collect();

    let mut maps = vec![];
    for block in blocks {
        let mut map: Vec<(usize, usize, usize)> = vec![];
        for line in block.lines().skip(1) {
            let values = line.split_whitespace().map(|n| n.parse().unwrap()).collect::<Vec<_>>();
            map.push((values[0], values[1], values[2]));
        }
        map.sort_by(|a, b| a.1.cmp(&b.1));
        maps.push(map);
    }

    let mut items = seeds.clone();

    process(&mut items, &maps);
    let p1 = items.iter().min().unwrap();
    dbg!(p1);

    let mut items = seeds.chunks(2)
        .flat_map(|item| (item[0]..(item[0] + item[1])))
        .collect::<Vec<_>>();

    process(&mut items, &maps);
    let p2 = items.iter().min().unwrap();
    dbg!(p2);
}

fn process(items: &mut [usize], maps: &[Vec<(usize, usize, usize)>]) {
    for i in 0..items.len() {
        for map in maps {
            for row in map {
                let &(dest, source, range) = row;
                if items[i] >= source && items[i] < (source + range) {
                    items[i] = items[i] + dest - source;
                    break
                }
            }
        }
    }
}