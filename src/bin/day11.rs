use advent21::*;

// the one with the dumbo octopuses
fn main() {
    let inputs = load_inputs("day11").unwrap();
    part_one(&inputs);
    part_two(&inputs);
}

fn part_two(inputs: &str) -> usize {
    let mut octogrid = parse_inputs(inputs);
    // println!("The stuff is here:\n{:#?}", &octogrid);
    // TO FINITY, AND BEYOND,
    let result = (1..usize::MAX).map(|step| {
        // pass 1: get everybody's heart started
        octogrid.charge_octopi();
        // pass 2: recursively flash octopi.
        octogrid.flash_octopi();
        // pass 3: reset any octopi who need it
        let flash_count = octogrid.reset_octopi();
        // Oh right, we also need to get the count of flashes. Let's do that above.
        (step, flash_count)
    }).find(|(_, flash_count)| *flash_count == 100 );
    let first_synchronized_step = result.unwrap().0;
    println!("First step with all flashes synchronized: {}", first_synchronized_step);
    first_synchronized_step
}

// Hmm. This might be easier than I first thought.
fn part_one(inputs: &str) -> usize {
    let mut octogrid = parse_inputs(inputs);
    // println!("The stuff is here:\n{:#?}", &octogrid);
    let flash_total: usize = (0..100).map(|_| {
        // pass 1: get everybody's heart started
        octogrid.charge_octopi();
        // pass 2: recursively flash octopi.
        octogrid.flash_octopi();
        // pass 3: reset any octopi who need it
        let flash_count = octogrid.reset_octopi();
        // Oh right, we also need to get the count of flashes. Let's do that above.
        flash_count
    }).sum();
    println!("Total flashes after 100 steps: {}", flash_total);
    flash_total
}

// Ok, right, so I THINK each octopus should be a struct.
// oh wait. That was when I was thinking I needed to keep track of who had
// flashed this turn. But, I think we can just let the values keep increasing
// past 9, use the transition from 9 -> 10 as the flash trigger, and do a final
// pass to reset to 0. OK, octopus can just be a usize. Wow, all that work to
// get the grid generic for nothing.

// OK, so,
// - first pass: increase every octopus's energy by 1.
// - second pass: .......
//      AH HA, I DO NEED A STRUCT! Consider: 1 10 11 2. Oct0: skip. Oct1: flash; oct2 goes to 12. oct2: should we flash? Can't tell if we did it already! OK fine.
// - second pass: iterate over every octopus. If energy > 9 and flashed == false, FLASH.
// - FLASH: ...ok let's just stfu and write some code. But the point is, gotta
//   handle flashing somewhat recursively because it might backtrack to an
//   octopus we already processed!
// - third pass: iterate over tiles, count flashes, and reset octopi.

impl Grid<Octopus> {
    fn charge_octopi(&mut self) {
        for tile in self.tiles() {
            let octo = self.get_tile_value_mut(tile).unwrap();
            octo.charge();
        }
    }

    fn flash_octopi(&mut self) {
        for tile in self.tiles() {
            self.recursively_flash_octopi(tile);
        }
    }

    // also returns the number of flashes discovered.
    fn reset_octopi(&mut self) -> usize {
        let mut flashes: usize = 0;
        for tile in self.tiles() {
            let octo = self.get_tile_value_mut(tile).unwrap();
            if octo.flashed() {
                flashes += 1;
            }
            octo.reset();
        }
        flashes
    }

    fn recursively_flash_octopi(&mut self, tile: Tile) {
        let octo = self.get_tile_value_mut(tile).unwrap();
        if octo.should_flash() {
            octo.flash();
            for neighbor in self.get_neighbors_all(tile) {
                if let Some(neighbor) = neighbor {
                    // charge each neighbor
                    self.get_tile_value_mut(neighbor).unwrap().charge();
                    // maybe flash it, if called for.
                    self.recursively_flash_octopi(neighbor);
                }
            }
        }
    }
}

#[derive(Debug)]
struct Octopus {
    energy: usize,
    flashed: bool,
}

impl Octopus {
    fn new(energy: usize) -> Octopus {
        Octopus { energy, flashed: false }
    }

    fn charge(&mut self) {
        self.energy += 1;
    }

    fn flash(&mut self) {
        self.flashed = true;
    }

    fn flashed(&self) -> bool {
        self.flashed
    }

    fn should_flash(&self) -> bool {
        !self.flashed && self.energy > 9
    }

    fn reset(&mut self) {
        if self.flashed && self.energy > 9 {
            self.flashed = false;
            self.energy = 0;
        } else if !self.flashed && self.energy <= 9 {
            // do nothing
        } else {
            panic!("This octopus wasn't processed properly! {:?}", self)
        }
    }
}

// But before any of that, it's time to set up our grid! IDK if we'll use this
// a third time, but if so I'll move it to lib. For the time being I'll make it
// more generic anyway.

/// A grid using game-like coordinates (i.e. 0,0 => upper left corner).
#[derive(Debug)]
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

fn parse_inputs(inputs: &str) -> Grid<Octopus> {
    let data: Vec<Vec<Octopus>> = inputs.lines().map(|line| {
        line.chars().map(|ch| {
            let energy = ch.to_digit(10).unwrap();
            Octopus::new(energy as usize)
        }).collect()
    }).collect();
    Grid::new(data)
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
        let answer = 195;
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
        for tile in grid.tiles() {
            println!("{:?}", tile);
        }
        let count = grid.tiles().count();
        assert_eq!(count, 8);
    }
}
