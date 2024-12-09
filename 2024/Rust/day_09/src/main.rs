use std::iter;

fn main() {
    let input = include_str!("../../../input/09")
        .chars().into_iter().collect::<Vec<_>>();

    let mut fs: Vec<Option<usize>> = vec![];
    for (id, data) in input.chunks(2).enumerate() {
        let occupied = data[0].to_digit(10).unwrap() as usize;
        fs.extend(iter::repeat_n(Some(id), occupied));

        if data.len() > 1 {
            let free_space = data[1].to_digit(10).unwrap() as usize;
            fs.extend(iter::repeat_n(None, free_space));
        }
    }

    println!("Part 1: {:#?}", p1(&mut fs.clone()));
    println!("Part 2: {:#?}", p2(&mut fs.clone()));
}

/// Rearrange the disk blocks by moving each block from the
/// end of the disk as far forward as possible.
fn p1(fs: &mut [Option<usize>]) -> usize {
    let mut free_space = 0;
    let mut final_element = fs.len() - 1;
    while free_space < final_element {
        while fs[free_space].is_some() { free_space += 1; }
        while fs[final_element].is_none() { final_element -= 1; }

        fs[free_space] = fs[final_element];
        fs[final_element] = None;
    }

    fs[final_element] = fs[free_space];
    fs[free_space] = None;

    checksum(&fs)
}


/// Rearrange whole files by moving all of their blocks from the
/// end of the disk as far forward as possible.
/// If a file cannot completely fit into a region, it will not be moved at all.
fn p2(fs: &mut [Option<usize>]) -> usize {
    let mut cursor = fs.len() - 1;
    let mut id = fs[cursor].unwrap();

    while id > 0 {
        // Find the file and navigate to its start:
        while fs[cursor] != Some(id) { cursor -= 1; }
        let mut size = cursor;
        while fs[cursor - 1] == Some(id) { cursor -= 1; }
        size = 1 + size - cursor;

        // Try to find a block in which it can fit:
        let mut free_start = 0;
        let mut free_end = 0;
        while free_start < cursor {
            while fs[free_start].is_some() { free_start += 1; }

            // Don't scan past the cursor:
            if free_start >= cursor { free_end = free_start; break; }

            free_end = free_start + 1;
            while fs[free_end].is_none() { free_end += 1; }

            if  1 + free_end - free_start > size { break; }
            else { free_start = free_end + 1 }
        }
        let mut free_size = 1 + free_end - free_start;

        // Write the block to the leftmost free region, if it is sufficiently large:
        if free_end < fs.len() && free_size > size {
            for i in 0..size {
                fs[free_start + i] = fs[cursor + i];
                fs[cursor + i] = None;
            }
        }

        // Skip free space:
        cursor -= 1;
        while fs[cursor] == None { cursor -= 1; }

        // Move on to the next file:
        id -= 1;
    }

    checksum(&fs)
}

fn checksum(fs: &[Option<usize>]) -> usize {
    fs.iter().enumerate()
        .map(|(a, x)| {
            if x.is_some() { x.unwrap()  * a } else { 0 }
        }).sum()
}