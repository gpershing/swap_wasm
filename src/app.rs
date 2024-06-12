use crate::{gameplay::{fallback_puzzle, PlayingPuzzle, Puzzle}, generator::{generate_puzzle, GeneratorSettings}, grids::GridSize, ux::{update_game, GameState, GameStyle, SegmentMeshData}};

pub struct App {
    puzzle: PlayingPuzzle,
    game_state: GameState,
    mesh_data: SegmentMeshData
}

const PUZZLE_KEY: &'static str = "swap_puzzle";

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        let puzzle = cc.storage
            .and_then(|storage| eframe::get_value(storage, &PUZZLE_KEY))
            .unwrap_or_else(|| PlayingPuzzle::play(fallback_puzzle()));
        let game_state = GameState::new(&puzzle);

        Self {
            puzzle,
            game_state,
            mesh_data: SegmentMeshData::init(0.03, 0.02, 0.04)
        }
    }
}

impl App {
    pub fn set_puzzle(&mut self, puzzle: Puzzle) {
        self.puzzle = PlayingPuzzle::play(puzzle);
        self.game_state = GameState::new(&self.puzzle);
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, PUZZLE_KEY, &self.puzzle);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }
                ui.menu_button("DEBUG", |ui| {
                    if ui.button("Generate").clicked() {
                        self.set_puzzle(generate_puzzle(&GeneratorSettings {
                            stop_sources: crate::generator::SourceSettings::Maybe,
                            rotator_sources: crate::generator::SourceSettings::Definitely,
                            size: GridSize { width: 4, height: 4 },
                            swap_count: 4,
                            max_intersections: 5,
                            intersection_chance: 0.25,
                            knockout_loop_chance: 0.9,
                            ..Default::default()
                        }));
                    }
                    if ui.button("Debug").clicked() {
                        self.set_puzzle(crate::gameplay::debug_puzzle::debug_puzzle());
                    }
                    if ui.button("Reset").clicked() {
                        self.set_puzzle(self.puzzle.puzzle().clone());
                    }
                });

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("swap");

            ui.centered_and_justified(|ui| {
                update_game(ui, &mut self.puzzle, &mut self.game_state, &GameStyle {
                    scale: 150.0
                }, &self.mesh_data);
            });
            // self.game.ui(ui);

            // ui.horizontal(|ui| {
            //     ui.label("Write something: ");
            //     ui.text_edit_singleline(&mut self.label);
            // });

            // ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            // if ui.button("Increment").clicked() {
            //     self.value += 1.0;
            // }

            // ui.separator();

            // ui.add(egui::github_link_file!(
            //     "https://github.com/emilk/eframe_template/blob/main/",
            //     "Source code."
            // ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
