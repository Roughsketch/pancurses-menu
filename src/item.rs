use std::str::FromStr;
use std::string::ParseError;

pub struct MenuItem {
    desc: String,
    selected: bool,
}

impl MenuItem {
    pub fn new<T: Into<String>>(desc: T) -> MenuItem {
        MenuItem {
            desc: desc.into(),
            selected: false,
        }
    }

    pub fn desc(&self) -> String {
        self.desc.clone()
    }

    pub fn toggle(&mut self) {
        self.selected = !self.selected;
    }

    pub fn selected(&self) -> bool {
        self.selected
    }
}

impl FromStr for MenuItem {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MenuItem::new(s))
    }
}