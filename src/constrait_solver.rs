use std::cmp::Ordering;
use std::fmt::Debug;
use rand::rngs::{SmallRng, ThreadRng};
use rand::{Rng, SeedableRng};
use crate::constrait_solver::CellSolveState::{Solved, Unsolved};
use crate::piece;
use crate::piece::{Cell, PieceOps, get_full_cell_domain, get_piece_domain};

pub const PUZZLE_WIDTH: usize = 6;
pub const PUZZLE_HEIGHT: usize = 6;
pub type Domain = Vec<Cell>;

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

    pub pieces_left: Vec<&'static (dyn PieceOps + Sync)>,
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
    pub fn place_piece(&mut self, piece: &(dyn PieceOps + Sync), x: usize, y: usize, rotation: PieceRotation) -> bool {
        assert!(self.pieces_left.contains(&piece));

        let mut width = piece.width();
        let mut height = piece.height();

        match rotation {
            PieceRotation::CCW0 => {}
            PieceRotation::CCW90 => {
                std::mem::swap(&mut width, &mut height);
            }
            PieceRotation::CCW180 => {}
            PieceRotation::CCW270 => {
                std::mem::swap(&mut width, &mut height);
            }
        }

        if x + width > 6 || y + height > 6 {
            return false;
        }

        let piece_cells = match rotation {
            PieceRotation::CCW0 => piece.cells(),
            PieceRotation::CCW90 => piece.rotate_90().cells(),
            PieceRotation::CCW180 => piece.rotate_90().rotate_90().cells(),
            PieceRotation::CCW270 => piece.rotate_90().rotate_90().rotate_90().cells(),
        };
        let mut changes = self.clone();
        for local_x in 0..width {
            for local_y in 0..height {
                let gx = local_x + x;
                let gy = local_y + y;
                if let Solved(_) = changes.grid[gy][gx] {
                    println!("Tried to place piece which would overwrite Solved cell at {}, {}", gx, gy);
                    return false;
                } else {
                    changes.grid[gy][gx] = Solved(piece_cells[local_y][local_x]);
                }
            }
        }
        if !changes.check() {
            return false
        }
        *self = changes;
        true
    }
}

#[derive(Debug)]
pub struct SolverState {
    pub current_grid: Grid,
    pub grid_stack: Vec<Grid>,

    rng: SmallRng,
}

impl SolverState {
    pub fn new() -> SolverState {
        let starting_grid = Grid::default();
        SolverState {
            current_grid: starting_grid,
            grid_stack: vec![],

            rng: SmallRng::seed_from_u64(69),
        }
    }

    pub fn propagate(&mut self) {
        let mut did_propagate = false;
        'outer: for y in 0..6usize {
            for x in 0..6usize {
                let curr_cell = &self.current_grid.grid[y][x];
                if let Unsolved(domain) = curr_cell.clone() {
                    if domain.len() == 1 {
                        for (i, p) in get_piece_domain().into_iter().enumerate(){
                            if p.cells()[0][0] == domain[0] {
                                for rotation in PieceRotation::ROTATIONS {
                                    let successfully_placed_piece = self.current_grid.place_piece(*p, x, y, rotation);
                                    if successfully_placed_piece {
                                        did_propagate = true;
                                        break 'outer;
                                    } else {
                                        println!("Failed to place piece. ID: {} Rotation: {:?}", i + 1, rotation);
                                    }

                                }
                            }
                        }
                    }
                }
            }
        }

        if !did_propagate {
            println!("No viable propagations. Picking random from lowest entropy");
            let lowest_entropy_cell = self.current_grid.grid.iter().flatten().filter_map(|cell_solve_state|{
                if let Unsolved(domain) = cell_solve_state {
                    Some(domain)
                }else {
                    None
                }
            }).min_by(|cell1, cell2| {
                cell1.len().cmp(&cell2.len())
            }).unwrap();
            let rand = self.rng.random_range(0..lowest_entropy_cell.len());
            let random_cell = lowest_entropy_cell[rand];
            println!("Random Cell: {:?}", random_cell);
        }
    }

    pub fn tick(&mut self) {
        self.current_grid.do_constraint_propagation();
        self.propagate();
    }

    pub fn push_state(&mut self){
        self.grid_stack.push(self.current_grid.clone());
    }

    pub fn pop_state(&mut self){
        self.current_grid = self.grid_stack.pop().unwrap();
    }

    pub fn solve(&self) {}
}

#[derive(Debug, Clone, Copy)]
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
