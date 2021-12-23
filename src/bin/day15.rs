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
    let end = (grid.width() - 1, grid.height() - 1);
    let start: (usize, usize) = (0,0);
    let mut pathfinder = PathFinder::new(grid, start, end);
    pathfinder.traverse();
    let final_cost = pathfinder.final_cost();
    println!("Final cost of best path: {}", final_cost);
    final_cost
}

/// Route represents a single step of the journey. It has some of the history of
/// the path embedded in it in the form of its cost, but it expects to be used
/// along with a global lookup table so you can find the came_from of its
/// came_from. Is this getting too heavyweight? Hard to say yet.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct Route {
    tile: Tile,
    came_from: Option<Tile>,
    cost_so_far: usize,
    h_distance: usize,
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
impl Route {
    /// Normal ordering: lesser is cheaper!
    fn cmp_cost(&self, other: &Self) -> Ordering {
        // reversed
        self.cost_so_far.cmp(&other.cost_so_far)
    }
    /// The heuristic total we use for deciding which route is most promising.
    /// AKA "f" (= g + h)
    fn est_total(&self) -> usize {
        self.cost_so_far + self.h_distance
    }
}
impl Ord for Route {
    /// Reversed ordering: greater is cheaper, based on cost + heuristic!
    fn cmp(&self, other: &Self) -> Ordering {
        other.est_total().cmp(&self.est_total())
    }
}
impl PartialOrd for Route {
    /// Normal ordering: don't double-reverse, just inherit from cmp.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
// and then I THINK I can get away with just derived eqs, because I do want to
// compare all fields on those.????

// Actually, let's wrap some of these data structure updates.
#[derive(Debug)]
struct PathFinder {
    priority_frontier: BinaryHeap<Route>,
    routes: HashMap<Tile, Route>,
    start: Tile,
    end: Tile,
    grid: Grid<usize>,
}
impl PathFinder {
    fn new(grid: Grid<usize>, start: Tile, end: Tile) -> Self {
        let mut priority_frontier = BinaryHeap::new();
        let mut routes = HashMap::new();
        // wow type inference is cool.
        let start_route = Route {
            tile: start,
            came_from: None,
            cost_so_far: 0,
            h_distance: manhattan_distance(start, end),
        };
        routes.insert(start, start_route);
        priority_frontier.push(start_route);

        PathFinder {
            priority_frontier,
            routes,
            start,
            end,
            grid,
        }
    }
    fn heuristic(&self, tile: Tile) -> usize {
        manhattan_distance(tile, self.end)
    }
    fn definitely_add(&mut self, destination: Tile, route: Route) {
        self.priority_frontier.push(route);
        self.routes.insert(destination, route);
    }
    fn maybe_add(&mut self, destination: Tile, route: Route) {
        // 1. I think Routes are Copy, fingers crossed.
        // 2. Only want to add if it's better than current route to this dest.
        // Also: could early-exit for case of start, but it's not needed.
        match self.routes.get(&destination) {
            None => {
                // Never been here! add it.
                self.definitely_add(destination, route);
            },
            Some(old_route) => {
                // not sure yet:
                match route.cmp_cost(old_route) {
                    Ordering::Less => {
                        self.definitely_add(destination, route);
                    },
                    _ => {}, // Current route's better, keep it.
                }
            },
        };
    }
    fn make_route(&self, destination: Tile, came_from: Tile, cost_so_far: usize) -> Route {
        let step_cost = self.grid.get_tile_value(destination).unwrap();
        Route {
            tile: destination,
            came_from: Some(came_from),
            cost_so_far: cost_so_far + *step_cost,
            h_distance: self.heuristic(destination),
        }
    }
    /// Returns None if there's no steps left to take.
    fn step(&mut self) -> Option<()> {
        if let Some(current) = self.priority_frontier.pop() {
            println!("Current best bet:\n{:?}", &current);
            let cost_so_far = current.cost_so_far;
            let came_from = current.tile;
            for neighbor in self.grid.get_neighbors_cardinal(current.tile) {
                if let Some(neighbor) = neighbor {
                    let route = self.make_route(neighbor, came_from, cost_so_far);
                    self.maybe_add(neighbor, route);
                    if neighbor == self.end {
                        return None; // just reached end
                    }
                }
            }
            return Some(()); // didn't reach end
        }
        None // nothing left to pop, must have reached end.
    }
    fn traverse(&mut self) {
        while let Some(_) = self.step() {}; // I think??
    }

    /// Panics if we haven't yet reached the end of the road.
    fn final_cost(&self) -> usize {
        self.routes.get(&self.end).unwrap().cost_so_far
    }

    /// Panics if we haven't yet reached the end of the road.
    /// Returns (full_path, cost)
    fn reconstruct_path(&self) -> (Vec<Tile>, usize) {
        let mut path = Vec::new();
        path.push(self.end);
        let mut currently = self.end;
        let total_cost = self.routes.get(&self.end).unwrap().cost_so_far;
        while let Some(
            Route {
                tile: _,
                came_from: Some(came_from),
                cost_so_far: _,
                h_distance: _,
            }
        ) = self.routes.get(&currently) {
            path.push(*came_from);
            currently = *came_from;
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
