use egui::{emath::{self, RectTransform}, EventFilter, Modifiers, Pos2, Rect, Response, Sense, Ui, Vec2};

use crate::{gameplay::{PlayingPuzzle, PuzzleSolveState, SwapRecord}, grids::GridIndex};

use super::{background::BackgroundAnimation, cell::{draw_cell, CellDrawData}, palette, simulation::Simulation, swaps_left::{SwapsLeftAnimation, SwapsLeftDrawData}, SegmentMeshData};

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
            GameInput::Down(_) => false,
        }
    }

    pub fn clear(&mut self) {
        self.highlight = None;
        self.selected = None;
        self.input = GameInput::None;
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct GameStyle {
    pub scale: f32
}

#[derive(Debug, Clone, Copy)]
enum GameInput {
    None,
    Down(Option<GridIndex>),
    Drag(Option<GridIndex>),
}

impl GameInput {
    pub fn is_none(&self) -> bool {
        match self {
            GameInput::None => true,
            GameInput::Down(_) => false,
            GameInput::Drag(_) => false,
        }
    }
}

impl Default for GameInput {
    fn default() -> Self {
        GameInput::None
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
            GameInput::Down(_) => None,
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
            *input = GameInput::Down(get_grid_pos(ui, puzzle, to_game_coords));
            return GameInputResponse::Down(get_grid_pos(ui, puzzle, to_game_coords));
        }
        else {
            *input = GameInput::None;
        }
    }
    return GameInputResponse::None;
    
}

pub fn handle_events(ui: &Ui, puzzle: &mut PlayingPuzzle, state: &mut GameState) {
    let events = ui.input(|i| i.filtered_events(&EventFilter::default()));
    let did_undo = events.iter().any(|e| match e {
        egui::Event::Key {
            key: egui::Key::Z,
            pressed: true,
            modifiers,
            ..
        } if modifiers.matches_logically(Modifiers::COMMAND) => true,
        _ => false
    });
    if did_undo {
        if let Some(record) = puzzle.try_undo() {
            state.simulation.swap(SwapRecord { a: record.a, b: record.b, a_rotation: record.b_rotation.inverse(), b_rotation: record.a_rotation.inverse() });
            state.simulation.update_fill(puzzle.grid());
            state.input.clear();
            state.solved = puzzle.is_solved();
        }
    }
}

pub fn update_game(
    ui: &mut Ui,
    puzzle: &mut PlayingPuzzle,
    state: &mut GameState,
    style: &GameStyle,
    mesh_data: &SegmentMeshData) {
    ui.ctx().request_repaint();

    let dt = ui.input(|i| i.stable_dt);
    let palette = if ui.ctx().style().visuals.dark_mode {
        &palette::DARK
    }
    else {
        &palette::DEFAULT
    };

    let bounds = puzzle.size();
    let margin: egui::Margin = ui.style().spacing.window_margin;
    let game_size = Vec2::new(bounds.width as f32, bounds.height as f32) * style.scale;
    let game_size_with_margins = game_size + margin.sum();
    let (response, painter) = ui.allocate_painter(game_size_with_margins, Sense::click_and_drag());

    let game_rect = Rect::from_center_size(painter.clip_rect().center(), game_size);
    let game_coords = Rect::from_min_size(Pos2::new(-0.5, -0.5), Vec2 { x: bounds.width as f32, y: bounds.height as f32 });
    let to_screen = emath::RectTransform::from_to(game_coords, game_rect);
    let to_game_coords = to_screen.inverse();

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
        }
    }
    
    handle_events(ui, puzzle, state);

    state.animation_time += dt;
    state.simulation.step(dt, palette);
    if state.solved == PuzzleSolveState::Solved {
        state.simulation.step_solved(dt, puzzle.grid());
    }

    let swap_indicator_y = game_rect.bottom() + style.scale * 0.15;
    state.swaps_left_animation.draw(&painter, puzzle.swaps_made(), puzzle.swap_limit(), dt, SwapsLeftDrawData {
        size: style.scale,
        left_x: game_rect.left(),
        right_x: game_rect.right(),
        y: swap_indicator_y,
        palette
    });

    for (grid_pos, cell) in puzzle.iter_cells() {
        let center = to_screen * Pos2 { x: grid_pos.x as f32, y: grid_pos.y as f32 };
        state.backgound_animation.draw_background_cell(&painter, palette, grid_pos, cell, center, style.scale, dt);
    }

    for (grid_pos, cell) in puzzle.iter_cells() {
        let center = if state.input.is_dragging(grid_pos) {
            ui.ctx().pointer_interact_pos().unwrap_or(Pos2::ZERO)
        }
        else { to_screen * Pos2 { x: grid_pos.x as f32, y: grid_pos.y as f32 } };
        let size = if Some(grid_pos) == state.input.selected || Some(grid_pos) == state.input.highlight { style.scale * 0.85 } else { style.scale };
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
}