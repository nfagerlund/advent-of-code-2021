use advent21::*;

fn main() {
    let inputs = load_inputs("day2").unwrap();
    part_two(inputs)
}

fn part_two(inputs: String) {
    // Okay, this one's also pretty simple.
    // Same commands, but up/down mutate "aim."
    // forward mutates both x_pos (units * 1) AND depth (units * aim)
    let mut x_pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in inputs.lines() {
        // Once again, I'm very willing to panic in the disco.
        let (command, units) = line.split_once(' ').unwrap();
        let units = i32::from_str_radix(units, 10).unwrap();
        match command {
            "up" => aim -= units,
            "down" => aim += units,
            "forward" => {
                x_pos += units;
                depth += units * aim;
            },
            _ => panic!("unrecognized command: {}", command),
        }
    }
    println!("Horizontal position: {}", x_pos);
    println!("Depth: {}", depth);
    println!("Aim (not that it matters anymore): {}", aim);
    println!("Multiplied: {}", x_pos * depth);
}

fn part_one(inputs: String) {
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
