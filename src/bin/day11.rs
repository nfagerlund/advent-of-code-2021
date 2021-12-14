use advent21::*;

// the one with the dumbo octopuses
fn main() {
    let inputs = load_inputs("day11").unwrap();
    part_one(&inputs);
    part_two(&inputs);
}

fn part_two(inputs: &str) {}

// Hmm. This might be easier than I first thought.
fn part_one(inputs: &str) -> usize {

    0
}

// But before any of that, it's time to set up our grid! IDK if we'll use this
// a third time, but if so I'll move it to lib. For the time being I'll make it
// more generic anyway.

/// A grid using game-like coordinates (i.e. 0,0 => upper left corner).
pub struct Grid<T> {
    data: Vec<Vec<T>>, // vec of rows of values
}

pub type Tile = (usize, usize);
enum Axis {
    X,
    Y,
}

impl<T> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Grid<T> {
        Grid { data }
    }

    pub fn height(&self) -> usize { self.data.len() }
    // Constructor is in charge of not giving us inconsistent widths! I'm still not checking.
    pub fn width(&self) -> usize { self.data[0].len() }

    pub fn tiles(&self) -> Box<dyn Iterator<Item = Tile>> {
        let height = self.height();
        let width = self.width();
        let tiles_iter = (0..height).map(move |y| {
            (0..width).map(move |x| (x, y))
        }).flatten();
        Box::new(tiles_iter)
    }

    /// Returns an immutable reference to the value in a tile. `None` means the
    /// coordinates in the requested tile are out of bounds; in curent
    /// implementation, every tile has to have a valid value, so you gotta use
    /// an Option of your own if some tiles need empty values.
    pub fn get_tile_value(&self, tile: Tile) -> Option<&T> {
        let (x, y) = tile;
        match self.data.get(y) { // row
            None => None,
            Some(row) => {
                match row.get(x) {
                    None => None,
                    Some(value) => Some(value),
                }
            },
        }
    }

    /// Same as before but mutable reference.
    pub fn get_tile_value_mut(&mut self, tile: Tile) -> Option<&mut T> {
        let (x, y) = tile;
        match self.data.get_mut(y) { // row
            None => None,
            Some(row) => {
                match row.get_mut(x) {
                    None => None,
                    Some(value) => Some(value),
                }
            },
        }
    }

    // Smash this neighbors thing apart a bit:

    /// Like usize.checked_sub(), except based on the grid size.
    fn more(&self, coord: usize, axis: Axis) -> Option<usize> {
        let axis_len = match axis {
            Axis::X => self.width(),
            Axis::Y => self.height(),
        };
        if coord + 1 < axis_len {
            Some(coord + 1)
        } else {
            None
        }
    }

    fn get_neighbor_n(&self, tile: Tile) -> Option<Tile> {
        let (x, y) = tile;
        if let Some(y) = y.checked_sub(1) {
            return Some((x, y));
        }
        None
    }
    fn get_neighbor_w(&self, tile: Tile) -> Option<Tile> {
        let (x, y) = tile;
        if let Some(x) = x.checked_sub(1) {
            return Some((x, y));
        }
        None
    }
    fn get_neighbor_nw(&self, tile: Tile) -> Option<Tile> {
        let (x, y) = tile;
        if let Some(y) = y.checked_sub(1) {
            if let Some(x) = x.checked_sub(1) {
                return Some((x, y));
            }
        }
        None
    }
    fn get_neighbor_s(&self, tile: Tile) -> Option<Tile> {
        let (x, y) = tile;
        if let Some(y) = self.more(y, Axis::Y) {
            return Some((x, y));
        }
        None
    }
    fn get_neighbor_e(&self, tile: Tile) -> Option<Tile> {
        let (x, y) = tile;
        if let Some(x) = self.more(x, Axis::X) {
            return Some((x, y));
        }
        None
    }
    fn get_neighbor_se(&self, tile: Tile) -> Option<Tile> {
        let (x, y) = tile;
        if let Some(y) = self.more(y, Axis::Y) {
            if let Some(x) = self.more(x, Axis::X) {
                return Some((x, y));
            }
        }
        None
    }
    fn get_neighbor_ne(&self, tile: Tile) -> Option<Tile> {
        let (x, y) = tile;
        if let Some(y) = y.checked_sub(1) {
            if let Some(x) = self.more(x, Axis::X) {
                return Some((x, y));
            }
        }
        None
    }
    fn get_neighbor_sw(&self, tile: Tile) -> Option<Tile> {
        let (x, y) = tile;
        if let Some(y) = self.more(y, Axis::Y) {
            if let Some(x) = x.checked_sub(1) {
                return Some((x, y));
            }
        }
        None
    }

    // get coords
    pub fn get_neighbors_cardinal(&self, tile: Tile) -> Vec<Option<Tile>> {
        vec![
            self.get_neighbor_n(tile),
            self.get_neighbor_e(tile),
            self.get_neighbor_s(tile),
            self.get_neighbor_w(tile),
        ]
    }
    pub fn get_neighbors_ordinal(&self, tile: Tile) -> Vec<Option<Tile>> {
        vec![
            self.get_neighbor_ne(tile),
            self.get_neighbor_nw(tile),
            self.get_neighbor_se(tile),
            self.get_neighbor_sw(tile),
        ]
    }
    pub fn get_neighbors_all(&self, tile: Tile) -> Vec<Option<Tile>> {
        let mut neighbors = self.get_neighbors_cardinal(tile);
        let mut ordinal_neighbors = self.get_neighbors_ordinal(tile);
        neighbors.append(&mut ordinal_neighbors);
        neighbors
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"#;

    #[test]
    fn example_part_one() {
        let answer = 1656;
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
    fn that_iterator_thing() {
        let data = vec![
            vec![1, 0, 5, 5],
            vec![0, 8, 9, 9],
        ];

        let grid = Grid::new(data);
        let count = grid.tiles().count();
        assert_eq!(count, 8);
    }
}
