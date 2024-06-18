use super::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ColorSet(u8);

impl ColorSet {
    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn singleton(color: Color) -> Self {
        Self(1 << color.bit())
    }

    pub const fn contains(&self, color: Color) -> bool {
        (self.0 & 1 << color.bit()) != 0
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, color: Color) {
        self.0 |= 1 << color.bit()
    }

    #[allow(dead_code)]
    pub fn remove(&mut self, color: Color) {
        self.0 &= (1u8 << color.bit()).reverse_bits()
    }

    pub fn union(self, other: ColorSet) -> ColorSet {
        ColorSet(self.0 | other.0)
    }

    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = Color> + '_ {
        Color::ALL.into_iter().filter(|c| self.contains(*c))
    }
}
