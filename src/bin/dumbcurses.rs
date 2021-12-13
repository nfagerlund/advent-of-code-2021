use easycurses::*;
// https://docs.rs/easycurses/0.13.0/easycurses/struct.EasyCurses.html

fn main() {
    hello_example();
}

// https://github.com/alisww/easycurses-rs/blob/master/examples/hello.rs
// ... ^^ this hasn't been updated since 2017 and is out of date w/ current impl.
// but still helpful!
fn hello_example() {
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
