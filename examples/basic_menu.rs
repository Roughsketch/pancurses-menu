extern crate pancurses;
extern crate pancurses_menu;

use pancurses::*;
use pancurses_menu::*;

fn main() {
    let window = initscr();

    noecho();
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

    let menu_win = window.subwin(5, max_x / 2 - 2, 2, 2).unwrap();
    let info_win = window.subwin(5, max_x / 2 - 2, 2, max_x / 2).unwrap();
    let mut menu = Menu::new(&menu_win, "Title");

    info_win.draw_box(0, 0);
    info_win.refresh();

    menu.boxed(true)
        .add_item("Choice 1")
        .add_item("Choice 2")
        .add_item("Choice 3")
        .add_item("Choice 4")
        .add_item("Choice 5")
        .add_item("Choice 6")
        .add_item("Choice 7")
        .add_item("Choice 8")
        .add_item("Choice 9")
        .add_item("Choice 10");

    
    menu.update();

    window.keypad(true);
    curs_set(0);

    loop {
        match window.getch() {
            Some(Input::Character('q')) => break,
            Some(Input::Character(' ')) => menu.select(),
            Some(Input::Character('\n')) => {
                let selected = menu.selected();

                match &*selected {
                    "Choice 1" => info_win.mvprintw(1, 1, "Choice 1 was selected"),
                    "Choice 2" => info_win.mvprintw(1, 1, "Choice 2 was selected"),
                    "Choice 3" => info_win.mvprintw(1, 1, "Choice 3 was selected"),
                    "Choice 4" => info_win.mvprintw(1, 1, "Choice 4 was selected"),
                    "Choice 5" => info_win.mvprintw(1, 1, "Choice 5 was selected"),
                    "Choice 6" => info_win.mvprintw(1, 1, "Choice 6 was selected"),
                    "Choice 7" => info_win.mvprintw(1, 1, "Choice 7 was selected"),
                    "Choice 8" => info_win.mvprintw(1, 1, "Choice 8 was selected"),
                    "Choice 9" => info_win.mvprintw(1, 1, "Choice 9 was selected"),
                    "Choice 10" => info_win.mvprintw(1, 1, "Choice 10 was selected"),
                    _ => info_win.mvprintw(1, 1, "Nothing is selected"),
                };

                info_win.refresh();
            }
            Some(Input::KeyDown) => menu.mvdown(),
            Some(Input::KeyUp) => menu.mvup(),
            Some(Input::KeyRight) => menu.mvpgdown(),
            Some(Input::KeyLeft) => menu.mvpgup(),
            Some(Input::KeyHome) => menu.mvtop(),
            Some(Input::KeyEnd) => menu.mvbot(),
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