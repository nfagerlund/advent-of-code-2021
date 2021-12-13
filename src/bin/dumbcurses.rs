use easycurses::*;
// https://docs.rs/easycurses/0.13.0/easycurses/struct.EasyCurses.html

fn main() {
    // hello_example();
    glow_example();
}

fn glow_example() {
    let mut curse = EasyCurses::initialize_system().unwrap();
    curse.set_cursor_visibility(CursorVisibility::Invisible);
    curse.set_echo(false);

    let lit = || ColorPair::new(Color::Cyan, Color::Black);
    let unlit = || ColorPair::new(Color::Blue, Color::Black);
    let line = vec![1, 1, 1, 0, 0, 1, 1, 1];
    for row in 0..9 {
        for (col, &val) in line.iter().enumerate() {
            curse.move_rc(row, col as i32);
            if val == 0 {
                curse.set_color_pair(lit());
            } else {
                curse.set_color_pair(unlit());
            }
            curse.print_char(std::char::from_digit(val, 10).unwrap());
        }
    }
    curse.refresh();
    curse.get_input();
}

// https://github.com/alisww/easycurses-rs/blob/master/examples/hello.rs
// ... ^^ this hasn't been updated since 2017 and is out of date w/ current impl.
// but still helpful!
fn _hello_example() {
    let mut curse = EasyCurses::initialize_system().unwrap();
    curse.set_cursor_visibility(CursorVisibility::Invisible);
    curse.set_echo(false);

    curse.set_color_pair(ColorPair::new(Color::Green, Color::Black));
    curse.set_bold(false);
    curse.clear();

    let content = "wassup";
    let (rows, cols) = curse.get_row_col_count();
    let center_row = rows/2; // as per math in the median thing from day10
    let start_col_for_centered_text = cols/2 - (content.len() as i32)/2;
    curse.move_rc(center_row, start_col_for_centered_text);
    // Oh! ^^ the _rc variants use graphics-like coords, where 0,0 => top left.
    // the _xy variants use cartesian coords where 0,0 => bottom left!
    curse.print("wassup");
    curse.refresh(); // call at end of draw cycle; sometimes needed sometimes not
    curse.get_input(); // stall until you press something, before exiting.
}
