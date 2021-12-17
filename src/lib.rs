use std::fs;

pub fn load_inputs(dataset: &str) -> std::io::Result<String> {
    let file = format!("./inputs/{}.txt", dataset);
    fs::read_to_string(file)
}

// Get a number or die trying
pub fn usize_or_die(s: &str) -> usize {
    usize::from_str_radix(s, 10).unwrap()
}

pub fn i32_or_die(s: &str) -> i32 {
    i32::from_str_radix(s, 10).unwrap()
}

pub mod grid {
    use std::fmt;

    /// A grid using game-like coordinates (i.e. 0,0 => upper left corner).
    #[derive(Debug)]
    pub struct Grid<T> {
        data: Vec<Vec<T>>, // vec of rows of values
    }

    pub type Tile = (usize, usize);

    #[derive(Debug)]
    pub enum Axis {
        X,
        Y,
    }

    #[derive(Debug)]
    pub struct OutOfBoundsError;
    impl fmt::Display for OutOfBoundsError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "attempted unrecoverable operation on a tile outside this grid")
        }
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

        pub fn set_tile_value(&mut self, tile: Tile, value: T) -> Result<(), OutOfBoundsError> {
            match self.get_tile_value_mut(tile) {
                None => Err(OutOfBoundsError),
                Some(mut_val) => {
                    *mut_val = value;
                    Ok(())
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
}
