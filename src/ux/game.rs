use egui::{emath::{self, RectTransform}, Color32, EventFilter, Id, Modifiers, Painter, Pos2, Rect, Response, Sense, Shape, Stroke, Ui, Vec2, Widget};

use crate::{gameplay::{Cell, Puzzle, SwapRecord}, grid_math};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct GameState {
    #[serde(skip)]
    input: GameInput,
    #[serde(skip)]
    highlight: Option<usize>,
    #[serde(skip)]
    selected: Option<usize>,

    history: Vec<SwapRecord>
}

impl GameState {
    pub fn new() -> GameState {
        GameState { input: GameInput::None, selected: None, highlight: None, history: Vec::new() }
    }

    pub fn is_dragging(&self, id: usize) -> bool {
        match self.input {
            GameInput::Drag(Some(drag)) => drag == id,
            GameInput::None => false,
            GameInput::Drag(None) => false,
            GameInput::Down(_) => false,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct GameStyle {
    pub scale: f32
}

#[derive(Debug, Clone, Copy)]
enum GameInput {
    None,
    Down(Option<usize>),
    Drag(Option<usize>),
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
    Down(Option<usize>),
    Up(Option<usize>),
    Drag(Option<usize>),
    Drop(Option<usize>, Option<usize>),
}

fn update_input(input: &mut GameInput, ui: &Ui, response: Response, puzzle: &Puzzle, to_game_coords: &RectTransform) -> GameInputResponse {
    fn get_input_id(ui: &Ui, puzzle: &Puzzle, to_game_coords: &RectTransform) -> Option<usize> {
        ui.ctx().pointer_interact_pos().and_then(|pos| {
            let game_coord_f = to_game_coords * pos;
            let game_coord = grid_math::Pos2::new(game_coord_f.x.round() as i8, game_coord_f.y.round() as i8);
            puzzle.get(game_coord).map(|cell| cell.id())
        })
    }

    if response.clicked() {
        *input = GameInput::None;
        return GameInputResponse::Up(get_input_id(ui, puzzle, to_game_coords))
    }

    if response.drag_stopped() {
        let dragging = match input {
            GameInput::None => None,
            GameInput::Down(_) => None,
            GameInput::Drag(id) => *id
        };
        *input = GameInput::None;
        return GameInputResponse::Drop(dragging, get_input_id(ui, puzzle, to_game_coords));
    }

    if response.drag_started() {
        let input_id = get_input_id(ui, puzzle, to_game_coords);
        *input = GameInput::Drag(input_id);
        return GameInputResponse::Drag(input_id);
    }

    if input.is_none() {
        let down = response.contains_pointer() && ui.ctx().input(|i| i.pointer.primary_down());
        if down {
            *input = GameInput::Down(get_input_id(ui, puzzle, to_game_coords));
            return GameInputResponse::Down(get_input_id(ui, puzzle, to_game_coords));
        }
        else {
            *input = GameInput::None;
        }
    }
    return GameInputResponse::None;
    
}

pub fn undo(puzzle: &mut Puzzle, state: &mut GameState) -> bool {
    if let Some(record) = state.history.pop() {
        puzzle.try_undo_swap(record)
    }
    else {
        false
    }
}

pub fn handle_events(ui: &Ui, puzzle: &mut Puzzle, state: &mut GameState) {
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
        undo(puzzle, state);
    }
}

pub fn update_game(
    ui: &mut Ui,
    puzzle: &mut Puzzle,
    state: &mut GameState,
    style: &GameStyle) {
    let bounds = puzzle.bounds();
    let margin: egui::Margin = ui.style().spacing.window_margin;
    let game_size = bounds.size.to_vecf() * style.scale + margin.sum();
    let (response, painter) = ui.allocate_painter(game_size, Sense::click_and_drag());

    let game_rect = Rect::from_center_size(painter.clip_rect().center(), game_size);
    let game_coords = Rect::from_min_size(bounds.origin.to_posf() - Vec2::splat(0.5), bounds.size.to_vecf());
    let to_screen = emath::RectTransform::from_to(game_coords, game_rect);
    let to_game_coords = to_screen.inverse();

    let swap_action = match update_input(&mut state.input, ui, response, puzzle, &to_game_coords) {
        GameInputResponse::None => None,
        GameInputResponse::Down(id) => {
            state.highlight = id;
            None
        },
        GameInputResponse::Up(id) => {
            state.highlight = None;
            match state.selected.take() {
                Some(prev) => id.map(|id| (prev, id)),
                None => {
                    state.selected = id;
                    None
                },
            }
        },
        GameInputResponse::Drag(id) => {
            state.highlight = id;
            state.selected = None;
            None
        },
        GameInputResponse::Drop(prev, id) => {
            state.highlight = None;
            state.selected = None;
            prev.and_then(|prev| id.map(|id| (prev, id)))
        },
    };

    if let Some((id_a, id_b)) = swap_action {
        if let Some(record) = puzzle.try_swap(id_a, id_b) {
            state.history.push(record);
            println!("{:?}", puzzle.is_solved());
        }
    }
    
    handle_events(ui, puzzle, state);

    for (grid_pos, cell) in puzzle.cells() {
        let center = if state.is_dragging(cell.id()) {
            ui.ctx().pointer_interact_pos().unwrap_or(Pos2::ZERO)
        }
        else { to_screen * grid_pos.to_posf() };
        let size = if Some(cell.id()) == state.selected || Some(cell.id()) == state.highlight { style.scale * 0.9 } else { style.scale };
        painter.extend(cell.connections().iter().map(|d| Shape::line_segment(
            [center, center + d.to_vecf() * 0.5 * size],
            Stroke::new(
                1.0,
                cell.fill().iter().next().map(|c| c.color32()).unwrap_or(Color32::WHITE)
            ))));
        if let Some(source) = cell.source() {
            painter.circle_stroke(center, style.scale * 0.2, Stroke::new(1.0, source.color32()));
        }
    }
}

// impl Default for Game {
//     fn default() -> Self {
//         Self {
//             puzzle: Puzzle::new(),
//             scale: 50.0
//         }
//     }
// }

// impl Game {
//     fn draw_cell(&self, painter: &Painter, to_screen: &RectTransform, pos: grid_math::Pos2, cell: &Cell) {
//         let center = to_screen * pos.to_posf();
//         painter.extend(cell.connections().iter().map(|d| Shape::line_segment(
//             [center, center + d.to_vecf() * 0.5 * self.scale],
//         Stroke::new(1.0, Color32::from_rgb(255, 255, 255)))));
//     }
// }

// impl Widget for Game {
//     fn ui(self, ui: &mut Ui) -> egui::Response {
//         let bounds = self.puzzle.bounds();
//         let margin = ui.style().spacing.window_margin;
//         let game_size = bounds.size.to_vecf() * self.scale + margin.sum();
//         let (response, painter) = ui.allocate_painter(game_size, Sense::click_and_drag());

//         let game_rect = Rect::from_center_size(painter.clip_rect().center(), game_size);
//         let game_coords = Rect::from_min_size(bounds.origin.to_posf() - Vec2::splat(0.5), bounds.size.to_vecf());
//         let to_screen = emath::RectTransform::from_to(game_coords, game_rect);
//         let to_game_coords = to_screen.inverse();

//         for (pos, cell) in self.puzzle.cells() {
//             self.draw_cell(&painter, &to_screen, *pos, cell);
//         }

//         let clicked_immediate = response.contains_pointer() && ui.ctx().input(|i| i.pointer.primary_down());

//         if clicked_immediate {
//             println!("click now");
//         }

//         if response.clicked() {
//             println!("clicked!");
//             if let Some(pos) = ui.ctx().pointer_interact_pos() {
//                 let game_coord = to_game_coords * pos;
//                 println!("{game_coord:?}");
//             }
//         }

//         if response.drag_started() {
//             println!("dragged!");
//         }
        
//         response
//     }
// }