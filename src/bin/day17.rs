use advent21::*;

// The one with the trick shot.
fn main() {
    let inputs = load_inputs("day17").unwrap();
    part_one(&inputs);
    part_two(&inputs);
}

fn part_two(_inputs: &str) {}

// Return the highest Y position the probe can hit on a trajectory that will at
// some point be within the target area on a step.
fn part_one(inputs: &str) -> i32 {

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "target area: x=20..30, y=-10..-5\n";

    #[test]
    fn example_part_one() {
        let answer = 45;
        let result = part_one(EXAMPLE);
        assert_eq!(result, answer);
    }

    #[test]
    fn example_part_two() {
        let answer = ();
        let result = part_two(EXAMPLE);
        assert_eq!(result, answer);
    }
}
