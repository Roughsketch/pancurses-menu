extern crate pancurses;

use pancurses::*;

pub mod action;
pub mod item;

pub use action::MenuAction;
pub use item::MenuItem;

pub struct Menu<'a> {
    title: String,
    items: Vec<MenuItem>,
    boxed: bool,
    multi_select: bool,
    window: &'a Window,
    selection: usize,
    top: usize,
}

impl<'a> Menu<'a> {
    pub fn new<T: Into<String>>(window: &Window, title: T) -> Menu {
        Menu {
            title: title.into(),
            items: Vec::new(),
            boxed: false,
            multi_select: false,
            window: window,
            selection: 0,
            top: 0,
        }
    }

    pub fn update(&self) {
        let mut offset = 0;
        self.window.mv(0, 0);
        self.window.clrtobot();

        if self.is_boxed() {
            self.window.draw_box(0, 0);
            offset += 1;
        }
        
        self.print_center(&self.window, 0, self.title.clone());

        for (idx, item) in self.items.iter().skip(self.top).take(self.max_shown() + 1).enumerate() {
            if idx == self.selection - self.top {
                self.window.mvprintw((idx + 1) as i32, offset, ">");
            }

            if item.selected() {
                self.window.mvprintw((idx + 1) as i32, offset + 1, &format!("* {}", item.desc()));
            } else {
                self.window.mvprintw((idx + 1) as i32, offset + 3, &item.desc());
            }
        }

        self.window.refresh();
    }

    pub fn max_shown(&self) -> usize {
        self.window.get_max_y() as usize - 2 * (self.is_boxed() as usize) - 1
    }

    pub fn boxed(&mut self, value: bool) -> &mut Self {
        self.boxed = value;
        self
    }

    pub fn is_boxed(&self) -> bool {
        self.boxed
    }

    pub fn multi(&mut self, value: bool) -> &mut Self {
        self.multi_select = value;
        self
    }

    pub fn is_multi(&self) -> bool {
        self.multi_select
    }

    pub fn selected(&self) -> String {
        self.items[self.selection].desc()
    }

    pub fn add_string<T: Into<String>>(&mut self, item: T) -> &mut Self {
        let mut item = MenuItem::new(item.into());

        if self.items.is_empty() {
            item.toggle();
        }

        self.items.push(item);
        self
    }

    pub fn add_item<T: Into<MenuItem>>(&mut self, item: T) -> &Self {
        self.items.push(item.into());
        self
    }

    pub fn mvdown(&mut self) {
        if self.selection < usize::max_value() && self.selection + 1 < self.items.len(){
            self.selection += 1;
        }

        self.inc(1 as usize);
    }

    pub fn mvup(&mut self) {
        if self.selection > usize::min_value() {
            self.selection -= 1;
        }

        self.dec(1 as usize);
    }

    pub fn mvpgdown(&mut self) {
        let max = self.max_shown() + 1;
        self.selection = usize::min(self.items.len() - 1, self.selection + max);
        self.inc(max);
    }

    pub fn mvpgup(&mut self) {
        let max = self.max_shown() + 1;
        self.dec(max);
        if max > self.selection {
            self.selection = 0;
        } else {
            self.selection -= max;
        }
    }

    pub fn mvtop(&mut self) {
        self.top = 0;
        self.selection = 0;
    }

    pub fn mvbot(&mut self) {
        if self.items.len() > self.max_shown() {
            self.top = self.items.len() - self.max_shown() - 1;
        }

        self.selection = self.items.len() - 1;
    }

    pub fn select(&mut self) {
        if !self.is_multi() {
            for item in self.items.iter_mut() {
                if item.selected() {
                    item.toggle();
                }
            }
        }

        if let Some(item) = self.items.get_mut(self.selection) {
            item.toggle();
        }
    }

    fn inc<T: Into<usize>>(&mut self, amount: T) {
        let max = self.max_shown();
        let item_len = self.items.len();

        if  self.selection - self.top > max &&
            max < item_len &&
            self.top < item_len - max {
            self.top = usize::min(item_len - max - 1, self.top + amount.into());
        }
    }

    fn dec<T: Into<usize>>(&mut self, amount: T) {
        let val = amount.into();
        if  self.selection == self.top && self.top > 0 {
            if val >= self.top {
                self.top = 0;
            } else {
                self.top -= val;
            }
        }
    }

    fn print_center<I: Into<i32>, T: Into<String>>(&self, window: &Window, row: I, text: T) {
        let max_x = window.get_max_x();
        let string = text.into();

        window.mvprintw(row.into(), max_x / 2 - (string.len() / 2) as i32, &string);
    }
}