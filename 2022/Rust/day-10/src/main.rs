fn main() {
    let input = include_str!("../../../input/10");
    println!("Sum: {:?}", run(input));
}

fn run(input: &str) -> i32 {
    fn draw(tick: i32, reg: i32, crt: &mut [char; 40 * 6]) {
        let tmod = (tick - 1) % 40;
        if reg == tmod 
            || (reg + 1) / 40 == reg / 40 && reg + 1 == tmod 
            || (reg - 1) / 40 == reg / 40 && reg - 1  == tmod
        {
            crt[(tick - 1) as usize] = '#';
        } 
    }

    fn add_reg(t: i32) -> bool {
        matches!(t, 20 | 60 | 100 | 140 | 180 | 220)
    }

    let mut crt = ['.'; 40 * 6];
    let mut tick = 1;
    let mut buf = 0;
    let mut register = 1;
    let mut sum = 0;

    for line in input.lines() {
        let mut add = false; // flag to avoid skipping summing during addition cycles
        let mut ticked_val = 0;
        register += buf;
        buf = 0;

        draw(tick, register, &mut crt);
        let mut splits = line.split_whitespace();
        match splits.next().unwrap() {
            "noop" => (),
            "addx" => { 
                buf = splits.next().unwrap().parse().unwrap();
                add = add_reg(tick);
                if add { ticked_val = tick; } 
                tick += 1;
                
                draw(tick, register, &mut crt);
            },
            _ => panic!("Unrecognised instruction")
        };

        if add || add_reg(tick) {
            sum += register * if add {ticked_val} else {tick};
        }
        tick += 1;
    }

    for row in 0..6 {
        for col in 0..40 {
            print!("{}", crt[col + (row * 40)]);
        }
        println!();
    }
    sum
}