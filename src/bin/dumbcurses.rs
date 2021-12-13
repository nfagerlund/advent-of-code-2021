use easycurses::*;

fn main() {
    hello_example();
}

fn hello_example() {
    let mut curse = EasyCurses::initialize_system().unwrap();
    curse.set_cursor_visibility(CursorVisibility::Invisible);
    curse.set_echo(false);
    let content = "wassup";
    let (rows, cols) = curse.get_row_col_count();
    let center_row = rows/2; // as per math in the median thing from day10
    let start_col_for_centered_text = cols/2 - (content.len() as i32)/2;
    curse.move_rc(center_row, start_col_for_centered_text);
    curse.print("wassup");
    curse.refresh(); // call at end of draw cycle; sometimes needed sometimes not
    curse.get_input(); // stall until you press something, before exiting.
}
