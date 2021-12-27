use advent21::*;

// The one with the trick shot.
fn main() {
    let inputs = load_inputs("day17").unwrap();
    part_one(&inputs);
    part_two(&inputs);
}

// Find every initial velocity that causes the probe to hit the target on Some
// Step, and return the count.
fn part_two(inputs: &str) -> usize {
    let results = parse_inputs(inputs);
    let x1 = results[0];
    let x2 = results[1];
    let y1 = results[2];
    let y2 = results[3];

    let minimum_v_x = (0..i32::MAX).find(|val| {
        val.pow(2) + *val >= x1 * 2
    }).unwrap();
    println!("Lower bound for v_x: {}", minimum_v_x);
    let maximum_v_x = x2;
    let maximum_v_y = -y1 - 1; // from part_one
    let minimum_v_y = y1;
    // Right right, and we only care about the count, don't we?
    let mut valid_trajectories: usize = 0;

    for v_x in minimum_v_x..=maximum_v_x {
        for v_y in minimum_v_y..=maximum_v_y {
            for step in 1..i32::MAX {
                let x = plot_x(v_x, step);
                if x > x2 {
                    // overshot. Better luck next time.
                    break;
                }
                if x >= x1 {
                    // Maybe!!
                    let y = plot_y(v_y, step);
                    if y > y2 {
                        // overshot.
                        break;
                    }
                    if y >= y1 {
                        // got one!
                        valid_trajectories += 1;
                        break;
                    }
                    // otherwise not there yet.
                }
                // otherwise not there yet.
            }
        }
    }
    println!("Found {} valid trajectories", valid_trajectories);

    valid_trajectories
}

// Return the highest Y position the probe can hit on a trajectory that will at
// some point be within the target area on a step.
fn part_one(inputs: &str) -> i32 {
    let final_y = parse_inputs_to_cheat_outrageously(inputs);
    // rudely assuming bottom of target area is always a negative y coord
    let v_y = -final_y - 1;
    // ...and then the peak is always when step == v_y, so,
    let max_height = plot_y(v_y, v_y);
    println!("Max height reached on optimal trajectory: {}", max_height);
    max_height
}

// OK, I derived the equations, at least. Using "w" as "time step":
// y = w * v_y - sum(1..(w - 1))
// and then x has a bail-out condition but is otherwise same idea.
fn plot_y(v_y: i32, time: i32) -> i32 {
    // it's just that doing division with ints requires some concentration, that's all.
    time * v_y - (time * (time - 1))/2
}
fn plot_x(v_x: i32, time: i32) -> i32 {
    if time < v_x {
        plot_y(v_x, time)
    } else {
        // Once the velocity is spent on friction, X position stays the same
        // instead of reversing.
        plot_y(v_x, v_x)
    }
}

// Only returns the most negative Y coordinate.
fn parse_inputs_to_cheat_outrageously(inputs: &str) -> i32 {
    let results = parse_inputs(inputs);
    results[2]
}

fn parse_inputs(inputs: &str) -> Vec<i32> {
    let inputs = inputs.trim();
    let (_, inputs) = inputs.split_once("area: ").unwrap();
    let (x_stuff, y_stuff) = inputs.split_once(", ").unwrap();
    let (_, x_stuff) = x_stuff.split_once("=").unwrap();
    let (_, y_stuff) = y_stuff.split_once("=").unwrap();
    let (x1, x2) = x_stuff.split_once("..").unwrap();
    let (y1, y2) = y_stuff.split_once("..").unwrap();
    [x1, x2, y1, y2].iter().map(|v| i32::from_str_radix(*v, 10).unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "target area: x=20..30, y=-10..-5\n";

    #[test]
    fn example_part_one() {
        let answer = 45;
        let result = part_one(EXAMPLE);
        assert_eq!(result, answer);
    }

    #[test]
    fn example_part_two() {
        let answer = 112;
        let result = part_two(EXAMPLE);
        assert_eq!(result, answer);
    }

    #[test]
    fn rounding_plots() {
        assert_eq!(plot_y(9, 5), 35);
        assert_eq!(plot_y(9, 6), 39);
    }

    #[test]
    fn parseage() {
        assert_eq!(
            parse_inputs(EXAMPLE),
            vec![20, 30, -10, -5]
        );
    }

    #[test]
    fn dumpage() {
        // this isn't really a "test" per se, I just want to see some stuff.
        let the_stuff: Vec<_> = (1..50).map(|step| (
            step,
            plot_x(7, step),
            plot_y(9, step)
        )).collect();
        println!("{:?}", the_stuff);
    }
    // Oh hey!!! For v_y > 0, we always land ON y = zero on a discrete step, and
    // it's always step 2v_y+1! and on the step right after that, y is -(v_y+1).
    // So for the example target, for any v_y > 9, our first step will take us ≥
    // 11 units below 0, which sends us right past the lower bound of the target
    // area. And... I think it stands to reason that the biggest Y velocity we
    // can handle will be the biggest one that lands us on the bottom border of
    // the area... and the first question actually doesn't care about the X
    // velocity at all, does it? So... I think... do I even need a computer to
    // solve this one? We just need to find v_y where final_y = -(v_y+1), then
    // find the peak of trajectory for that v_y.

    // OK ok ok. so, part 2 is obnoxious enough that I don't feel bad about
    // cheesing part 1. Guess first question is, how do we find outer bounds
    // beyond which we don't bother checking anymore? And second question is,
    // within that barely-limited set, can we get away with brute forcing it?

    // I guess we found an outer limit already, on the Y side:
    // * Only care about y_v ≤ -final_y - 1.
    // Starting y_v can be negative too, so what's a lower bound for that? I
    // guess we can use "skip in one hop" for that too, so,
    // * Only care about y_v ≥ final_y.
    // Yikes, getting pretty big. Ah well. How about X? If the probe stops
    // before the zone starts, that's a loss, so that's a lower bound. And if
    // it skips the zone in the first step, that's an upper bound.
    // * Only about x_v where plot_x(x_v, x_v) ≥ first_x and plot_x(x_v, 1) ≤ final_x.
    // Mmmmm can we pre-process that at all? Not fully, but I think we can
    // reduce it to a cheap sequence. For example, consider the example, where
    // first_x = 20. For that lower bound, if you use the same value for v_x
    // and step, you can reduce the formula to (x^2 + x)/2 = x_pos, so we're
    // looking for the first v_x value where v_x^2 + v_x ≥ first_x.
}
