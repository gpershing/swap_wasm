#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
}

impl Color {
    pub const ALL: [Color; 6] = [
        Color::Red,
        Color::Orange,
        Color::Yellow,
        Color::Green,
        Color::Blue,
        Color::Purple,
    ];

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
}
