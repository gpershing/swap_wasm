use crate::{gameplay::{fill_playing_grid, is_solved, puzzle_grid_to_playing_grid, Cell, Puzzle, PuzzleSolveState, SwapRecord}, grid_math::{Grid, Pos2, Rotation}};

fn swap_without_fill(grid: &mut Grid<Cell>, a: Pos2, rotate_a: Rotation, mut cell_a: Cell, b: Pos2, rotate_b: Rotation, mut cell_b: Cell) {
    cell_a.rotate_connections(rotate_a);
    cell_b.rotate_connections(rotate_b);
    grid.insert(a, cell_b);
    grid.insert(b, cell_a);
}

fn get_possible_swaps(grid: &Grid<Cell>) -> Vec<SwapRecord> {
    let entries: Vec<_> = grid.iter().collect();
    let mut swaps = Vec::new();
    for i in 0..(entries.len() - 1) {
        let entry_i = entries[i];
        for j in (i+1)..entries.len() {
            let entry_j = entries[j];
            if Cell::can_swap(entry_i.1, entry_j.1) {
                swaps.push(SwapRecord::new(*entry_i.0, *entry_j.0, entry_i.1.fill().get_rotation(), entry_j.1.fill().get_rotation()));
            }
        }
    }
    swaps
}

pub fn find_solution(puzzle: &Puzzle, maximum_swaps: u8) -> Option<Vec<SwapRecord>> {
    let mut grid = puzzle_grid_to_playing_grid(puzzle.start());
    if let Some(mut reverse_sol) = find_solution_from_grid(&mut grid, maximum_swaps) {
        reverse_sol.reverse();
        Some(reverse_sol)
    }
    else {
        None
    }
}

fn find_solution_from_grid(grid: &mut Grid<Cell>, swaps_left: u8) -> Option<Vec<SwapRecord>> {
    let swaps = get_possible_swaps(grid);
    for swap in swaps {
        let cell_a = *grid.get(swap.a).unwrap();
        let cell_b = *grid.get(swap.b).unwrap();
        swap_without_fill(grid, swap.a, swap.a_rotation, cell_a, swap.b, swap.b_rotation, cell_b);
        fill_playing_grid(grid);

        if is_solved(grid) == PuzzleSolveState::Solved {
            return Some(vec![swap]);
        }
        if swaps_left > 1 {
            if let Some(mut solution) = find_solution_from_grid(grid, swaps_left - 1) {
                solution.push(swap);
                return Some(solution)
            }
        };

        swap_without_fill(grid, swap.a, Rotation::None, cell_b, swap.b, Rotation::None, cell_a);
    }
    None
}