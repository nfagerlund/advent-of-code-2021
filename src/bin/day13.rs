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
    // dbg!(&grid);
    // dbg!(&folds);
    let new_grid = grid.crease(&folds[0]);
    // dbg!(&new_grid);
    let count = new_grid.tiles().filter(|t| *(new_grid.get_tile_value(*t).unwrap()) ).count();
    dbg!(count);
    count
}

trait DotGrid {
    fn crease(&self, fold: &Fold) -> Grid<bool>;
}

impl DotGrid for Grid<bool> {
    fn crease(&self, fold: &Fold) -> Grid<bool> {
        match fold.axis {
            Axis::X => {
                let mut creased_data = self.data.clone();
                for row in creased_data.iter_mut() {
                    let right_part = row.split_off(fold.coordinate + 1);
                    // throw away the crease line
                    row.pop();
                    // OR the items on the row:
                    for (index, val) in right_part.iter().enumerate() {
                        let left_slot = (row.len() - 1).checked_sub(index);
                        match left_slot {
                            None => panic!("Folded a larger right than left! need to implement this still."),
                            Some(left_slot) => {
                                let left_val = row[left_slot];
                                row[left_slot] = left_val || *val;
                            }
                        }
                    }
                }
                Grid::new(creased_data)
            },
            Axis::Y => {
                let mut creased_data = self.data.clone();
                let bottom_part = creased_data.split_off(fold.coordinate + 1);
                // throw away the crease line
                creased_data.pop();
                // OR the rows, working backwards on the top and forwards on the bottom
                for (index, row) in bottom_part.iter().enumerate() {
                    let top_row = (creased_data.len() - 1).checked_sub(index);
                    match top_row {
                        None => {
                            panic!("Folded a larger bottom than top! Need to implement this still. (.insert, etc.)");
                        },
                        Some(top_row) => {
                            for (col, val) in row.iter().enumerate() {
                                let top_val = creased_data[top_row][col];
                                creased_data[top_row][col] = top_val || *val;
                            }
                        },
                    }
                }
                Grid::new(creased_data)
            },
        }
    }
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
