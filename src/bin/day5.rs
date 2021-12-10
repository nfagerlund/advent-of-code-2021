use advent21::*;
use std::collections::HashMap;

fn main() {
    let inputs = load_inputs("day5").unwrap();
    part_one(&inputs);
    part_two(&inputs);
}

fn part_two(inputs: &str) -> usize {
    let mut points_on_lines: HashMap<Point, usize> = HashMap::new();
    let lines = parse_inputs(inputs);
    for line in lines {
        for point in line.points_on_line() {
            let count = points_on_lines.entry(point).or_insert(0);
            *count += 1;
        }
    }
    let number_of_intersections: usize = points_on_lines.iter()
        .filter(|(_point, count)| **count > 1).count();
    println!("Number of intersections (horizontal/vertical lines only):\n{}", number_of_intersections);
    number_of_intersections
}

fn part_one(inputs: &str) -> usize {
    let mut points_on_lines: HashMap<Point, usize> = HashMap::new();
    let lines = parse_inputs(inputs);
    for line in lines {
        for point in line.points_on_line_no_diags() {
            let count = points_on_lines.entry(point).or_insert(0);
            *count += 1;
        }
    }
    let number_of_intersections: usize = points_on_lines.iter()
        .filter(|(_point, count)| **count > 1).count();
    println!("Number of intersections (horizontal/vertical lines only):\n{}", number_of_intersections);
    number_of_intersections
}

fn parse_inputs(inputs: &str) -> Vec<Line> {
    inputs.lines().map(|line| Line::parse(line)).collect()
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Line {
        Line { start, end }
    }

    //8,0 -> 0,8
    fn parse(line_str: &str) -> Line {
        let (start_str, end_str) = line_str.split_once(" -> ").unwrap();
        Line::new(
            Point::parse(start_str),
            Point::parse(end_str),
        )
    }

    fn points_on_line(&self) -> Vec<Point> {
        self.points_on_line_yes_diags()
    }

    // Diags are only ever 45 degrees. Get to be nice and lazy!!
    fn points_on_line_yes_diags(&self) -> Vec<Point> {
        // ...wait. I think this might be easier than the first version. Can I
        // just......
        let mut points = Vec::new();
        for x in bidirectional_inclusive_range(self.start.x, self.end.x) {
            for y in bidirectional_inclusive_range(self.start.y, self.end.y) {
                points.push(Point::new(x, y));
            }
        }
        points
    }

    fn points_on_line_no_diags(&self) -> Vec<Point> {
        if self.start.x == self.end.x {
            bidirectional_inclusive_range(self.start.y, self.end.y)
                .map(|y| Point::new(self.start.x, y)).collect()
        } else if self.start.y == self.end.y {
            bidirectional_inclusive_range(self.start.x, self.end.x)
                .map(|x| Point::new(x, self.start.y)).collect()
        } else {
            Vec::new()
        }
    }
}

// Returns the same range when given 9,18 or 18,9.
fn bidirectional_inclusive_range(one: usize, two: usize) -> std::ops::RangeInclusive<usize> {
    let mut inputs = [one, two];
    inputs.sort();
    inputs[0]..=inputs[1]
}

#[derive(PartialEq, Eq, Hash)]
struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    // "8,0"
    fn parse(point_str: &str) -> Point {
        let (x_str, y_str) = point_str.split_once(',').unwrap();
        Point::new(usize_or_die(x_str), usize_or_die(y_str))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"#;

    #[test]
    fn example_part_one() {
        let answer = 5;
        let result = part_one(EXAMPLE);
        assert_eq!(result, answer);
    }

    #[test]
    fn example_part_two() {
        let answer = 12;
        let result = part_two(EXAMPLE);
        assert_eq!(result, answer);
    }
}
