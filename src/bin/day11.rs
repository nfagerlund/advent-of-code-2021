use advent21::*;
use advent21::grid::{Grid, Tile};
use easycurses::*;
use std::{thread, time};

// the one with the dumbo octopuses
fn main() {
    let inputs = load_inputs("day11").unwrap();
    part_one(&inputs);
    part_two(&inputs);
}

fn part_two(inputs: &str) -> usize {
    let mut octogrid = parse_inputs(inputs);
    let mut curse = EasyCurses::initialize_system().unwrap();
    curse.set_cursor_visibility(CursorVisibility::Invisible);
    curse.set_echo(false);

    // draw initial state:
    octogrid.draw_all_octopi(&mut curse);

    // println!("The stuff is here:\n{:#?}", &octogrid);
    // TO FINITY, AND BEYOND,
    let result = (1..usize::MAX).map(|step| {
        // pass 1: get everybody's heart started
        octogrid.charge_octopi();
        // pass 2: recursively flash octopi.
        octogrid.flash_octopi();
        // PASS 2.5: PRETTY PICTURES
        octogrid.draw_all_octopi(&mut curse);
        // pass 3: reset any octopi who need it
        let flash_count = octogrid.reset_octopi();
        // Oh right, we also need to get the count of flashes. Let's do that above.
        (step, flash_count)
    }).find(|(_, flash_count)| *flash_count == 100 );

    // clean up curses session:
    drop(curse);

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

trait OctoGrid {
    fn charge_octopi(&mut self);
    fn flash_octopi(&mut self);
    fn recursively_flash_octopi(&mut self, tile: Tile);
    // also returns the number of flashes discovered.
    fn reset_octopi(&mut self) -> usize;
    fn draw_octopus(&self, tile: Tile, curse: &mut EasyCurses);
    fn draw_all_octopi(&self, curse: &mut EasyCurses);
}

impl OctoGrid for Grid<Octopus> {
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

    fn draw_octopus(&self, tile: Tile, curse: &mut EasyCurses) {
        let octo = self.get_tile_value(tile).unwrap();
        let (x, y) = tile;
        let x = x * 2;
        if octo.flashed() {
            curse.set_color_pair(ColorPair::new(Color::Cyan, Color::Black));
        } else {
            curse.set_color_pair(ColorPair::new(Color::Blue, Color::Black));
        }
        let energy = octo.energy();
        let ch = match energy {
            (0..=9) => char::from_digit(energy as u32, 10).unwrap(),
            _ => '*',
        };
        curse.move_rc(y as i32, x as i32);
        curse.print_char(ch);
        // Need to call refresh after calling this!
    }

    fn draw_all_octopi(&self, curse: &mut EasyCurses) {
        for tile in self.tiles() {
            self.draw_octopus(tile, curse);
        }
        curse.refresh();
        // make sure I can see the damn thing!
        thread::sleep(time::Duration::from_millis(80));
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

    fn energy(&self) -> usize {
        self.energy
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

// Update: we're gonna use Grid a third time, so I'm moving it to lib.

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
