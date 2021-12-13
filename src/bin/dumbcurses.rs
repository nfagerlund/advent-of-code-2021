use easycurses::*;

fn main() {
    hello_example();
}

fn hello_example() {
    let mut curse = EasyCurses::initialize_system().unwrap();
    curse.set_cursor_visibility(CursorVisibility::Invisible);
    curse.set_echo(false);
    curse.print("wassup");
    curse.refresh(); // call at end of draw cycle; sometimes needed sometimes not
    curse.get_input(); // stall until you press something, before exiting.
}
