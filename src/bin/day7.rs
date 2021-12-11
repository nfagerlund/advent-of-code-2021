use advent21::*;
use std::cmp;

// Crab submarines day! inputs are horizontal positions. Find the slot that will
// take the least total units of fuel to get everyone to.

fn main() {
    let inputs = load_inputs("day7").unwrap();
    part_one(&inputs);
    part_two(&inputs);
}

fn part_two(inputs: &str) -> usize {

    0
}

fn part_one(inputs: &str) -> usize {
    part_every(inputs, fuel_cost)
}

fn part_every(inputs: &str, fuel_cost_fn: fn(usize, usize) -> usize) -> usize {
    let crab_positions = parse_inputs(inputs);
    // Right OK. I think we can get away with just one additional vec and a
    // double loop, for this one.
    let mut fuel_costs = vec![0usize; crab_positions.len()];
    for (destination, cost) in fuel_costs.iter_mut().enumerate() {
        for (crab_position, crab_count) in crab_positions.iter().enumerate() {
            if *crab_count > 0 {
                let individual_cost = fuel_cost_fn(crab_position, destination);
                *cost += individual_cost * *crab_count;
            }
        }
    }
    let (cheapest_destination, cheapest_cost) = fuel_costs.iter().enumerate().reduce(
        |min, cur| {
            match min.1.cmp(cur.1) {
                cmp::Ordering::Equal => min, // be lazy if dupes
                cmp::Ordering::Less => min,
                cmp::Ordering::Greater => cur,
            }
        }
    ).unwrap();
    let (worst_destination, worst_cost) = fuel_costs.iter().enumerate().reduce(
        |max, cur| {
            match max.1.cmp(cur.1) {
                cmp::Ordering::Equal => max, // be lazy if dupes
                cmp::Ordering::Less => cur,
                cmp::Ordering::Greater => max,
            }
        }
    ).unwrap();
    println!(
        "Cheapest destination: {} ({} fuel units)\nMost expensive destination: {} ({} fuel units)",
        cheapest_destination, cheapest_cost,
        worst_destination, worst_cost,
    );
    *cheapest_cost
}

fn fuel_cost(start: usize, end: usize) -> usize {
    let difference = start as i32 - end as i32;
    difference.abs() as usize
}

// ...I think we need a vec for this, bc who knows what all horizontal positions
// they might have. (Ah yeah, it's out in the thousands somewhere.) Or,
// alternately, I could just find the max first. Especially since I think I want
// to initialize it zeroed-out anyway. Oh, but we won't know the length before
// running the function, so we won't know what return type to use. Okay, vec it
// is!
// - The x-axis is unsigned it looks like.
// - I think we want each slot to represent the number of crabs at that x
//   position (so, usize).
fn parse_inputs(inputs: &str) -> Vec<usize> {
    let inputs = inputs.trim();
    let converted_inputs: Vec<usize> = inputs.split(',').map(usize_or_die).collect();
    let max_x_pos = converted_inputs.iter()
        .fold(0usize, |max, val| cmp::max(max, *val) );
    let mut outputs = vec![0usize; max_x_pos + 1];
        // ah ah, zero-indexing, gotta make sure outputs[max] is a valid slot.
    for crab in converted_inputs {
        outputs[crab] += 1;
    }
    println!("Parsed inputs to vec of crab counts at each x position:\n{:?}", &outputs);
    outputs
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "16,1,2,0,4,2,7,1,2,14\n";

    #[test]
    fn example_part_one() {
        let answer = 37;
        let result = part_one(EXAMPLE);
        assert_eq!(result, answer);
    }

    #[test]
    fn example_part_two() {
        let answer = 168;
        let result = part_two(EXAMPLE);
        assert_eq!(result, answer);
    }
}
