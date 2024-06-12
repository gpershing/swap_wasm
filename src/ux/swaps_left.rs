use std::f32::consts::TAU;

use egui::{epaint::CubicBezierShape, Color32, Painter, Pos2, Shape, Stroke, Vec2};

use crate::gameplay::Color;

use super::palette::Palette;

pub struct SwapsLeftAnimation {
    swaps_used: f32    
}

pub struct SwapsLeftDrawData<'a> {
    pub size: f32,
    pub left_x: f32,
    pub right_x: f32,
    pub y: f32,
    pub palette: &'a Palette
}

fn draw_swap_indicator_with_color(painter: &Painter, center: Pos2, t: f32, color: Color32, data: &SwapsLeftDrawData<'_>) {
    let closed = false;
    let fill = Color32::TRANSPARENT;
    let stroke = Stroke::new(0.015 * data.size, color);
    let max = if t >= 0.99 { 1 } else { 6 };
    painter.extend((0..max).map(|i| (i as f32 * (1.0 - t)) * TAU / 6.0 + TAU / 4.0)
        .map(|theta| {
            let end = center + Vec2::angled(theta) * data.size * 0.09;
            let tan = Vec2::angled(theta + TAU * 0.25) * data.size * 0.025;
            [
                Shape::CubicBezier(CubicBezierShape { points: [
                    center,
                    center + tan,
                    end + tan,
                    end
                ], closed, fill, stroke }),
                Shape::CubicBezier(CubicBezierShape { points: [
                    end,
                    end - tan,
                    center - tan,
                    center
                ], closed, fill, stroke })
            ]
        })
        .flatten());
}

fn draw_swap_indicator(painter: &Painter, center: Pos2, t: f32, data: &SwapsLeftDrawData<'_>) {
    draw_swap_indicator_with_color(painter, center, t, data.palette.empty, data);
    draw_swap_indicator_with_color(painter, center, t, data.palette.get(Color::Purple).gamma_multiply(1.0 - t), data);
}

impl SwapsLeftAnimation {
    pub fn new(swaps_made: usize) -> Self {
        Self { swaps_used: swaps_made as f32 }
    }
    
    pub fn draw(&mut self, painter: &Painter, swaps_made: usize, swap_limit: usize, dt: f32, data: SwapsLeftDrawData<'_>) {
        const SPEED: f32 = 1.0;
        const SPEED_UNDO: f32 = 3.0;

        let target = swaps_made as f32;
        if self.swaps_used < target {
            self.swaps_used = (self.swaps_used + dt * SPEED).min(target);
        }
        else {
            self.swaps_used = (self.swaps_used - dt * SPEED_UNDO).max(target);
        }

        for i in 0..swap_limit {
            let i_float = i as f32;
            let t = if self.swaps_used < i_float {
                0.0
            }
            else if self.swaps_used < i_float + 1.0 {
                self.swaps_used - i_float
            }
            else {
                1.0
            };

            let center_t = (i_float + 0.5) / (swap_limit as f32);
            let center = Pos2::new(data.right_x * center_t + data.left_x * (1.0 - center_t), data.y);

            draw_swap_indicator(painter, center, t, &data);
        }
    }
}