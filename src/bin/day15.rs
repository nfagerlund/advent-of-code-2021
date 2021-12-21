use advent21::*;
use advent21::grid::*;
use std::collections::BinaryHeap;
use std::fmt::Binary;
use std::cmp::Ordering;

// The one where yup, it's another grid, and you gotta do pathfinding to get the
// lowest total score from top-left to bottom-right. It's totally A*.
fn main() {
    let inputs = load_inputs("day15").unwrap();
    part_one(&inputs);
    part_two(&inputs);
}

fn part_two(_inputs: &str) {}

fn part_one(inputs: &str) -> usize {
    let grid = parse_inputs(inputs);
    // println!("The stuff is here: \n{:#?}", &grid);
    0
}

#[derive(Eq, PartialEq)]
struct Path {
    route: Vec<Tile>,
    cost: usize,
}

impl Path {
    fn new(start: Tile) -> Path {
        // Per the problem statement, your starting tile doesn't add to your
        // cost. Only entering a tile bumps it!
        Path { route: vec![start], cost: 0 }
    }

    fn head(&self) -> &Tile {
        // We'll always have at least one tile in the route, so unwrap.
        self.route.last().unwrap()
    }
}

// Per the BinaryHeap docs, it's only a max heap but I can get a min heap by
// defining a custom Ord. And I need that anyway because I only want to compare
// paths by cost!
impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        // reversed
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // but don't double-reverse.
        Some(self.cmp(&other))
    }
}
// and then I THINK I can get away with just derived eqs, because I do want to
// compare all fields on those.????

fn parse_inputs (inputs: &str) -> Grid<usize> {
    let grid_data: Vec<Vec<usize>> = inputs.lines().map(
        |line| {
            line.chars().map(
                |ch| {
                    let st = String::from(ch);
                    usize_or_die(&st[..])
                }
            ).collect()
        }
    ).collect();
    Grid::new(grid_data)
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
