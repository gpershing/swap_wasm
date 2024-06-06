use std::ops::{Add, AddAssign, Mul};

use egui::Color32;

#[derive(Debug, Clone, Copy)]
pub struct FColor([f32; 3]);

impl FColor {
    pub const fn rgb(r: f32, g: f32, b: f32) -> FColor {
        Self([r, g, b])
    }

    pub fn to_color32(self) -> Color32 {
        Color32::from_rgb(self.r() as u8, self.g() as u8, self.b() as u8)
    }

    pub fn r(&self) -> f32 { self.0[0].clamp(0.0, 255.0) }
    pub fn g(&self) -> f32 { self.0[1].clamp(0.0, 255.0) }
    pub fn b(&self) -> f32 { self.0[2].clamp(0.0, 255.0) }
}

impl Add for FColor {
    type Output = FColor;

    fn add(self, rhs: Self) -> Self::Output {
        Self([self.0[0] + rhs.0[0], self.0[1] + rhs.0[1], self.0[2] + rhs.0[2]])
    }
}

impl AddAssign for FColor {
    fn add_assign(&mut self, rhs: Self) {
        self.0[0] += rhs.0[0];
        self.0[1] += rhs.0[1];
        self.0[2] += rhs.0[2];
    }
}

impl Mul<f32> for FColor {
    type Output = FColor;

    fn mul(self, rhs: f32) -> Self::Output {
        Self([self.0[0] * rhs, self.0[1] * rhs, self.0[2] * rhs])
    }
}