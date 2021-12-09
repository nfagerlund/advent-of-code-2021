use advent21::*;

fn main() {
    part_one()
}

fn part_one() {
    let inputs = load_inputs("day2").unwrap();
    // OK, we have lines, we need to split them on spaces, then we need to switch
    // on commands, and we need to keep two running totals. Easy peasy.
    let mut x_pos = 0;
    let mut depth = 0;
    for line in inputs.lines() {
        // Once again, I'm very willing to panic in the disco.
        let (command, units) = line.split_once(' ').unwrap();
        let units = i32::from_str_radix(units, 10).unwrap();
        match command {
            "up" => depth -= units,
            "down" => depth += units,
            "forward" => x_pos += units,
            _ => panic!("unrecognized command: {}", command),
        }
    }
    println!("Horizontal position: {}", x_pos);
    println!("Depth: {}", depth);
    println!("Multiplied: {}", x_pos * depth);
}
