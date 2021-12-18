use advent21::*;

// The one where yup, it's another grid, and you gotta do pathfinding to get the
// lowest total score from top-left to bottom-right. A*?
fn main() {
    let inputs = load_inputs("day15").unwrap();
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
    const EXAMPLE: &str = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"#;

    #[test]
    fn example_part_one() {
        let answer = 40;
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
