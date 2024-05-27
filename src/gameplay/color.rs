use egui::Color32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}