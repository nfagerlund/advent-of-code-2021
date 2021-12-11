use advent21::*;

// Crab submarines day! inputs are horizontal positions. Find the slot that will
// take the least total units of fuel to get everyone to.

fn main() {
    let inputs = load_inputs("day7").unwrap();
    part_one(&inputs);
    part_two(&inputs);
}

fn part_two(_inputs: &str) {}

fn part_one(inputs: &str) -> usize {

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "16,1,2,0,4,2,7,1,2,14\n";

    #[test]
    fn example_part_one() {
        let answer = 37;
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
