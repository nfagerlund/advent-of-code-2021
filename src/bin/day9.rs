use advent21::*;

// The one with the smoke seeking low points.
fn main() {
    let inputs = load_inputs("day9").unwrap();
    part_one(&inputs);
    part_two(&inputs);
}

fn part_two(inputs: &str) {}

fn part_one(inputs: &str) -> usize {
    let grid = parse_inputs(inputs);
    println!("The stuff is here. First row: \n{:?}", &grid.data[0]);

    0
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
            self.get_tile_height((x - 1, y)),
            self.get_tile_height((x + 1, y)),
            self.get_tile_height((x, y - 1)),
            self.get_tile_height((x, y + 1)),
        ]
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
