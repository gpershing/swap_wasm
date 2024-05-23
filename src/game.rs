use egui::{Color32, Painter, Pos2, Rect, Shape, Stroke, Ui, emath};

use crate::gameplay::{Cell, Puzzle};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Game {
    puzzle: Puzzle
}

impl Default for Game {
    fn default() -> Self {
        Self {
            puzzle: Puzzle::new()
        }
    }
}

impl Game {
    pub fn ui(&mut self, ui: &mut Ui) {
        let painter = Painter::new(
            ui.ctx().clone(),
            ui.layer_id(),
            ui.available_rect_before_wrap()
        );
        self.paint(&painter);
        ui.expand_to_include_rect(painter.clip_rect());
    }

    fn paint(&self, painter: &Painter) {
        let clip_rect = painter.clip_rect();
        let coords_size: egui::Vec2 = clip_rect.size();
        let coords_rect = Rect::from_min_size(Pos2::new(-coords_size.x / 2.0, 0.0), coords_size);
        // Transform from anchored coords to screen coords
        let to_screen = emath::RectTransform::from_to(coords_rect, clip_rect);

        fn paint_cell(painter: &Painter, to_screen: emath::RectTransform, pos: Pos2, cell: &Cell) {
            let center = Pos2::new(pos.x + 0.5, pos.y + 0.5) * 50.0;
            painter.extend(cell.connections().iter().map(|d| Shape::line_segment(
                [to_screen * center, to_screen * (center + d.to_vecf() * 25.0)],
                Stroke::new(1.0, Color32::from_rgb(255, 255, 255)))))
        }

        for (pos, cell) in self.puzzle.cells() {
            paint_cell(painter, to_screen, Pos2::new(pos.x as f32, pos.y as f32), cell)
        }
    }
}