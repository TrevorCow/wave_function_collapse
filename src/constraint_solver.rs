use crate::constraint_solver::CellSolveState::{Solved, Unsolved};
use crate::piece;
use crate::piece::VisualCell::CellEmpty;
use crate::piece::{Cell, PieceOps, VisualCell, get_full_cell_domain, get_piece_domain};
use itertools::Itertools;
use rand::rngs::SmallRng;
use rand::seq::{IndexedRandom, SliceRandom};
use rand::SeedableRng;
use std::fmt::Debug;

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
            (x, _, Direction::Right) if x >= MAX_X => Some(&piece::CELL_EMPTY),
            (_, 0, Direction::Up) => Some(&piece::CELL_EMPTY),
            (0, _, Direction::Left) => Some(&piece::CELL_EMPTY),
            (_, y, Direction::Down) if y >= MAX_Y => Some(&piece::CELL_EMPTY),
            (x, y, direction) => {
                let (x, y) = match direction {
                    Direction::Right => (x + 1, y),
                    Direction::Up => (x, y - 1),
                    Direction::Left => (x - 1, y),
                    Direction::Down => (x, y + 1),
                };
                let cell_solve_state = &self.grid[y][x];
                if let Solved(cell) = cell_solve_state { Some(cell) } else { None }
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

    pub fn can_place_piece(&self, piece: &dyn PieceOps, x: usize, y: usize) -> bool {
        debug_assert!(self.pieces_left.contains(&piece));

        let width = piece.width();
        let height = piece.height();

        if x + width > 6 || y + height > 6 {
            return false;
        }

        for local_x in 0..width {
            for local_y in 0..height {
                let gx = local_x + x;
                let gy = local_y + y;
                if let Solved(_) = self.grid[gy][gx] {
                    // println!("Tried to place piece which would overwrite Solved cell at {}, {}", gx, gy);
                    return false;
                }
            }
        }
        if !self.check() {
            return false;
        }

        true
    }

    pub fn place_piece_unchecked(&mut self, piece: &dyn PieceOps, x: usize, y: usize) {
        let width = piece.width();
        let height = piece.height();

        let piece_cells = piece.cells_flat();
        for local_x in 0..width {
            for local_y in 0..height {
                let gx = local_x + x;
                let gy = local_y + y;
                self.grid[gy][gx] = Solved(piece_cells[local_y * width + local_x]);
            }
        }
        self.do_constraint_propagation();
        self.pieces_left.retain(|p| piece != *p);
        let visual_cells = piece.visual_cells_flat();
        for local_x in 0..width {
            for local_y in 0..height {
                self.visual_grid[local_y + y][local_x + x] = visual_cells[local_y * width + local_x];
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
        debug_assert!(self.pieces_left.contains(&piece));

        let width = piece.width();
        let height = piece.height();

        if x + width > 6 || y + height > 6 {
            return Err("Piece cell outside bounds");
        }

        // let piece_cells = piece.cells();
        // let mut changes = self.clone();
        // for local_x in 0..width {
        //     for local_y in 0..height {
        //         let gx = local_x + x;
        //         let gy = local_y + y;
        //         if let Solved(_) = changes.grid[gy][gx] {
        //             // println!("Tried to place piece which would overwrite Solved cell at {}, {}", gx, gy);
        //             return Err("Tried to place piece which would overwrite Solved cell");
        //         } else {
        //             changes.grid[gy][gx] = Solved(piece_cells[local_y][local_x]);
        //         }
        //     }
        // }
        let piece_cells = piece.cells_flat();
        let mut changes = vec![];
        for local_x in 0..width {
            for local_y in 0..height {
                let gx = local_x + x;
                let gy = local_y + y;
                if let Solved(_) = self.grid[gy][gx] {
                    // println!("Tried to place piece which would overwrite Solved cell at {}, {}", gx, gy);
                    return Err("Tried to place piece which would overwrite Solved cell");
                } else {
                    changes.push((gy, gx, Solved(piece_cells[local_y * width + local_x])));
                }
            }
        }
        for change in changes {
            self.grid[change.0][change.1] = change.2;
        }
        self.do_constraint_propagation();
        if !self.check() {
            return Err("Failed check after placing piece");
        }
        self.pieces_left.retain(|p| piece != *p);

        let visual_cells = piece.visual_cells_flat();
        for local_x in 0..width {
            for local_y in 0..height {
                self.visual_grid[local_y + y][local_x + x] = visual_cells[local_y * width + local_x];
            }
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
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
        SolverState { grid_stack: vec![starting_grid], tried_branches: vec![vec![]], rng: SmallRng::seed_from_u64(69) }
    }

    pub fn step_propagate(&mut self) -> Result<(), ()> {
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

        let cells_by_entropy = self
            .current_grid()
            .grid
            .iter()
            .enumerate()
            .map(|(y, row)| row.iter().enumerate().map(move |(x, cell_state)| if let Unsolved(domain) = cell_state { Some(DomainWithPosition { domain: domain.clone(), x, y }) } else { None }))
            .flatten()
            .flatten()
            .sorted_by(|d1, d2| d1.domain.len().cmp(&d2.domain.len()))
            .next();
        let cells_by_entropy = if let Some(cells_by_entropy) = cells_by_entropy {
            cells_by_entropy
        } else {
            return Err(());
        };

        let mut placed_piece = false;

        let domain = cells_by_entropy;
        // domain.domain.shuffle(&mut self.rng);
        // let chosen_cell = domain.domain.choose(&mut self.rng);
        // TODO: Choose pieces better
        'outer:
            for piece in self.current_grid().pieces_left.clone() {
                let pid = piece.piece_id();
                'rot: for rotation in PieceRotation::ROTATIONS {
                    let permutation = piece.rotate(rotation);
                    // if !permutation.cells().iter().flatten().contains(chosen_cell) {
                    //     // If the permutation doesn't contain the chosen cell, then we don't have to try it
                    //     continue;
                    // }
                    'cont:{
                        for cell in &domain.domain {
                            if permutation.cells_flat().contains(cell) {
                                // If the permutation doesn't contain the chosen cell, then we don't have to try it
                                break 'cont;
                            }
                        }
                        continue 'rot;
                    }
                    for local_x in 0..permutation.width() {
                        for local_y in 0..permutation.height() {
                            if local_x > domain.x || local_y > domain.y {
                                continue;
                            }
                            let solver_move = SolverMove { piece_id: pid, rotation, x: domain.x - local_x, y: domain.y - local_y };
                            if self.tried_branches.last().unwrap().contains(&solver_move) {
                                // println!("Skipping move because it already failed");
                                continue;
                            }
                            let can_place_piece = self.current_grid().can_place_piece(&*permutation, solver_move.x, solver_move.y);
                            // let place_result = self.current_grid_mut().place_piece(&*permutation, solver_move.x, solver_move.y);
                            if can_place_piece {
                                self.tried_branches.last_mut().unwrap().push(solver_move.clone());
                                self.push_state();
                                self.current_grid_mut().place_piece_unchecked(&*permutation, solver_move.x, solver_move.y);
                                // println!("placed piece at {}, {}", domain.x, domain.y);
                                placed_piece = true;
                                break 'outer;
                            } else {
                                // self.pop_state();
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
                }
            }

        // println!("We didn't find any matching piece for domain: {:?}", domain)

        if !placed_piece {
            if self.grid_stack.len() == 1 {
                return Err(());
            }
            self.pop_state();
            // Couldn't place a piece, we need to backtrack
            // println!("Failed to place any piece, backtracking...")
        }
        Ok(())
    }

    pub fn current_grid(&self) -> &Grid {
        self.grid_stack.last().unwrap()
    }

    pub fn current_grid_mut(&mut self) -> &mut Grid {
        self.grid_stack.last_mut().unwrap()
    }

    pub fn push_state(&mut self) {
        self.grid_stack.push(self.current_grid().clone());
        self.tried_branches.push(vec![]);
        // println!("Pushed state with solved: {}/{}", get_piece_domain().len() - self.current_grid().pieces_left.len(), get_piece_domain().len());
    }

    pub fn pop_state(&mut self) {
        self.grid_stack.pop().unwrap();
        self.tried_branches.pop().unwrap();
    }

    pub fn solve(&mut self) -> bool {
        while let Ok(_) = self.step_propagate() {}
        self.current_grid().pieces_left.len() == 0
    }
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
