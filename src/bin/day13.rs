use advent21::*;
use advent21::grid::{Grid,Tile,Axis};
use std::cmp;

fn main() {
    let inputs = load_inputs("day13").unwrap();
    part_one(&inputs);
    part_two(&inputs);
}

fn part_two(_inputs: &str) {}

// Count of visible dots after completing only the first fold.
fn part_one(inputs: &str) -> usize {
    let (grid, folds) = parse_inputs(inputs);
    println!("The stuff is here:");
    dbg!(&grid);
    dbg!(&folds);

    0
}

#[derive(Debug)]
struct Fold {
    coordinate: usize,
    axis: Axis,
}

fn parse_inputs(inputs: &str) -> (Grid<bool>, Vec<Fold>) {
    let (coords, folds) = inputs.split_once("\n\n").unwrap();
    (parse_dots(coords), parse_folds(folds))
}

fn parse_folds(inputs: &str) -> Vec<Fold> {
    let folds: Vec<Fold> = inputs.lines().map(|line| {
        let (axis, coordinate) = line
            .strip_prefix("fold along ").unwrap()
            .split_once('=').unwrap();
        let axis = match axis {
            "x" => Axis::X,
            "y" => Axis::Y,
            _ => panic!("wyd"),
        };
        let coordinate = usize_or_die(coordinate);
        Fold {
            axis,
            coordinate,
        }
    }).collect();
    folds
}

fn parse_dots(inputs: &str) -> Grid<bool> {
    let dots: Vec<Tile> = inputs.lines().map(|line| {
        let (x, y) = line.split_once(',').unwrap();
        (usize_or_die(x), usize_or_die(y))
    }).collect();
    let (max_x, max_y) = dots.iter().fold((0, 0), |maxes, &dot| {
        (cmp::max(maxes.0, dot.0), cmp::max(maxes.1, dot.1))
    });
    let mut data: Vec<Vec<bool>> = Vec::with_capacity(max_y + 1);
    // initialize every tile as false
    for _ in 0..=max_y {
        data.push(vec![false; max_x + 1]);
    }
    // Grid now, so we can use convenience methods:
    let mut grid = Grid::new(data);
    // mark dots as true
    for tile in dots {
        grid.set_tile_value(tile, true).unwrap();
    }
    // ok, should be good:
    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
"#;

    #[test]
    fn example_part_one() {
        let answer = 17;
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
