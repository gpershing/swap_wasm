use rand::prelude::*;
use crate::{gameplay::{Cell, Color, GameGrid, Puzzle, SwapRecord}, generator::solver::find_solution, grids::{Grid, GridIndex, GridSize, Rotation}};

use super::solutions::generate_solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum SourceSettings {
    None,
    Maybe,
    Definitely
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GeneratorSettings {
    // Size of the grid.
    pub size: GridSize,
    // Number of swaps.
    pub swap_count: u8,
    // Odds that a single space is missing.
    pub missing_chance: f32,
    // Maximum number of missing spaces.
    pub missing: usize,
    // Can the puzzle include red sources?
    pub stop_sources: SourceSettings,
    // Can the puzzle include yellow or orange sources?
    pub rotator_sources: SourceSettings,
    // Minimum number of regions to add, exluding the required purple region.
    pub min_regions: usize,
    // Chance of adding an extra region after the minimum.
    pub extra_region_chance: f32,
    // Chance of adding an extra source to a region.
    pub extra_source_chance: f32,
    // Chance of adding an intersection.
    pub intersection_chance: f32,
    // Maximum number of intersections.
    pub max_intersections: usize,
    // Chance to remove extra, loop-forming connections.
    pub knockout_loop_chance: f32,
    // If non-zero, will check for shorter solutions than intended,
    // and attempt to find a longer solution.
    pub check_solution_len: usize,
    // If solution check fails, how many times to we retry before giving up.
    pub check_solution_retries: usize
}

impl Default for GeneratorSettings {
    fn default() -> Self {
        Self {
            size: GridSize::new(3, 3),
            swap_count: 4,
            missing_chance: 0.1,
            missing: 2,
            stop_sources: SourceSettings::None,
            rotator_sources: SourceSettings::None,
            min_regions: 2,
            extra_region_chance: 0.1,
            extra_source_chance: 0.1,
            intersection_chance: 0.0,
            max_intersections: 0,
            knockout_loop_chance: 0.33,
            check_solution_len: 1,
            check_solution_retries: 3
        }
    }
}

pub fn generate_puzzle(generator_settings: &GeneratorSettings) -> Puzzle {
    if generator_settings.size.width * generator_settings.size.height <= 1 {
        panic!("Size too small!");
    }
    loop {
        if let Some(puzzle) = try_generate_puzzle(generator_settings) {
            break puzzle
        }
    }
}

fn try_generate_puzzle(generator_settings: &GeneratorSettings) -> Option<Puzzle> {
    let solution_grid = generate_solution(generator_settings);
    let mut working_grid = Grid::from_puzzle_grid(solution_grid);

    let mut solution = reverse_solution(&mut working_grid, generator_settings.swap_count);
    if solution.len() == 0 {
        return None
    }
    println!("initial {:?}", solution);

    let mut puzzle = create_puzzle_from_grid(&mut working_grid, solution.len() as u8, solution.first().copied().unwrap());

    let check = generator_settings.check_solution_len.min(generator_settings.swap_count as usize - 1);
    for _ in 0..generator_settings.check_solution_retries {
        if let Some(shorter_solution) = find_solution(&puzzle, check as u8) {
            let remaining = reverse_solution(&mut working_grid, generator_settings.swap_count - shorter_solution.len() as u8);
            solution = [remaining, shorter_solution].concat();
            if solution.len() == 0 {
                return None
            }
            puzzle = create_puzzle_from_grid(&mut working_grid, solution.len() as u8, solution.first().copied().unwrap());
            println!("retry {:?}", solution);
        }
        else {
            break;
        }
    }

    Some(puzzle)
}

fn create_puzzle_from_grid(game_grid: &mut Grid<Cell>, swaps: u8, first_move: SwapRecord) -> Puzzle {
    game_grid.fill();
    let hint = if game_grid.get(first_move.a).map(|cell| cell.has_color_in_any_layer(Color::SWAP)).unwrap_or(false) {
        first_move.b
    }
    else {
        first_move.a
    };

    let mut grid = Grid::with_size(game_grid.size());
    for (pos, cell) in game_grid.iter() {
        grid.insert(pos, cell.to_puzzle_cell()).unwrap();
    }
    Puzzle::new(grid, swaps, hint)
}
fn swap_record_matches(grid: &Grid<Cell>, record: SwapRecord) -> bool {
    let cell_a = grid.get(record.a).unwrap();
    let cell_b = grid.get(record.b).unwrap();
    Cell::can_swap(cell_a, cell_b)
        && cell_a.rotation_for_fill() == record.a_rotation
        && cell_b.rotation_for_fill() == record.b_rotation
}

fn swap_is_trivial(grid: &Grid<Cell>, record: SwapRecord) -> bool {
    let a = grid.get(record.a).unwrap();
    let b = grid.get(record.b).unwrap();
    if a.source() != b.source() { return false; }
    if a.get_layer_count() != b.get_layer_count() { return false; }

    let layers_a: Vec<_> = a.iter_layers().collect();
    let layers_b: Vec<_> = b.iter_layers().collect();
    if layers_a.len() == 1 {
        layers_a[0].connections == layers_b[0].connections.rotated(record.a_rotation.inverse()) &&
        layers_b[0].connections == layers_a[0].connections.rotated(record.b_rotation.inverse())
    }
    else {
        false // rare enough that we can ignore.
    }
}

fn reverse_solution(grid: &mut Grid<Cell>, swaps: u8) -> Vec<SwapRecord> {
    let mut solution = Vec::new();
    for _ in 0..swaps {
        if let Some(record) = reverse_swap(grid) {
            solution.push(record);
        }
        else {
            break
        }
    }
    solution.reverse();
    solution
}

fn reverse_swap(grid: &mut Grid<Cell>) -> Option<SwapRecord> {
    let mut possible_rotations = vec![Rotation::None];
    if grid.iter_layers().any(|layer| layer.1.fill.contains(Color::CCW)) {
        possible_rotations.push(Rotation::CounterClockwise);
    }
    if grid.iter_layers().any(|layer| layer.1.fill.contains(Color::CW)) {
        possible_rotations.push(Rotation::Clockwise);
    }
    let positions: Vec<_> = grid.iter().filter(|(_, c)| c.source() != Some(Color::Red))
        .map(|(p, _)| p).collect();
    let mut selected: [GridIndex; 2] = [GridIndex { x: 0, y: 0 }, GridIndex { x: 0, y: 0 }];
    let mut rotations: [Rotation; 2] = [Rotation::None, Rotation::None];
    for _ in 0..999 {
        positions.choose_multiple(&mut rand::thread_rng(), selected.len())
            .zip(selected.iter_mut()).for_each(|(s, buf)| *buf = *s);
        rotations.iter_mut().for_each(|buf| *buf = *possible_rotations.choose(&mut rand::thread_rng()).unwrap());

        let record = SwapRecord::new(selected[0], selected[1], rotations[0], rotations[1]);
        
        if swap_is_trivial(grid, record) {
            continue;
        }
        grid.swap(record.a, record.b);
        grid.get_mut(record.a).unwrap().rotate(record.a_rotation.inverse());
        grid.get_mut(record.b).unwrap().rotate(record.b_rotation.inverse());
        grid.fill();

        if swap_record_matches(grid, record) {
            return Some(record)
        }
        else {
            grid.get_mut(record.a).unwrap().rotate(record.a_rotation);
            grid.get_mut(record.b).unwrap().rotate(record.b_rotation);
            grid.swap(record.a, record.b);
        }
    }
    None
}