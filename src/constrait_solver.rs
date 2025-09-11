use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::Debug;
use itertools::Itertools;
use rand::rngs::{SmallRng, ThreadRng};
use rand::{Rng, SeedableRng};
use rand::seq::IndexedRandom;
use crate::constrait_solver::CellSolveState::{Solved, Unsolved};
use crate::piece;
use crate::piece::{Cell, PieceOps, get_full_cell_domain, get_piece_domain, VisualCell};
use crate::piece::VisualCell::CellEmpty;

pub const PUZZLE_WIDTH: usize = 6;
pub const PUZZLE_HEIGHT: usize = 6;
pub type Domain = Vec<Cell>;

#[derive(Debug)]
pub enum Direction {
    Right,
    Up,
    Left,
    Down,
}

#[derive(Debug, Clone)]
pub enum CellSolveState {
    Solved(Cell),
    Unsolved(Domain),
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub grid: [[CellSolveState; PUZZLE_WIDTH]; PUZZLE_HEIGHT],
    pub visual_grid: [[VisualCell; PUZZLE_WIDTH]; PUZZLE_WIDTH],

    pub pieces_left: Vec<&'static dyn PieceOps>,
}

impl Default for Grid {
    fn default() -> Self {
        let full_domain = get_full_cell_domain();
        let full_domain_cell_state = Unsolved(full_domain.clone());
        let pieces_left = get_piece_domain().clone();
        Grid {
            grid: [
                [
                    full_domain_cell_state.clone(),
                    full_domain_cell_state.clone(),
                    full_domain_cell_state.clone(),
                    full_domain_cell_state.clone(),
                    full_domain_cell_state.clone(),
                    full_domain_cell_state.clone(),
                ],
                [
                    full_domain_cell_state.clone(),
                    full_domain_cell_state.clone(),
                    full_domain_cell_state.clone(),
                    full_domain_cell_state.clone(),
                    full_domain_cell_state.clone(),
                    full_domain_cell_state.clone(),
                ],
                [
                    full_domain_cell_state.clone(),
                    full_domain_cell_state.clone(),
                    full_domain_cell_state.clone(),
                    full_domain_cell_state.clone(),
                    full_domain_cell_state.clone(),
                    full_domain_cell_state.clone(),
                ],
                [
                    full_domain_cell_state.clone(),
                    full_domain_cell_state.clone(),
                    full_domain_cell_state.clone(),
                    full_domain_cell_state.clone(),
                    full_domain_cell_state.clone(),
                    full_domain_cell_state.clone(),
                ],
                [
                    full_domain_cell_state.clone(),
                    full_domain_cell_state.clone(),
                    full_domain_cell_state.clone(),
                    full_domain_cell_state.clone(),
                    full_domain_cell_state.clone(),
                    full_domain_cell_state.clone(),
                ],
                [
                    full_domain_cell_state.clone(),
                    full_domain_cell_state.clone(),
                    full_domain_cell_state.clone(),
                    full_domain_cell_state.clone(),
                    full_domain_cell_state.clone(),
                    full_domain_cell_state.clone(),
                ],
            ],
            visual_grid: [[CellEmpty; PUZZLE_WIDTH]; PUZZLE_HEIGHT],
            pieces_left,
        }
    }
}

impl Grid {

    fn get_neighbor_cell(&self, x: usize, y: usize, direction: Direction) -> Option<&Cell> {
        const MAX_X: usize = PUZZLE_WIDTH - 1;
        const MAX_Y: usize = PUZZLE_HEIGHT - 1;
        match (x, y, direction) {
            (x, _, Direction::Right) if x >= MAX_X => {
                Some(&piece::Cell_Empty)
            }
            (_, 0, Direction::Up) => {
                Some(&piece::Cell_Empty)
            }
            (0, _, Direction::Left) => {
                Some(&piece::Cell_Empty)
            }
            (_, y, Direction::Down) if y >= MAX_Y => {
                Some(&piece::Cell_Empty)
            }
            (x, y, direction) => {
                let (x, y) = match direction {
                    Direction::Right => {
                        (x + 1, y)
                    }
                    Direction::Up => {
                        (x, y - 1)
                    }
                    Direction::Left => {
                        (x - 1, y)
                    }
                    Direction::Down => {
                        (x, y + 1)
                    }
                };
                let cell_solve_state = &self.grid[y][x];
                if let Solved(cell) = cell_solve_state {
                    Some(cell)
                }else {
                    None
                }
            }
        }
    }

    pub fn check(&self) -> bool {
        for cell in self.grid.iter().flatten() {
            if let Unsolved(domain) = cell {
                if domain.is_empty() {
                    return false;
                }
            }
        }

        for y in 0..6usize {
            for x in 0..6usize {
                let curr_cell = &self.grid[y][x];
                if let Solved(curr_cell) = curr_cell {
                    let neighbor_right = self.get_neighbor_cell(x, y, Direction::Right).cloned();
                    let neighbor_top = self.get_neighbor_cell(x, y, Direction::Up).cloned();
                    let neighbor_left = self.get_neighbor_cell(x, y, Direction::Left).cloned();
                    let neighbor_bottom = self.get_neighbor_cell(x, y, Direction::Down).cloned();
                    if let Some(other) = neighbor_right {
                        if curr_cell.right != other.left {
                            return false;
                        }
                    }
                    if let Some(other) = neighbor_top {
                        if curr_cell.top != other.bottom {
                            return false;
                        }
                    }
                    if let Some(other) = neighbor_left {
                        if curr_cell.left != other.right {
                            return false;
                        }
                    }
                    if let Some(other) = neighbor_bottom {
                        if curr_cell.bottom != other.top {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }

    pub fn do_constraint_propagation(&mut self) {
        for y in 0..6usize {
            for x in 0..6usize {
                let neighbor_right = self.get_neighbor_cell(x, y, Direction::Right).cloned();
                let neighbor_top = self.get_neighbor_cell(x, y, Direction::Up).cloned();
                let neighbor_left = self.get_neighbor_cell(x, y, Direction::Left).cloned();
                let neighbor_bottom = self.get_neighbor_cell(x, y, Direction::Down).cloned();
                // TODO: More constraints (Can't have 2 of the same cell edges diagonal from each other)
                let curr_cell = &mut self.grid[y][x];
                if let Unsolved(domain) = curr_cell {
                    domain.retain(|cell| {
                        if let Some(other) = neighbor_right {
                            if cell.right != other.left {
                                return false;
                            }
                        }
                        if let Some(other) = neighbor_top {
                            if cell.top != other.bottom {
                                return false;
                            }
                        }
                        if let Some(other) = neighbor_left {
                            if cell.left != other.right {
                                return false;
                            }
                        }
                        if let Some(other) = neighbor_bottom {
                            if cell.bottom != other.top {
                                return false;
                            }
                        }
                        true
                    });
                }
            }
        }
    }

    // 00 10 20 30 40 50
    // 10 11 21 31 41 51
    // 20 12 22 32 42 52
    // 30 13 23 33 43 53
    // 40 14 24 34 44 54
    // 50 15 25 35 45 55
    pub fn place_piece(&mut self, piece: &dyn PieceOps, x: usize, y: usize) -> Result<(), &'static str> {
        assert!(self.pieces_left.contains(&piece));

        let width = piece.width();
        let height = piece.height();

        if x + width > 6 || y + height > 6 {
            return Err("Piece cell outside bounds");
        }

        let piece_cells = piece.cells();
        let mut changes = self.clone();
        for local_x in 0..width {
            for local_y in 0..height {
                let gx = local_x + x;
                let gy = local_y + y;
                if let Solved(_) = changes.grid[gy][gx] {
                    // println!("Tried to place piece which would overwrite Solved cell at {}, {}", gx, gy);
                    return Err("Tried to place piece which would overwrite Solved cell");
                } else {
                    changes.grid[gy][gx] = Solved(piece_cells[local_y][local_x]);
                }
            }
        }
        changes.do_constraint_propagation();
        if !changes.check() {
            return Err("Failed check after placing piece")
        }
        *self = changes;
        self.pieces_left.retain(|p| piece != *p);

        let visual_cells = piece.visual_cells();
        for local_x in 0..width {
            for local_y in 0..height {
                self.visual_grid[local_y + y][local_x + x] = visual_cells[local_y][local_x];
            }
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct SolverMove {
    piece_id: usize,
    rotation: PieceRotation,
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct SolverState {
    pub grid_stack: Vec<Grid>,

    // Wait a second, that just sounds like recursion with extra steps!
    pub tried_branches: Vec<Vec<SolverMove>>,

    rng: SmallRng,
}

impl SolverState {
    pub fn new() -> SolverState {
        let starting_grid = Grid::default();
        SolverState {
            grid_stack: vec![starting_grid],
            tried_branches: vec![vec![]],

            rng: SmallRng::seed_from_u64(69),
        }
    }

    pub fn propagatev2(&mut self){
        // Push state
        // Try to place a piece
        // If find piece to place, done
        // If can't find piece to place, pop and try again
        // If nothing left to pop, failed

        #[derive(Debug)]
        struct DomainWithPosition {
            domain: Domain,
            x: usize,
            y: usize,
        }

        let mut cells_by_entropy = self
            .current_grid()
            .grid
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter().enumerate().map(move |(x, cell_state)| {
                    if let Unsolved(domain) = cell_state {
                        Some(
                            DomainWithPosition{
                                domain: domain.clone(),
                                x,
                                y
                            }
                        )
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .flatten()
            .sorted_by(|d1, d2| d1.domain.len().cmp(&d2.domain.len()))
            .collect::<VecDeque<_>>();

        let mut placed_piece = false;

        'outer: for domain in cells_by_entropy {
            // TODO: Choose pieces better
             for piece in self.current_grid().pieces_left.clone() {
                for rotation in PieceRotation::ROTATIONS {
                    let permutation = piece.rotate(rotation);
                    let solver_move = SolverMove {
                        piece_id: permutation.piece_id(),
                        rotation,
                        x: domain.x,
                        y: domain.y,
                    };
                    if self.tried_branches.last().unwrap().contains(&solver_move) {
                        // println!("Skipping move because it already failed");
                        continue;
                    }
                    let mut temp_grid = self.current_grid().clone();
                    let place_result = temp_grid.place_piece(&*permutation, domain.x, domain.y);
                    if place_result.is_ok() {
                        self.push_state(solver_move);
                        *self.grid_stack.last_mut().unwrap() = temp_grid;
                        // println!("placed piece at {}, {}", domain.x, domain.y);
                        placed_piece = true;
                        break 'outer;
                    } else {
                        // println!(
                        //     "Tried to place piece {} rotation {:?} at ({}, {}), failed.",
                        //     permutation.piece_id(),
                        //     rotation,
                        //     domain.x,
                        //     domain.y
                        // );
                    }
                }
            }
            // break;
        }

        if !placed_piece {
            self.pop_state();
            // Couldn't place a piece, we need to backtrack
            // println!("Failed to place any piece, backtracking...")
        }

    }

    pub fn tick(&mut self) {
        // self.propagate();
        self.propagatev2();
    }

    pub fn current_grid(&self) -> &Grid {
        self.grid_stack.last().unwrap()
    }

    pub fn current_grid_mut(&mut self) -> &mut Grid {
        self.grid_stack.last_mut().unwrap()
    }

    pub fn push_state(&mut self, solver_move: SolverMove){
        self.grid_stack.push(self.current_grid().clone());
        self.tried_branches.last_mut().unwrap().push(solver_move);
        self.tried_branches.push(vec![]);
        // println!("Pushed state with solved: {}/{}", get_piece_domain().len() - self.current_grid().pieces_left.len(), get_piece_domain().len());
    }

    pub fn pop_state(&mut self){
        self.grid_stack.pop().unwrap();
        self.tried_branches.pop().unwrap();
    }

    pub fn solve(&self) {}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceRotation {
    CCW0,
    CCW90,
    CCW180,
    CCW270,
}

impl PieceRotation {
    const ROTATIONS: [PieceRotation; 4] = [PieceRotation::CCW0, PieceRotation::CCW90, PieceRotation::CCW180, PieceRotation::CCW270];
}

pub fn solve(solver_state: &mut SolverState) {

    // Place starting piece
    // Setup domain
    // Constraint propagation
    // Fill in any domains that only have 1 option left
    // If all domains have n > 1, pick one of the lowest n and pick randomly
}
