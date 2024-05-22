use egui::{Color32, Painter, Pos2, Rect, Shape, Stroke, Ui, emath};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Game {

}

impl Default for Game {
    fn default() -> Self {
        Self {}
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

        painter.add(Shape::line_segment([to_screen * Pos2::ZERO, to_screen * Pos2::new(0.0, 50.0)], Stroke::new(1.0, Color32::from_rgb(255, 255, 255))));
    }
}