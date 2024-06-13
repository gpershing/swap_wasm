use std::f32::consts::TAU;

use egui::{ahash::HashMap, Color32, Painter, Pos2, Rect, Rounding, Stroke, Vec2};

use crate::{gameplay::{Cell, Color, FColor}, grids::{Grid, GridIndex, Rotation}};

use super::palette::Palette;

pub struct BackgroundAnimation {
    data: HashMap<GridIndex, BackgroundData>
}

#[derive(Debug, Clone, Copy, Default)]
struct BackgroundData {
    hint_glow: f32,
    swap_glow: f32,
    stop_glow: f32,
    rotation_glow: f32,
    last_rotation_color: Option<Color>,
    last_direction: Option<f32>,
    rotation_t: f32
}

impl BackgroundAnimation {
    const SWAP_GLOW_LOSS: f32 = 2.0;
    const STOP_GLOW_LOSS: f32 = 2.0;
    const ROTATION_GLOW_LOSS: f32 = 2.0;
    const ROTATION_SPEED: f32 = 0.3;

    pub fn new(grid: &Grid<Cell>) -> Self {
        let data = grid.iter()
            .map(|(index, _cell)| (index, BackgroundData::default()))
            .collect();
        Self { data }
    }

    fn update_cell(data: &mut BackgroundData, cell: &Cell, dt: f32) {
        let stop = cell.has_color_in_any_layer(Color::STOP);
        data.swap_glow = if cell.has_color_in_any_layer(Color::SWAP) && !stop {
            (data.swap_glow + dt).min(1.0)
        }
        else {
            (data.swap_glow - dt * Self::SWAP_GLOW_LOSS).max(0.0)
        };
        data.stop_glow = if stop {
            (data.stop_glow + dt).min(1.0)
        }
        else {
            (data.stop_glow - dt * Self::STOP_GLOW_LOSS).max(0.0)
        };
        let rotation = if stop { Rotation::None } else { cell.rotation_for_fill() };
        data.rotation_glow = if rotation != Rotation::None {
            (data.rotation_glow + dt).min(1.0)
        }
        else {
            (data.rotation_glow - dt * Self::ROTATION_GLOW_LOSS).max(0.0)
        };
        data.last_rotation_color = match rotation {
            Rotation::CCW => Some(Color::CCW),
            Rotation::CW => Some(Color::CW),
            _ => data.last_rotation_color
        };
        let drotation = dt * Self::ROTATION_SPEED;
        let direction = if rotation == Rotation::CCW {
            Some(-1.0)
        }
        else if rotation == Rotation::CW {
            Some(1.0)
        }
        else if rotation == Rotation::None && data.rotation_t > drotation && data.rotation_t < 1.0 - drotation {
            data.last_direction
        }
        else {
            None
        };
        data.last_direction = direction;
        if let Some(direction) = direction {
            data.rotation_t += direction * drotation;
            while data.rotation_t >= 1.0 {
                data.rotation_t -= 1.0;
            }
            while data.rotation_t < 0.0 {
                data.rotation_t += 1.0;
            }
        }
        else {
            data.rotation_t = 0.0;
        }
    }

    pub fn draw_background_cell(&mut self, painter: &Painter, palette: &Palette, index: GridIndex, cell: &Cell, center: Pos2, scale: f32, show_hint: bool, dt: f32) {
        let data = self.data.get_mut(&index).unwrap();
        Self::update_cell(data, cell, dt);

        let mut stroke_color = FColor::rgb(0.0, 0.0, 0.0);
        let mut alpha = 0.0;
        if data.swap_glow > 0.001 {
            stroke_color = palette.get_fcolor(Color::SWAP) * data.swap_glow;
            alpha = 100.0 * data.swap_glow;
        }
        let stroke_color = stroke_color.to_color32_with_alpha(alpha as u8);

        painter.rect(
            Rect::from_center_size(center, Vec2::splat(scale * 0.95)),
            Rounding::same(scale * 0.05), palette.background, Stroke::new(data.swap_glow * scale * 0.02, stroke_color));
        
        if data.stop_glow > 0.001 {
            painter.rect_stroke(
                Rect::from_center_size(center, Vec2::splat(scale * 0.9)),
                Rounding::same(scale * 0.05),
                Stroke::new(data.stop_glow * scale * 0.03, palette.get(Color::STOP)));
        };

        if data.rotation_glow > 0.001 {
            let dot_count: usize = 4;
            let dot_angle_range: f32 = TAU / (dot_count as f32);

            let color = palette.get(data.last_rotation_color.unwrap_or(Color::CCW)).gamma_multiply(0.95 * data.rotation_glow * data.rotation_glow);
            for i in 0..dot_count {
                let theta = dot_angle_range * (i as f32 + data.rotation_t);
                let cos = theta.cos();
                let sin = theta.sin();
                let radius = f32::powf(cos.abs().powi(6) + sin.abs().powi(6), -0.16666666) * scale * 0.35;
                painter.circle_filled(center + Vec2::angled(theta) * radius, scale * 0.02, color);
            }
        }

        fn update_hint(hint_glow: &mut f32, painter: &Painter, palette: &Palette, center: Pos2, scale: f32, show_hint: bool, dt: f32) {
            if show_hint {
                *hint_glow = (*hint_glow + dt).min(1.0);
            }
            else {
                *hint_glow = (*hint_glow - dt * 5.0).max(0.0);
            }

            if *hint_glow > 0.001 {
                painter.rect(
                    Rect::from_center_size(center, Vec2::splat(scale * 0.94)),
                    Rounding::same(scale * 0.05), palette.get(Color::SWAP).gamma_multiply(*hint_glow * 0.33), Stroke::new(0.0, Color32::TRANSPARENT));
            }
        }
        update_hint(&mut data.hint_glow, painter, palette, center, scale, show_hint, dt);
    }
}