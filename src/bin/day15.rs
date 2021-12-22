use advent21::*;
use advent21::grid::*;
use std::collections::{BinaryHeap, HashSet, HashMap};
use std::cmp::Ordering;

// The one where yup, it's another grid, and you gotta do pathfinding to get the
// lowest total score from top-left to bottom-right. It's totally A*.
// Best articles I could find so far:
// https://www.redblobgames.com/pathfinding/a-star/introduction.html
// https://www.redblobgames.com/pathfinding/a-star/implementation.html#algorithm
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

/// Route represents a single edge of the journey. It has some of the history of
/// the path embedded in it in the form of its cost, but it expects to be used
/// along with a global lookup table so you can find the came_from of its
/// came_from.
#[derive(Eq, PartialEq)]
struct Route {
    came_from: Option<Tile>,
    cost: usize,
}

/// Oh hmm, I think I misunderstood something in my imagined version of this. I
/// was going to store each full path on the priority heap to avoid contention
/// for mutable state between paths... but the article I was reading only stored
/// a single dict of came_froms, and now that I think about it, the conditional
/// guard to only overwrite when the current route is cheaper is all the
/// protection we need -- it's actually fine and good that routes can stomp on
/// the state of other routes, and if the directional field gets corrupted
/// partway through, well, we only need the _one best path_ to be legible at the
/// end!

// Per the BinaryHeap docs, it's only a max heap but I can get a min heap by
// defining a custom Ord. And I need that anyway because I only want to compare
// paths by cost!
impl Ord for Route {
    fn cmp(&self, other: &Self) -> Ordering {
        // reversed
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for Route {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // but don't double-reverse.
        Some(self.cmp(&other))
    }
}
// and then I THINK I can get away with just derived eqs, because I do want to
// compare all fields on those.????

// Actually, let's wrap some of these data structure updates.
struct PathFinder {
    priority_frontier: BinaryHeap<Tile>,
    routes: HashMap<Tile, Route>,
    start: Tile,
    end: Tile,
}
impl PathFinder {
    fn new(start: Tile, end: Tile) -> Self {
        let mut priority_frontier = BinaryHeap::new();
        let mut routes = HashMap::new();
        // wow type inference is cool.
        priority_frontier.push(start);
        let null_route = Route { came_from: None, cost: 0 };
        routes.insert(start, null_route);

        PathFinder {
            priority_frontier,
            routes,
            start,
            end,
        }
    }
    fn hard_insert(&mut self, destination: Tile, route: Route) {
        self.priority_frontier.push(destination);
        self.routes.insert(destination, route);
    }
    fn visit(&mut self, destination: Tile, route: Route) {
        // 1. I think Routes are Copy, fingers crossed.
        // 2. Only want to add if it's better than current route to this dest.
        // Also: could early-exit for case of start, but it's not needed.
        match self.routes.get(&destination) {
            None => {
                // Never been here! add it.
                self.hard_insert(destination, route);
            },
            Some(old_route) => {
                // not sure yet:
                match route.cmp(old_route) {
                    Ordering::Greater => {
                        // Remember we did a custom Ord? So that means if the
                        // new route is Greater than the old one, it has a lower
                        // cost. Greater == Better.
                        self.hard_insert(destination, route);
                    },
                    _ => {}, // Current route's better, keep it.
                }
            },
        };
    }

    /// Panics if we haven't yet reached the end of the road.
    /// (full_path, cost)
    fn reconstruct_path(&self) -> (Vec<Tile>, usize) {
        let mut path = Vec::new();
        path.push(self.end);
        let mut currently = self.end;
        let mut total_cost: usize = 0;
        while let Some(Route{came_from: Some(came_from), cost}) = self.routes.get(&currently) {
            path.push(*came_from);
            currently = *came_from;
            total_cost += *cost;
        }

        (path, total_cost)
    }
}

fn manhattan_distance(start: Tile, end: Tile) -> usize {
    let x_diff = (start.0 as i32) - (end.0 as i32);
    let y_diff = (start.1 as i32) - (end.1 as i32);
    let sum = x_diff.abs() + y_diff.abs();
    sum as usize
}

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
