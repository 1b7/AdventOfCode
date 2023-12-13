fn main() {
    let maps = include_str!("../../../input/13")
        .split("\n\n")
        .map(map_to_tiles)
        .collect::<Vec<_>>();

    let lines = maps.iter().map(|map| find_mirror(&map, None)).collect::<Vec<_>>();
    let p1 = lines.iter().sum::<usize>();
    println!("Part 1: {p1}");

    let p2 = maps.iter().zip(lines).map(|(m, s)| brute_force(m, s)).sum::<usize>();
    println!("Part 2: {p2}");
}

fn map_to_tiles(s: &str) -> Vec<Vec<Tile>> {
    s.lines().map(|row| row.chars()
        .map(|c| match c {
            '.' => Tile::Ash,
            '#' => Tile::Rock,
            _  => unreachable!()
        }).collect()
    ).collect()
}

fn find_mirror(map: &Vec<Vec<Tile>>, ignore: Option<usize>) -> usize {
    for row in 1..(map.len() as i32) {
        let (mut down, mut up) = (row - 1, row);
        let mut is_match = true;
        
        while down >= 0 && up < map.len() as i32 {
            if map[down as usize] != map[up as usize] {
                is_match = false;
                break;
            }
            down -= 1;
            up += 1;
        }

        let skip = ignore.is_some() && ignore.unwrap() == row as usize * 100;
        if is_match && !skip { return row as usize * 100 }
    };

    for col in 1..(map[0].len() as i32) {
        let mut is_match = true;
        let (mut left, mut right) = (col - 1, col);
        
        while left >= 0 && right < map[0].len() as i32 {
            if !map.iter().all(|row| row[left as usize] == row[right as usize]) {
                is_match = false;
                break;
            }
            left -= 1;
            right += 1;
        }

        let skip = ignore.is_some() && ignore.unwrap() == col as usize;
        if is_match && !skip { return col as usize; }
    };
    0
}

fn brute_force(map: &Vec<Vec<Tile>>, s: usize) -> usize {
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            let mut alt = map.clone();
            alt[r][c].invert();
            let val = find_mirror(&alt, Some(s));
            if val != 0 && val != s { return val }
        }
    }
    unreachable!()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile { Ash, Rock }

impl Tile {
    pub fn invert(&mut self) {
        match self {
            Tile::Ash => { *self = Tile::Rock},
            Tile::Rock => { *self = Tile::Ash},
        }
    }
}