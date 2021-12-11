use advent21::*;

// Lanternfish day!
// all right fishface, I been warned about u.
// I'm going to keep a collection of NINE ELEMENTS ONLY, and y'all get to leave
// my stack alone.

// Oh unrelatedly: TIL that if rust-analyzer keeps losing track of stuff in
// scope from use statements, run cargo check to see if you get the same error,
// and if it's hosed there too, run cargo clean. IDK what went wrong, but that's
// the fix!

fn main() {
    let inputs = load_inputs("day6").unwrap();
    part_one(&inputs);
    part_two(&inputs);
}

fn part_two(_inputs: &str) {}

fn part_one(inputs: &str) -> usize {
    let pop = parse_inputs(inputs);
    let final_pop = n_generations_later(pop, 80);
    println!("Final population: \n{:#?}", &final_pop);
    let count = final_pop.iter()
        .fold(0usize, |total, val| { total + *val });
    println!("Total population size: {}", count);
    count
}

type Population = [usize; 9];

fn n_generations_later(pop: Population, generations: usize) -> Population {
    // ...how do I recursion?? IDK if 80 is too deep, but maybe it's fine, soooo
    // let's just find out!!! yolooo
    println!("{} generations left", generations);
    match generations {
        0 => pop,
        1 => the_next_generation(&pop),
        _ => n_generations_later(pop, generations - 1),
    }
}

fn the_next_generation(pop: &Population) -> Population {
    // I think we can get away with doing this immutably, since really we're
    // only doing like 80 loops of this part.
    let mut next_pop = [0usize; 9];
    let mut pop_iter = pop.iter().enumerate();
    // set the 0 slot aside for later:
    let (_, reproducing) = pop_iter.next().unwrap();
    let reproducing = *reproducing;
    // copy slots 1 thru 8 into their next ticks:
    for (age, count) in pop_iter {
        next_pop[age - 1] = *count;
    }
    // Put the original reproducing fish into slot 6, and put an equal number of
    // NEW fish into slot 8:
    next_pop[6] += reproducing;
    next_pop[8] = reproducing;
    next_pop
}

fn parse_inputs(inputs: &str) -> Population {
    let mut pop = [0usize; 9];
    for age in inputs.trim().split(',') {
        let age = usize_or_die(age);
        // If we get an age counter in the inputs that's older than 8, just
        // panic when we access into the array.
        pop[age] += 1;
    }
    pop
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "3,4,3,1,2\n";

    #[test]
    fn example_part_one() {
        let answer = 5934;
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
