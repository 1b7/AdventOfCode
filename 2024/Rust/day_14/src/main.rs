use bmp::{Image, Pixel};

/// Note that the result for Part 2 is found by generating and visually inspecting
/// a series of bitmap images.
fn main() {
    let robots: Vec<_> = include_str!("../../../input/14")
        .lines()
        .map(|line| {
            let (p, v) = line.split_once(" ").unwrap();
            let (p, v) = (&p[2..], &v[2..]);

            let (px, py) = p.split_once(',').unwrap();
            let (vx, vy) = v.split_once(',').unwrap();

            ((px.parse::<i32>().unwrap(), py.parse::<i32>().unwrap()),
             (vx.parse::<i32>().unwrap(), vy.parse::<i32>().unwrap()))
        }).collect();

    println!("Part 1: {}", p1(&robots));
    // p2(&robots);
}

fn p2(robots: &[((i32, i32), (i32, i32))]) {
    let w = 101;
    let h = 103;

    // Pattern observed from scanning all iterations, the tree tends to converge over a series of
    // iterations spaced `w` steps apart, starting from the 95th iteration.
    // Consequently, we just render these images of interest.
    for t in (95..20000).step_by(w as usize) {
        let mut img = Image::new(w as u32, h as u32);

        for ((x, y), (vx, vy)) in robots.iter() {
            let nx = (x + t * vx).rem_euclid(w);
            let ny = (y + t * vy).rem_euclid(h);
            img.set_pixel(nx as u32, ny as u32, Pixel::new(0, 255, 0));
        }

        let _ = img.save(format!("./img/{:06}_frame.bmp", t));
    }
}

fn p1(robots: &[((i32, i32), (i32, i32))]) -> usize {
    let t = 100;
    let w = 101;
    let h = 103;

    // Treat each robot as a series of line equations, using
    // Euclidean division to simulate wrapping behaviour.
    let mut quadrants = [0; 4];
    for ((x, y), (vx, vy)) in robots.iter() {
        let nx = (x + t * vx).rem_euclid(w);
        let ny = (y + t * vy).rem_euclid(h);

        if nx > (w - 1) / 2 && ny > (h - 1) / 2 {
            quadrants[3] += 1;
        } else if nx > (w - 1) / 2 && ny < (h - 1) / 2 {
            quadrants[2] += 1;
        } else if nx < (w - 1) / 2 && ny > (h - 1) / 2 {
            quadrants[1] += 1;
        } else if nx < (w - 1) / 2 && ny < (h - 1) / 2 {
            quadrants[0] += 1;
        }
    }

    quadrants.iter().product()
}
