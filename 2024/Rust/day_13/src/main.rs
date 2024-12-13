#[derive(Debug, Clone, Copy)]
struct Point { x: f64, y: f64 }

fn main() {
    let equations = include_str!("../../../input/14")
        .split("\r\n\r\n")
        .map(|block|
            block.lines()
                .filter(|line| !line.is_empty())
                .map(|l| {
                    let (x, y) = l.split_once(':').unwrap().1.split_once(',').unwrap();
                    Point { x: x[3..].parse::<f64>().unwrap(), y: y[3..].parse::<f64>().unwrap() }
                }).collect::<Vec<_>>()
        ).collect::<Vec<_>>();

    println!("Part 1: {}", equations.iter()
        .map(|eqn| solve(eqn[0], eqn[1], eqn[2]))
        .sum::<f64>()
    );

    let err = 10000000000000.0;
    println!("Part 2: {}", equations.iter()
        .map(|eqn|
            solve(eqn[0], eqn[1], Point { x: eqn[2].x + err, y: eqn[2].y + err })
        ).sum::<f64>()
    );
}


// Using Cramer's rule to solve the system of linear equations.
fn solve(a: Point, b: Point, p: Point) -> f64 {
    let det = a.x * b.y - a.y * b.x;
    let m = b.y * p.x - b.x * p.y;
    let n = a.x * p.y - a.y * p.x;
    if m % det == 0. && n % det == 0. { ((3. * m + n) / det).floor() } else { 0. }
}