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
}
