use egui::ahash::{HashMap, HashMapExt};
use rand::prelude::*;
use crate::{gameplay::{Cell, Color, GameGrid, LayerConnection, Puzzle, PuzzleCell, SwapRecord}, generator::solver::find_solution, grids::{Direction, DirectionMap, DirectionMapData, DirectionSet, Grid, GridIndex, GridSize, Rotation}};

use super::solutions::generate_solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceSettings {
    None,
    Maybe,
    Definitely
}

#[derive(Debug, Clone)]
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
    // Chance of adding any extra connection.
    pub extra_connection_chance: f32,
    // Chance of adding an intersection.
    pub intersection_chance: f32,
    // Maximum number of intersections.
    pub max_intersections: usize,
    // Chance to remove extra, loop-forming connections.
    pub knockout_loop_chance: f32
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
            extra_connection_chance: 0.5,
            intersection_chance: 0.0,
            max_intersections: 0,
            knockout_loop_chance: 0.33
        }
    }
}

pub fn generate_puzzle(generator_settings: &GeneratorSettings) -> Puzzle {
    let solution_grid = generate_solution(generator_settings);
    let mut working_grid = Grid::from_puzzle_grid(solution_grid);

    println!("NEW PUZZLE");
    let solution = reverse_solution(&mut working_grid, generator_settings.swap_count);
    println!("EXPECTED {solution:?}");

    let puzzle = create_puzzle_from_grid(working_grid, solution.len() as u8);

    // if let Some(short_sol) = find_solution(&puzzle, solution.len() as u8 - 1) {
    //     println!("ACTUAL {short_sol:?}");
    // }

    puzzle
}

fn create_puzzle_from_grid(game_grid: Grid<Cell>, swaps: u8) -> Puzzle {
    let mut grid = Grid::with_size(game_grid.size());
    for (pos, cell) in game_grid.iter() {
        grid.insert(pos, cell.to_puzzle_cell()).unwrap();
    }
    Puzzle::new(grid, swaps)
}
fn swap_record_matches(grid: &Grid<Cell>, record: SwapRecord) -> bool {
    let cell_a = grid.get(record.a).unwrap();
    let cell_b = grid.get(record.b).unwrap();
    Cell::can_swap(cell_a, cell_b)
        && cell_a.rotation_for_fill() == record.a_rotation
        && cell_b.rotation_for_fill() == record.b_rotation
}

fn reverse_solution(grid: &mut Grid<Cell>, swaps: u8) -> Vec<SwapRecord> {
    let mut solution = Vec::new();
    for _ in 0..swaps {
        solution.push(reverse_swap(grid));
    }
    solution.reverse();
    solution
}

fn reverse_swap(grid: &mut Grid<Cell>) -> SwapRecord {
    let mut possible_rotations = vec![Rotation::None];
    if grid.iter_layers().any(|layer| layer.1.fill.contains(Color::CCW)) {
        possible_rotations.push(Rotation::CCW);
    }
    if grid.iter_layers().any(|layer| layer.1.fill.contains(Color::CW)) {
        possible_rotations.push(Rotation::CW);
    }
    let positions: Vec<_> = grid.iter().filter(|(_, c)| c.source() != Some(Color::Red))
        .map(|(p, c)| p).collect();
    let mut selected: [GridIndex; 2] = [GridIndex { x: 0, y: 0 }, GridIndex { x: 0, y: 0 }];
    let mut rotations: [Rotation; 2] = [Rotation::None, Rotation::None];
    loop {
        positions.choose_multiple(&mut rand::thread_rng(), selected.len())
            .zip(selected.iter_mut()).for_each(|(s, buf)| *buf = *s);
        rotations.iter_mut().for_each(|buf| *buf = *possible_rotations.choose(&mut rand::thread_rng()).unwrap());

        let record = SwapRecord::new(selected[0], selected[1], rotations[0], rotations[1]);
        grid.swap(record.a, record.b);
        grid.get_mut(record.a).unwrap().rotate(record.a_rotation.inverse());
        grid.get_mut(record.b).unwrap().rotate(record.b_rotation.inverse());
        grid.fill();

        if swap_record_matches(grid, record) {
            break record
        }
        else {
            grid.get_mut(record.a).unwrap().rotate(record.a_rotation);
            grid.get_mut(record.b).unwrap().rotate(record.b_rotation);
            grid.swap(record.a, record.b);
        }
    }
}