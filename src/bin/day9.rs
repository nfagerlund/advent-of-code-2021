use advent21::*;

// The one with the smoke seeking low points.
fn main() {
    let inputs = load_inputs("day9").unwrap();
    part_one(&inputs);
    part_two(&inputs);
}

// Sizes of the three largest basins, multiplied together.
// OK! I think... hashmap. then collect values in a vec (bc we stop caring about
// locations), sort them, reverse and take three.
// Only question is whether we can use a tuple as a hash key...
fn part_two(inputs: &str) -> usize {

    0
}

// (all low points).map(+1).sum()
fn part_one(inputs: &str) -> usize {
    let grid = parse_inputs(inputs);
    println!("The stuff is here. First row: \n{:?}", &grid.data[0]);
    // just uh... check all tiles! And add them to a running total!
    let mut total: usize = 0;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let tile = (x, y);
            if grid.tile_is_low_point(tile) {
                let height = grid.get_tile_height(tile).unwrap();
                // let neighbors = grid.get_neighbor_heights(tile);
                // println!("Got a low point! ({},{}) => {}\n    neighbors: {:?}", x, y, height, neighbors);
                let risk_level = height + 1;
                total += risk_level;
            }
        }
    }

    println!("Total height of all low points: {}", total);

    total
}

// OK, I think I see where we're going here!
type Tile = (usize, usize);
struct Grid {
    pub data: Vec<Vec<usize>>, // vec of rows
}

impl Grid {
    fn height(&self) -> usize { self.data.len() }
    // Constructor is in charge of not giving us inconsistent widths!
    fn width(&self) -> usize { self.data[0].len() }

    fn get_tile_height(&self, tile: Tile) -> Option<usize> {
        let (x, y) = tile;
        match self.data.get(y) { // row
            None => None,
            Some(row) => {
                match row.get(x) {
                    None => None,
                    Some(&height) => Some(height),
                }
            },
        }
    }

    // get coords
    fn get_neighbor_tiles(&self, tile: Tile) -> Vec<Option<Tile>> {
        let (x, y) = tile;
        vec![
            // Need to do some goofy bounds checking so we don't wrap around to
            // usize::MAX by doing 0_usize - 1:
            // (which, btw, panics in dev builds, which is very nice.)
            match x.checked_sub(1) {
                None => None,
                Some(less_x) => Some((less_x, y)),
            },
            match y.checked_sub(1) {
                None => None,
                Some(less_y) => Some((x, less_y)),
            },
            // Then gotta make sure we don't go past the outer edge:
            if x + 1 < self.width() {
                Some((x + 1, y))
            } else {
                None
            },
            if y + 1 < self.height() {
                Some((x, y + 1))
            } else {
                None
            },
        ]
    }

    fn get_neighbor_heights(&self, tile: Tile) -> Vec<Option<usize>> {
        self.get_neighbor_tiles(tile).iter().map(|&neighbor| {
            match neighbor {
                None => None,
                Some(tile) => self.get_tile_height(tile),
            }
        }).collect()
    }

    fn tile_is_low_point(&self, tile: Tile) -> bool {
        let tile_height = self.get_tile_height(tile);
        let neighbor_heights = self.get_neighbor_heights(tile);
        // right, first off,
        if let None = tile_height {
            return false;
        }
        let tile_height = tile_height.unwrap();
        for maybe_neighbor in neighbor_heights {
            if let Some(neighbor) = maybe_neighbor {
                if tile_height >= neighbor {
                    return false;
                }
            }
        }
        true
    }
}

fn parse_inputs(inputs: &str) -> Grid {
    let data: Vec<Vec<usize>> = inputs.lines().map(|line| {
        line.chars().map(|c| {
            c.to_digit(10).unwrap() as usize
        }).collect()
    }).collect();
    Grid { data }
}


#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678
"#;

    #[test]
    fn example_part_one() {
        let answer = 15;
        let result = part_one(EXAMPLE);
        assert_eq!(result, answer);
    }

    #[test]
    fn example_part_two() {
        let answer = 1134;
        let result = part_two(EXAMPLE);
        assert_eq!(result, answer);
    }
}
