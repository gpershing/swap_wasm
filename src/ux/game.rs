use egui::{emath::{self, RectTransform}, Button, EventFilter, Modifiers, Painter, Pos2, Rect, Response, Sense, Ui, Vec2};

use crate::{gameplay::{Color, PlayingPuzzle, PuzzleSolveState, SwapRecord}, grids::GridIndex};

use super::{background::{BackgroundAnimation, BackgroundAnimationDrawData}, cell::{draw_cell, CellDrawData}, palette::{self, Palette}, simulation::Simulation, swaps_left::{SwapsLeftAnimation, SwapsLeftDrawData}, SegmentMeshData};

pub enum GameCompletionAction {
    Reset,
    Next
}

pub struct GameState {
    input: GameInputState,
    solved: PuzzleSolveState,
    simulation: Simulation,
    backgound_animation: BackgroundAnimation,
    swaps_left_animation: SwapsLeftAnimation,
    animation_time: f32
}

impl GameState {
    pub fn new(puzzle: &PlayingPuzzle) -> Self {
        Self {
            input: GameInputState::new(),
            solved: puzzle.is_solved(),
            simulation: Simulation::new(puzzle.grid()),
            backgound_animation: BackgroundAnimation::new(puzzle.grid()),
            swaps_left_animation: SwapsLeftAnimation::new(puzzle.swaps_made()),
            animation_time: 0.0
        }
    }
}

pub struct GameInputState {
    input: GameInput,
    highlight: Option<GridIndex>,
    selected: Option<GridIndex>
}

impl GameInputState {
    pub fn new() -> Self {
        Self { input: GameInput::None, selected: None, highlight: None }
    }

    pub fn is_dragging(&self, index: GridIndex) -> bool {
        match self.input {
            GameInput::Drag(Some(drag)) => drag == index,
            GameInput::None => false,
            GameInput::Drag(None) => false,
            GameInput::Down => false,
        }
    }

    pub fn clear(&mut self) {
        self.highlight = None;
        self.selected = None;
        self.input = GameInput::None;
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct PuzzleState {
    pub hint_shown: bool,
    pub solved: bool
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct GameStyle {
    pub scale: f32
}

#[derive(Debug, Clone, Copy)]
enum GameInput {
    None,
    Down,
    Drag(Option<GridIndex>),
}

impl GameInput {
    pub fn is_none(&self) -> bool {
        match self {
            GameInput::None => true,
            GameInput::Down => false,
            GameInput::Drag(_) => false,
        }
    }
}

enum GameInputResponse {
    None,
    Down(Option<GridIndex>),
    Up(Option<GridIndex>),
    Drag(Option<GridIndex>),
    Drop(Option<GridIndex>, Option<GridIndex>),
}

fn update_input(input: &mut GameInput, ui: &Ui, response: Response, puzzle: &PlayingPuzzle, to_game_coords: &RectTransform) -> GameInputResponse {
    fn get_grid_pos(ui: &Ui, puzzle: &PlayingPuzzle, to_game_coords: &RectTransform) -> Option<GridIndex> {
        ui.ctx().pointer_interact_pos().and_then(|pos| {
            let game_coord_f = to_game_coords * pos;
            let game_coord_round = game_coord_f.round();
            if game_coord_round.x < 0.0 || game_coord_round.y < 0.0 {
                None
            }
            else {
                let game_coord = GridIndex::new(game_coord_round.x as usize, game_coord_round.y as usize);
                puzzle.index_has_cell(game_coord).then_some(game_coord)
            }
        })
    }

    if response.clicked() {
        *input = GameInput::None;
        return GameInputResponse::Up(get_grid_pos(ui, puzzle, to_game_coords))
    }

    if response.drag_stopped() {
        let dragging = match input {
            GameInput::None => None,
            GameInput::Down => None,
            GameInput::Drag(id) => *id
        };
        *input = GameInput::None;
        return GameInputResponse::Drop(dragging, get_grid_pos(ui, puzzle, to_game_coords));
    }

    if response.drag_started() {
        let input_id = get_grid_pos(ui, puzzle, to_game_coords);
        *input = GameInput::Drag(input_id);
        return GameInputResponse::Drag(input_id);
    }

    if input.is_none() {
        let down = response.contains_pointer() && ui.ctx().input(|i| i.pointer.primary_down());
        if down {
            *input = GameInput::Down;
            return GameInputResponse::Down(get_grid_pos(ui, puzzle, to_game_coords));
        }
        else {
            *input = GameInput::None;
        }
    }
    GameInputResponse::None
}

struct ControlsResponse {
    undo: bool,
    reset: bool,
    hint: bool,
    skip: bool
}

fn control_button(ui: &mut Ui, palette: &Palette, name: impl Into<String>, disabled: bool, highlight: bool) -> bool {
    let mut button = Button::new(egui::RichText::new(name).text_style(egui::TextStyle::Heading));
    if highlight {
        button = button.fill(palette.get(Color::SWAP).linear_multiply(0.33));
    }
    ui.vertical_centered_justified(move |ui|
        ui.add_enabled(!disabled, button)
    ).inner.clicked()
}

fn draw_controls(ui: &mut Ui, palette: &Palette, show_hint: bool, solved: bool) -> ControlsResponse {
    ui.columns(4, |columns| {
        let undo = control_button(&mut columns[0], palette, "Undo", false, false);
        let reset = control_button(&mut columns[1], palette, "Reset", false, false);
        let hint = control_button(&mut columns[2], palette, "Hint", show_hint, false);
        let skip = control_button(&mut columns[3], palette, if solved { "Next" } else { "Skip" }, false, solved);
        ControlsResponse {
            undo,
            reset,
            hint,
            skip
        }
    })
}

fn handle_events(ui: &Ui, controls: &mut ControlsResponse) {
    let events = ui.input(|i| i.filtered_events(&EventFilter::default()));
    controls.undo = controls.undo || events.iter().any(|e| matches!(e, egui::Event::Key {
        key: egui::Key::Z,
        pressed: true,
        modifiers,
        ..
    } if modifiers.matches_logically(Modifiers::COMMAND)));
}

fn handle_controls(controls: ControlsResponse, puzzle: &mut PlayingPuzzle, state: &mut GameState, puzzle_state: &mut PuzzleState) -> Option<GameCompletionAction> {
    if controls.undo {
        if let Some(record) = puzzle.try_undo() {
            state.simulation.swap(SwapRecord { a: record.a, b: record.b, a_rotation: record.b_rotation.inverse(), b_rotation: record.a_rotation.inverse() });
            state.simulation.update_fill(puzzle.grid());
            state.input.clear();
            state.solved = puzzle.is_solved();
            puzzle_state.solved = puzzle_state.solved || state.solved == PuzzleSolveState::Solved;
        }
    }
    if controls.hint {
        puzzle_state.hint_shown = true;
    }

    if controls.reset {
        Some(GameCompletionAction::Reset)
    }
    else if controls.skip {
        Some(GameCompletionAction::Next)
    }
    else {
        None
    }
}

pub fn update_game(
    ui: &mut Ui,
    puzzle: &mut PlayingPuzzle,
    state: &mut GameState,
    puzzle_state: &mut PuzzleState,
    style: &GameStyle,
    mesh_data: &SegmentMeshData
) -> Option<GameCompletionAction> {
    ui.ctx().request_repaint();

    const CONTROLS_HEIGHT: f32 = 50.0;
    const INDICATORS_HEIGHT: f32 = 50.0;
    const MAX_SIZE: f32 = 72.0;

    let (response, painter) = ui.allocate_painter(ui.available_size(), Sense::click_and_drag());
    
    let bounds = puzzle.size();
    let margin: egui::Margin = ui.style().spacing.window_margin;
    let max_size: Vec2 = painter.clip_rect().size();
    let game_width_max = max_size.x - margin.sum().x;
    let game_height_max = max_size.y - margin.sum().y - CONTROLS_HEIGHT - INDICATORS_HEIGHT;
    let cell_size = (game_width_max / bounds.width as f32).min(game_height_max / bounds.height as f32).min(style.scale).floor().max(MAX_SIZE);

    let center_x = painter.clip_rect().center().x;
    let game_size = Vec2::new(bounds.width as f32, bounds.height as f32) * cell_size;
    let controls_size = Vec2::new(game_size.x - ui.spacing().item_spacing.x, CONTROLS_HEIGHT);
    let controls_rect = Rect::from_center_size(Pos2 { x: center_x, y: painter.clip_rect().top() + controls_size.y * 0.5 }, controls_size);
    let game_rect = Rect::from_center_size(Pos2 { x: center_x, y: controls_rect.bottom() + game_size.y * 0.5 }, game_size);
    let indicators_size = Vec2::new(game_size.x, INDICATORS_HEIGHT);
    let indicators_rect = Rect::from_center_size(Pos2 { x: center_x, y: game_rect.bottom() + indicators_size.y * 0.5 }, indicators_size);

    update_game_after_sizing(ui, puzzle, state, puzzle_state, response, GameSizing { painter, cell_size, game_rect, controls_rect, indicators_rect }, mesh_data)
}

struct GameSizing {
    painter: Painter,
    cell_size: f32,
    game_rect: Rect,
    controls_rect: Rect,
    indicators_rect: Rect
}

fn update_game_after_sizing(
    ui: &mut Ui,
    puzzle: &mut PlayingPuzzle,
    state: &mut GameState,
    puzzle_state: &mut PuzzleState,
    response: Response,
    game_sizing: GameSizing,
    mesh_data: &SegmentMeshData
) -> Option<GameCompletionAction> {

    let dt = ui.input(|i| i.stable_dt);
    let palette = if ui.ctx().style().visuals.dark_mode {
        &palette::DARK
    }
    else {
        &palette::DEFAULT
    };

    let GameSizing { painter, cell_size, game_rect, controls_rect, indicators_rect } = game_sizing;
    let bounds = puzzle.size();

    let game_coords = Rect::from_min_size(Pos2::new(-0.5, -0.5), Vec2 { x: bounds.width as f32, y: bounds.height as f32 });
    let to_screen = emath::RectTransform::from_to(game_coords, game_rect);
    let to_game_coords = to_screen.inverse();

    let mut controls_response = ui.allocate_ui_at_rect(controls_rect, |ui|
        draw_controls(ui, palette, puzzle_state.hint_shown || puzzle.swaps_made() > 0, puzzle_state.solved)).inner;

    handle_events(ui, &mut controls_response);
    let completion_response = handle_controls(controls_response, puzzle, state, puzzle_state);

    let swap_action = match update_input(&mut state.input.input, ui, response, puzzle, &to_game_coords) {
        GameInputResponse::None => None,
        GameInputResponse::Down(id) => {
            state.input.highlight = id;
            None
        },
        GameInputResponse::Up(id) => {
            state.input.highlight = None;
            match state.input.selected.take() {
                Some(prev) => id.map(|id| (prev, id)),
                None => {
                    state.input.selected = id;
                    None
                },
            }
        },
        GameInputResponse::Drag(id) => {
            state.input.highlight = id;
            state.input.selected = None;
            None
        },
        GameInputResponse::Drop(prev, id) => {
            state.input.highlight = None;
            state.input.selected = None;
            prev.and_then(|prev| id.map(|id| (prev, id)))
        },
    };

    if let Some((a, b)) = swap_action {
        if let Some(record) = puzzle.try_swap(a, b) {
            state.simulation.swap(record);
            state.simulation.update_fill(puzzle.grid());
            state.solved = puzzle.is_solved();
            puzzle_state.solved = puzzle_state.solved || state.solved == PuzzleSolveState::Solved;
        }
    }

    state.animation_time += dt;
    state.simulation.step(dt, palette);
    if state.solved == PuzzleSolveState::Solved {
        state.simulation.step_solved(dt, puzzle.grid());
    }

    state.swaps_left_animation.draw(&painter, puzzle.swaps_made(), puzzle.swap_limit(), dt, SwapsLeftDrawData {
        rect: indicators_rect,
        palette
    });

    for (grid_pos, cell) in puzzle.iter_cells() {
        let center = to_screen * Pos2 { x: grid_pos.x as f32, y: grid_pos.y as f32 };
        let show_hint = puzzle_state.hint_shown && puzzle.puzzle().hint() == grid_pos && puzzle.swaps_made() == 0;
        state.backgound_animation.draw_background_cell(&painter, palette, cell, dt, BackgroundAnimationDrawData {
            index: grid_pos,
            center,
            scale: cell_size,
            show_hint, 
        });
    }

    for (grid_pos, cell) in puzzle.iter_cells() {
        let center = if state.input.is_dragging(grid_pos) {
            ui.ctx().pointer_interact_pos().unwrap_or(Pos2::ZERO)
        }
        else { to_screen * Pos2 { x: grid_pos.x as f32, y: grid_pos.y as f32 } };
        let size = if Some(grid_pos) == state.input.selected || Some(grid_pos) == state.input.highlight { cell_size * 0.85 } else { cell_size };
        draw_cell(cell, &painter, CellDrawData {
            index: grid_pos,
            center,
            size,
            mesh_data,
            simulation: &state.simulation,
            animation_t: state.animation_time,
            palette
        });
    }

    completion_response
}