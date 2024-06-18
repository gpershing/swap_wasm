use egui::Color32;

use crate::gameplay::{Color, FColor};

pub const DEFAULT: Palette = Palette {
    background: Color32::from_rgb(237, 234, 221),

    empty: Color32::from_rgb(133, 124, 94),
    red: Color32::from_rgb(166, 30, 20),
    orange: Color32::from_rgb(201, 131, 24),
    yellow: Color32::from_rgb(212, 185, 53),
    green: Color32::from_rgb(58, 140, 66),
    blue: Color32::from_rgb(57, 81, 143),
    purple: Color32::from_rgb(102, 44, 138),
};

pub const DARK: Palette = Palette {
    background: Color32::from_rgb(61, 60, 55),

    empty: Color32::from_rgb(161, 160, 157),
    red: Color32::from_rgb(194, 40, 23),
    orange: Color32::from_rgb(209, 138, 23),
    yellow: Color32::from_rgb(214, 211, 17),
    green: Color32::from_rgb(25, 186, 17),
    blue: Color32::from_rgb(17, 141, 186),
    purple: Color32::from_rgb(182, 15, 219),
};

pub struct Palette {
    pub background: Color32,

    pub empty: Color32,
    pub red: Color32,
    pub orange: Color32,
    pub yellow: Color32,
    pub green: Color32,
    pub blue: Color32,
    pub purple: Color32,
}

impl Palette {
    pub const fn get(&self, color: Color) -> Color32 {
        match color {
            Color::Red => self.red,
            Color::Orange => self.orange,
            Color::Yellow => self.yellow,
            Color::Green => self.green,
            Color::Blue => self.blue,
            Color::Purple => self.purple,
        }
    }

    pub const fn empty_f(&self) -> FColor {
        FColor::from_color32(self.empty)
    }

    pub const fn get_fcolor(&self, color: Color) -> FColor {
        FColor::from_color32(self.get(color))
    }
}
