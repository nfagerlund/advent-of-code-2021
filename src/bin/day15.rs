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
            // println!("Current best bet:\n{:?}", &current);
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

fn parse_inputs(inputs: &str) -> Grid<usize> {
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

// Transform the original grid into a 5x5 tesselation! The original grid is a
// tile at (0,0), and we need to fill the super-grid out to (4,4). Each
// micro-tile in a given mega-tile gets the manhattan distance from the (0,0)
// mega-tile added to it, with a wrap-around at 9. (use modulo I guess.)
fn parse_inputs_hugely(inputs: &str) -> Grid<usize> {
    let first_mega_row: Vec<Vec<usize>> = inputs.lines().map(|line| {
        let base_row: Vec<usize> = line.chars().map(|ch| {
            let st = String::from(ch);
            usize_or_die(&st[..])
        }).collect();
        let mut row: Vec<usize> = Vec::with_capacity(base_row.len() * 5);
        for i in 0_usize..5 {
            row.extend(bump_and_wrap9_row(&base_row, i));
        }
        row
    }).collect();
    let mut data: Vec<Vec<usize>> = Vec::with_capacity(first_mega_row.len() * 5);
    for i in 0_usize..5 {
        for row in first_mega_row.iter() {
            data.push(bump_and_wrap9_row(row, i).collect());
        }
    }
    Grid::new(data)
}

fn bump_and_wrap9_row(row: &Vec<usize>, bump: usize) -> impl Iterator<Item = usize> + '_ {
    row.iter().map(move |num| {
        let bumped = *num + bump;
        if bumped > 9 {
            bumped - 9
        } else {
            bumped
        }
    })
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

    #[test]
    fn parse_inputs_hugely_test() {
        let answer = "11637517422274862853338597396444961841755517295286
13813736722492484783351359589446246169155735727126
21365113283247622439435873354154698446526571955763
36949315694715142671582625378269373648937148475914
74634171118574528222968563933317967414442817852555
13191281372421239248353234135946434524615754563572
13599124212461123532357223464346833457545794456865
31254216394236532741534764385264587549637569865174
12931385212314249632342535174345364628545647573965
23119445813422155692453326671356443778246755488935
22748628533385973964449618417555172952866628316397
24924847833513595894462461691557357271266846838237
32476224394358733541546984465265719557637682166874
47151426715826253782693736489371484759148259586125
85745282229685639333179674144428178525553928963666
24212392483532341359464345246157545635726865674683
24611235323572234643468334575457944568656815567976
42365327415347643852645875496375698651748671976285
23142496323425351743453646285456475739656758684176
34221556924533266713564437782467554889357866599146
33859739644496184175551729528666283163977739427418
35135958944624616915573572712668468382377957949348
43587335415469844652657195576376821668748793277985
58262537826937364893714847591482595861259361697236
96856393331796741444281785255539289636664139174777
35323413594643452461575456357268656746837976785794
35722346434683345754579445686568155679767926678187
53476438526458754963756986517486719762859782187396
34253517434536462854564757396567586841767869795287
45332667135644377824675548893578665991468977611257
44961841755517295286662831639777394274188841538529
46246169155735727126684683823779579493488168151459
54698446526571955763768216687487932779859814388196
69373648937148475914825958612593616972361472718347
17967414442817852555392896366641391747775241285888
46434524615754563572686567468379767857948187896815
46833457545794456865681556797679266781878137789298
64587549637569865174867197628597821873961893298417
45364628545647573965675868417678697952878971816398
56443778246755488935786659914689776112579188722368
55172952866628316397773942741888415385299952649631
57357271266846838237795794934881681514599279262561
65719557637682166874879327798598143881961925499217
71484759148259586125936169723614727183472583829458
28178525553928963666413917477752412858886352396999
57545635726865674683797678579481878968159298917926
57944568656815567976792667818781377892989248891319
75698651748671976285978218739618932984172914319528
56475739656758684176786979528789718163989182927419
67554889357866599146897761125791887223681299833479
";
        assert_eq!(parse_inputs(answer).data, parse_inputs_hugely(EXAMPLE).data);
    }
}
