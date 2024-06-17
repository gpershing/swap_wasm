use egui::{Context, Slider, Ui};

use crate::generator::{GeneratorSettings, SourceSettings};

pub fn edit_generator_settings(ctx: &Context, use_settings: &mut bool, settings: &mut GeneratorSettings, open: &mut bool) {
    egui::Window::new("Generator settings")
        .resizable([true, true])
        .constrain(true)
        .collapsible(true)
        .title_bar(true)
        .scroll2([false, true])
        .enabled(true)
        .open(open)
        .show(ctx, |ui| {
            ui.checkbox(use_settings, "Use custom generator");
            ui.separator();
            ui.add_enabled_ui(*use_settings, |ui| {
                egui::Grid::new("generator_settings_grid")
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| edit_generator_settings_grid(ui, settings));
                ui.separator();
                ui.collapsing("Advanced", |ui| {
                    egui::Grid::new("generator_settings_advanced_grid")
                        .num_columns(2)
                        .striped(true)
                        .show(ui, |ui| edit_advanced_settings_grid(ui, settings));
                });
            });
        });
}

fn edit_generator_settings_grid(ui: &mut Ui, settings: &mut GeneratorSettings) {
    ui.label("Swap count");
    ui.add(Slider::new(&mut settings.swap_count, 1..=6));
    ui.end_row();

    let min_width = if settings.size.height == 1 { 2 } else { 1 };
    ui.label("Width");
    ui.add(Slider::new(&mut settings.size.width, min_width..=6));
    ui.end_row();

    ui.label("Height");
    ui.add(Slider::new(&mut settings.size.height, 1..=6));
    ui.end_row();

    ui.label("Include stoppers");
    ui.horizontal(|ui| {
        ui.radio_value(&mut settings.stop_sources, SourceSettings::None, "No");
        ui.radio_value(&mut settings.stop_sources, SourceSettings::Maybe, "Maybe");
        ui.radio_value(&mut settings.stop_sources, SourceSettings::Definitely, "Always");
    });
    ui.end_row();

    ui.label("Include rotators");
    ui.horizontal(|ui| {
        ui.radio_value(&mut settings.rotator_sources, SourceSettings::None, "No");
        ui.radio_value(&mut settings.rotator_sources, SourceSettings::Maybe, "Maybe");
        ui.radio_value(&mut settings.rotator_sources, SourceSettings::Definitely, "Always");
    });
    ui.end_row();

    ui.label("Minimum regions");
    let mut total_regions = settings.min_regions + 1;
    ui.add(Slider::new(&mut total_regions, 1..=6));
    settings.min_regions = total_regions - 1;
    ui.end_row();
}

fn edit_advanced_settings_grid(ui: &mut Ui, settings: &mut GeneratorSettings) {
    ui.label("Missing chance").on_hover_text("Chance that a cell in the grid will have nothing.");
    ui.add(Slider::new(&mut settings.missing_chance, 0.0..=1.0));
    ui.end_row();

    ui.label("Maximum missing").on_hover_text("Maximum number of missing tiles.");
    ui.add(Slider::new(&mut settings.missing, 0..=4));
    ui.end_row();

    ui.label("Extra region chance").on_hover_text("Chance that additional regions will be added beyond the minimum.");
    ui.add(Slider::new(&mut settings.extra_region_chance, 0.0..=1.0));
    ui.end_row();

    ui.label("Extra source chance").on_hover_text("Chance that a region will have more than one source tile.");
    ui.add(Slider::new(&mut settings.extra_source_chance, 0.0..=1.0));
    ui.end_row();

    ui.label("Intersection chance").on_hover_text("Chance that any attempt to make an intersection tile will succeed.");
    ui.add(Slider::new(&mut settings.intersection_chance, 0.0..=1.0));
    ui.end_row();

    ui.label("Maximum intersections").on_hover_text("Maximum number of intersection tiles.");
    ui.add(Slider::new(&mut settings.max_intersections, 0..=5));
    ui.end_row();

    ui.label("Knockout loop chance").on_hover_text("Chance that any loop will be removed from a region.");
    ui.add(Slider::new(&mut settings.knockout_loop_chance, 0.0..=1.0));
    ui.end_row();

    ui.label("Check solution depth").on_hover_text("How deep to explore when looking for a possible shorter solution.");
    ui.add(Slider::new(&mut settings.check_solution_len, 0..=(settings.swap_count as usize - 1)));
    ui.end_row();

    ui.label("Check solution retries").on_hover_text("If a shorter solution is found, how many times to retry before giving up.");
    ui.add(Slider::new(&mut settings.check_solution_retries, 1..=5));
    ui.end_row();
}