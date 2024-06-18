use crate::{
    gameplay::{Cell, GameGrid, GridSolveState, Puzzle, SwapRecord},
    grids::{Grid, GridIndex, Rotation},
};

fn swap_without_fill(
    grid: &mut Grid<Cell>,
    a: GridIndex,
    rotate_a: Rotation,
    b: GridIndex,
    rotate_b: Rotation,
) {
    grid.get_mut(a).unwrap().rotate(rotate_a);
    grid.get_mut(b).unwrap().rotate(rotate_b);
    grid.swap(a, b);
}

fn get_possible_swaps(grid: &Grid<Cell>) -> Vec<SwapRecord> {
    let entries: Vec<_> = grid.iter().collect();
    let mut swaps = Vec::new();
    for i in 0..(entries.len() - 1) {
        let entry_i = &entries[i];
        for entry_j in entries.iter().skip(i + 1) {
            if Cell::can_swap(entry_i.1, entry_j.1) {
                swaps.push(SwapRecord::new(
                    entry_i.0,
                    entry_j.0,
                    entry_i.1.rotation_for_fill(),
                    entry_j.1.rotation_for_fill(),
                ));
            }
        }
    }
    swaps
}

pub fn find_solution(puzzle: &Puzzle, maximum_swaps: u8) -> Option<Vec<SwapRecord>> {
    let mut grid = Grid::from_puzzle_grid(puzzle.start());
    grid.fill();
    if grid.is_solved() == GridSolveState::Solved {
        Some(vec![])
    } else if maximum_swaps == 0 {
        None
    } else if let Some(mut reverse_sol) = find_solution_from_grid(&mut grid, maximum_swaps) {
        reverse_sol.reverse();
        Some(reverse_sol)
    } else {
        None
    }
}

fn find_solution_from_grid(grid: &mut Grid<Cell>, swaps_left: u8) -> Option<Vec<SwapRecord>> {
    let swaps = get_possible_swaps(grid);
    for swap in swaps {
        swap_without_fill(grid, swap.a, swap.a_rotation, swap.b, swap.b_rotation);
        grid.fill();

        if grid.is_solved() == GridSolveState::Solved {
            return Some(vec![swap]);
        }
        if swaps_left > 1 {
            if let Some(mut solution) = find_solution_from_grid(grid, swaps_left - 1) {
                solution.push(swap);
                return Some(solution);
            }
        };

        swap_without_fill(
            grid,
            swap.a,
            swap.b_rotation.inverse(),
            swap.b,
            swap.a_rotation.inverse(),
        );
    }
    None
}
