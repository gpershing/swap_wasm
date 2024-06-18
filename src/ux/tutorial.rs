use egui::{Context, RichText, Sense, Ui, Vec2};

use crate::gameplay::Color;

use super::{cell::draw_source, palette};

pub fn tutorial_window(ctx: &Context, open: &mut bool) {
    egui::Window::new("How to Play")
        .resizable([true, true])
        .constrain(true)
        .collapsible(true)
        .title_bar(true)
        .scroll2([false, true])
        .enabled(true)
        .open(open)
        .show(ctx, draw_tutorial_window);
}

fn draw_tutorial_window(ui: &mut Ui) {
    let palette = if ui.ctx().style().visuals.dark_mode {
        &palette::DARK
    } else {
        &palette::DEFAULT
    };

    ui.heading("Solving a puzzle");
    ui.label("Complete the puzzle by swapping tiles until:");
    ui.label("- Every open path is connected to another path.");
    ui.label("- Every connected path is connected to at least one source tile.");
    ui.label("- No source tile is connected to a source of a different type.");
    ui.label("- All sources of the same type are connected.");
    ui.label("");

    ui.label("Each puzzle has a limit to the number of swaps, given by the flowers below the puzzle. You may continue swapping after you've hit the limit, but the puzzle won't be considered solved.");

    ui.label("");

    ui.heading("Source Effects");
    ui.label("Source tiles have a small icon in the center. Source tiles and all connected tiles have certain effects:");
    let grid_spacing = Vec2::new(
        ui.spacing().item_spacing.x * 0.25,
        ui.spacing().item_spacing.y,
    );
    egui::Grid::new("tutorial_effects_grid")
        .num_columns(2)
        .spacing(grid_spacing)
        .show(ui, |ui| {
            // let font_img_size = ui.fonts(|ft| ft.font_image_size());
            // let icon_size = Vec2::new(font_img_size[0] as f32, font_img_size[1] as f32);
            let text = RichText::new("a");
            let font_height = ui.fonts(|fonts| text.font_height(fonts, ui.style()));
            let icon_size = Vec2::splat(font_height) + ui.spacing().button_padding;

            ui.style_mut().wrap = Some(true);

            let (_, painter) = ui.allocate_painter(icon_size, Sense::focusable_noninteractive());
            draw_source(
                &painter,
                Color::SWAP,
                painter.clip_rect().center(),
                painter.clip_rect().size().min_elem() * 0.45,
                palette,
                0.0,
            );
            ui.label(
                "May be swapped. At least one tile in a swap must be connected to a swap source.",
            );
            ui.end_row();

            let (_, painter) = ui.allocate_painter(icon_size, Sense::focusable_noninteractive());
            draw_source(
                &painter,
                Color::Blue,
                painter.clip_rect().center(),
                painter.clip_rect().size().min_elem() * 0.45,
                palette,
                0.0,
            );
            ui.label("No effect.");
            ui.end_row();

            let (_, painter) = ui.allocate_painter(icon_size, Sense::focusable_noninteractive());
            draw_source(
                &painter,
                Color::Green,
                painter.clip_rect().center(),
                painter.clip_rect().size().min_elem() * 0.45,
                palette,
                0.0,
            );
            ui.label("No effect.");
            ui.end_row();

            let (_, painter) = ui.allocate_painter(icon_size, Sense::focusable_noninteractive());
            draw_source(
                &painter,
                Color::CW,
                painter.clip_rect().center(),
                painter.clip_rect().size().min_elem() * 0.45,
                palette,
                ui.input(|i| i.time) as f32,
            );
            ui.label("Rotates clockwise when swapped.");
            ui.end_row();

            let (_, painter) = ui.allocate_painter(icon_size, Sense::focusable_noninteractive());
            draw_source(
                &painter,
                Color::CCW,
                painter.clip_rect().center(),
                painter.clip_rect().size().min_elem() * 0.45,
                palette,
                ui.input(|i| i.time) as f32,
            );
            ui.label("Rotates counterclockwise when swapped.");
            ui.end_row();

            let (_, painter) = ui.allocate_painter(icon_size, Sense::focusable_noninteractive());
            draw_source(
                &painter,
                Color::STOP,
                painter.clip_rect().center(),
                painter.clip_rect().size().min_elem() * 0.45,
                palette,
                0.0,
            );
            ui.label("May never be swapped.");
            ui.end_row();
        });

    ui.label("");
    ui.heading("Intersections");
    ui.label("Some tiles may have two disjoint paths. In this case, the disjoint paths may be part of different regions. The tile will inheret effects from both of its paths, and the paths will swap as one.");

    ui.label("");
    ui.heading("Hints");
    ui.label("At the start of a puzzle, you may a request a hint. A tile will be highlighted that is part of the first swap.");
}
