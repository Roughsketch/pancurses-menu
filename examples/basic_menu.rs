extern crate pancurses;
extern crate pancurses_menu;

use pancurses::*;
use pancurses_menu::*;

fn main() {
    let window = initscr();

    if has_colors() {
        start_color();

        if can_change_color() {
            init_color(COLOR_BLACK, 0, 0, 0);
            init_color(COLOR_BLUE, 400, 400, 1000);
            init_color(COLOR_GREEN, 0, 1000, 0);
            init_color(COLOR_CYAN, 0, 1000, 1000);
            init_color(COLOR_RED, 1000, 0, 0);
            init_color(COLOR_MAGENTA, 1000, 0, 1000);
            init_color(COLOR_YELLOW, 1000, 1000, 0);
            init_color(COLOR_WHITE, 1000, 1000, 1000);
        }
    }

    print_center(&window, 0, "Basic Menu Example");

    let max_x = window.get_max_x();

    let menu_win = window.subwin(5, max_x / 2, 2, max_x / 4).unwrap();
    let mut menu = Menu::new(&menu_win, "Title");

    menu.boxed(true)
        .add_string("Choice 1")
        .add_string("Choice 2")
        .add_string("Choice 3")
        .add_string("Choice 4")
        .add_string("Choice 5")
        .add_string("Choice 6")
        .add_string("Choice 7")
        .add_string("Choice 8")
        .add_string("Choice 9")
        .add_string("Choice 10");

    
    menu.update();

    window.keypad(true);
    curs_set(0);

    loop {
        match window.getch() {
            Some(Input::Character('q')) => break,
            Some(Input::KeyDown) => menu.mvdown(),
            Some(Input::KeyUp) => menu.mvup(),
            Some(Input::Character(' ')) => menu.select(),
            _ => (),
        }
        menu.update();
    }
    
    endwin();
}

fn print_center<T: Into<String>>(window: &Window, row: i32, text: T) {
    let width = window.get_max_x();
    let msg = text.into();

    window.mvprintw(row, width / 2 - (msg.len() / 2) as i32, &msg);
}