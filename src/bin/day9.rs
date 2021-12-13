use advent21::*;

// The one with the smoke seeking low points.
fn main() {
    let inputs = load_inputs("day9").unwrap();
    part_one(&inputs);
    part_two(&inputs);
}

fn part_two(_inputs: &str) {}

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
                println!("Got a low point! ({},{}) => {}", x, y, height);
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

    fn get_neighbor_heights(&self, tile: Tile) -> Vec<Option<usize>> {
        let (x, y) = tile;
        vec![
            // Need to do some goofy bounds checking so we don't wrap around to
            // usize::MAX by doing 0_usize - 1:
            // (which, btw, panics in dev builds, which is very nice.)
            match x.checked_sub(1) {
                None => None,
                Some(less_x) => self.get_tile_height((less_x, y)),
            },
            match y.checked_sub(1) {
                None => None,
                Some(less_y) => self.get_tile_height((x, less_y)),
            },
            self.get_tile_height((x + 1, y)),
            self.get_tile_height((x, y + 1)),
        ]
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
                if tile_height > neighbor {
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
        let answer = ();
        let result = part_two(EXAMPLE);
        assert_eq!(result, answer);
    }
}
