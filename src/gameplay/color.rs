use egui::Color32;

use super::FColor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple
}

impl Color {
    pub const ALL: [Color; 6] = [Color::Red, Color::Orange, Color::Yellow, Color::Green, Color::Blue, Color::Purple];

    pub const SWAP: Color = Color::Purple;
    pub const STOP: Color = Color::Red;
    pub const CCW: Color = Color::Orange;
    pub const CW: Color = Color::Yellow;

    pub const fn bit(self) -> u8 {
        match self {
            Color::Red => 0,
            Color::Orange => 1,
            Color::Yellow => 2,
            Color::Green => 3,
            Color::Blue => 4,
            Color::Purple => 5,
        }
    }

    pub const fn index(self) -> usize {
        match self {
            Color::Red => 0,
            Color::Orange => 1,
            Color::Yellow => 2,
            Color::Green => 3,
            Color::Blue => 4,
            Color::Purple => 5,
        }
    }

    pub const fn color32(self) -> Color32 {
        match self {
            Color::Red => Color32::from_rgb(255, 0, 0),
            Color::Orange => Color32::from_rgb(255, 125, 0),
            Color::Yellow => Color32::from_rgb(255, 255, 0),
            Color::Green => Color32::from_rgb(0, 255, 0),
            Color::Blue => Color32::from_rgb(0, 0, 255),
            Color::Purple => Color32::from_rgb(255, 0, 255),
        }
    }
    
    pub const fn fcolor(self) -> FColor {
        match self {
            Color::Red => FColor::rgb(255.0, 0.0, 0.0),
            Color::Orange => FColor::rgb(255.0, 125.0, 0.0),
            Color::Yellow => FColor::rgb(255.0, 255.0, 0.0),
            Color::Green => FColor::rgb(0.0, 255.0, 0.0),
            Color::Blue => FColor::rgb(0.0, 0.0, 255.0),
            Color::Purple => FColor::rgb(255.0, 0.0, 255.0),
        }
    }
}