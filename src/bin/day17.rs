use advent21::*;

// The one with the trick shot.
fn main() {
    let inputs = load_inputs("day17").unwrap();
    part_one(&inputs);
    part_two(&inputs);
}

fn part_two(_inputs: &str) {}

// Return the highest Y position the probe can hit on a trajectory that will at
// some point be within the target area on a step.
fn part_one(inputs: &str) -> i32 {

    0
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
        let answer = ();
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

}
