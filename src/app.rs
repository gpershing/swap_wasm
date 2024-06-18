use crate::{
    gameplay::{fallback_puzzle, PlayingPuzzle, Puzzle},
    generator::generate_puzzle,
    ux::{
        edit_generator_settings, tutorial_window, update_game, GameState, GameStyle, PuzzleState,
        SegmentMeshData, SettingsConfig,
    },
};

pub struct App {
    puzzle: PlayingPuzzle,
    puzzle_state: PuzzleState,
    game_state: GameState,
    mesh_data: SegmentMeshData,

    config: SettingsConfig,

    editing_generator_settings: bool,
    showing_tutorial: bool,
}

const PUZZLE_KEY: &str = "swap_puzzle";
const PUZZLE_STATE_KEY: &str = "swap_puzzle_state";
const SETTINGS_KEY: &str = "swap_settings";

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let puzzle = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, PUZZLE_KEY))
            .unwrap_or_else(|| PlayingPuzzle::play(fallback_puzzle()));
        let puzzle_state = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, PUZZLE_STATE_KEY))
            .unwrap_or_default();
        let game_state = GameState::new(&puzzle);

        let config = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, SETTINGS_KEY))
            .unwrap_or_default();

        Self {
            puzzle,
            puzzle_state,
            game_state,
            mesh_data: SegmentMeshData::init(0.03, 0.02, 0.04),
            config,
            editing_generator_settings: false,
            showing_tutorial: false,
        }
    }
}

impl App {
    fn set_puzzle_without_puzzle_state(&mut self, puzzle: Puzzle) {
        self.puzzle = PlayingPuzzle::play(puzzle);
        self.game_state = GameState::new(&self.puzzle);
    }

    pub fn reset_puzzle(&mut self) {
        self.set_puzzle_without_puzzle_state(self.puzzle.puzzle().clone());
    }

    pub fn set_puzzle(&mut self, puzzle: Puzzle) {
        self.set_puzzle_without_puzzle_state(puzzle);
        self.puzzle_state = PuzzleState::default();
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, PUZZLE_KEY, &self.puzzle);
        eframe::set_value(storage, PUZZLE_STATE_KEY, &self.puzzle_state);
        eframe::set_value(storage, SETTINGS_KEY, &self.config);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                }

                if ui.button("How to play...").clicked() {
                    self.showing_tutorial = true;
                }
                if ui.button("Generator...").clicked() {
                    self.editing_generator_settings = true;
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                let response = update_game(
                    ui,
                    &mut self.puzzle,
                    &mut self.game_state,
                    &mut self.puzzle_state,
                    &GameStyle { scale: 150.0 },
                    &self.mesh_data,
                );
                if let Some(response) = response {
                    match response {
                        crate::ux::GameCompletionAction::Reset => self.reset_puzzle(),
                        crate::ux::GameCompletionAction::Skip
                        | crate::ux::GameCompletionAction::Solved => {
                            self.set_puzzle(generate_puzzle(&self.config.get_current_settings()));
                        }
                    }
                }
            });

            edit_generator_settings(
                ctx,
                &mut self.config.custom_override,
                &mut self.config.custom_settings,
                &mut self.editing_generator_settings,
            );

            tutorial_window(ctx, &mut self.showing_tutorial);

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
        ui.label(". View the ");
        ui.hyperlink_to("source", "https://github.com/gpershing/swap_wasm");
        ui.label(".");
    });
}
