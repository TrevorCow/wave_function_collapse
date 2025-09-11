use std::collections::HashSet;
use std::fmt::Debug;
use std::iter::Flatten;
use crate::constrait_solver::PieceRotation;
use crate::piece::ConnectionType::{Double, NoConnection, Straight};
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::process::exit;
use std::sync::LazyLock;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ConnectionType {
    NoConnection,
    Straight,
    Double,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Cell {
    pub right: ConnectionType,
    pub top: ConnectionType,
    pub left: ConnectionType,
    pub bottom: ConnectionType,
}

impl Cell {
    pub const fn rotate_90_ccw(&self) -> Cell {
        Cell { right: self.bottom, top: self.right, left: self.top, bottom: self.left }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PieceDefinition<const WIDTH: usize, const HEIGHT: usize> {
    cells: [[Cell; WIDTH]; HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize> PieceDefinition<WIDTH, HEIGHT> {
    pub const fn new(cells: [[Cell; WIDTH]; HEIGHT]) -> Self {
        Self { cells}
    }

    // pub fn into_piece(self) -> Piece {
    //     Piece { width: WIDTH, height: HEIGHT, cells: self.cells.to_vec().iter().map(|c| c.to_vec()).collect() }
    // }

    pub fn as_piece_ops(&self) -> &(dyn PieceOps + Sync){
        self
    }
}

pub static Cell_Empty: Cell = Cell { right: NoConnection, top: NoConnection, left: NoConnection, bottom: NoConnection };

static Cell_N_Right: Cell = Cell { right: Straight, top: NoConnection, left: NoConnection, bottom: Double };
static Cell_N_Right_90: Cell = Cell_N_Right.rotate_90_ccw();
static Cell_N_Right_180: Cell = Cell_N_Right_90.rotate_90_ccw();
static Cell_N_Right_270: Cell = Cell_N_Right_180.rotate_90_ccw();

static Cell_N_Left: Cell = Cell { right: NoConnection, top: NoConnection, left: Straight, bottom: Double };
static Cell_N_Left_90: Cell = Cell_N_Left.rotate_90_ccw();
static Cell_N_Left_180: Cell = Cell_N_Left_90.rotate_90_ccw();
static Cell_N_Left_270: Cell = Cell_N_Left_180.rotate_90_ccw();

static Cell_N_Center: Cell = Cell { right: NoConnection, top: Straight, left: NoConnection, bottom: Double };
static Cell_N_Center_90: Cell = Cell_N_Center.rotate_90_ccw();
static Cell_N_Center_180: Cell = Cell_N_Center_90.rotate_90_ccw();
static Cell_N_Center_270: Cell = Cell_N_Center_180.rotate_90_ccw();

static Cell_Straight: Cell = Cell { right: NoConnection, top: Straight, left: NoConnection, bottom: Straight };

// Used for the weird 2x2 piece
static CELL_WEIRD_1: Cell = Cell { right: NoConnection, top: Double, left: NoConnection, bottom: NoConnection };
static CELL_WEIRD_2: Cell = Cell { right: NoConnection, top: Double, left: NoConnection, bottom: NoConnection };

pub static P1: PieceDefinition<4, 1> = PieceDefinition::new([[Cell_Empty, Cell_Empty, Cell_N_Right, Cell_N_Left]]);
pub static P2: PieceDefinition<1, 3> = PieceDefinition::new([[Cell_N_Center_180], [Cell_N_Left_270], [Cell_N_Left_90]]);
pub static P3: PieceDefinition<1, 3> = PieceDefinition::new([[Cell_N_Right_180], [Cell_Empty], [Cell_Empty]]);
pub static P4: PieceDefinition<1, 3> = PieceDefinition::new([[Cell_N_Left_90], [Cell_Straight], [Cell_N_Center]]);
pub static P5: PieceDefinition<3, 1> = PieceDefinition::new([[Cell_N_Right, Cell_N_Right_180, Cell_Empty]]);
pub static P6: PieceDefinition<3, 1> = PieceDefinition::new([[Cell_N_Left_180, Cell_N_Left, Cell_Empty]]);
pub static P7: PieceDefinition<2, 1> = PieceDefinition::new([[Cell_N_Left, Cell_N_Center]]);
pub static P8: PieceDefinition<1, 3> = PieceDefinition::new([[Cell_N_Center_180], [Cell_N_Center], [Cell_N_Right_180]]);
pub static P9: PieceDefinition<2, 1> = PieceDefinition::new([[Cell_Empty, Cell_Empty]]);
pub static P10_P11: PieceDefinition<2, 2> = PieceDefinition::new([[CELL_WEIRD_1, Cell_N_Right], [Cell_Empty, CELL_WEIRD_2]]);
pub static P12: PieceDefinition<1, 2> = PieceDefinition::new([[Cell_N_Left], [Cell_N_Left_180]]);
pub static P13: PieceDefinition<2, 1> = PieceDefinition::new([[Cell_N_Right_90, Cell_N_Center_270]]);
pub static P14: PieceDefinition<2, 1> = PieceDefinition::new([[Cell_N_Right_90, Cell_N_Left_270]]);

pub trait PieceOps: Debug {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn cells(&self) -> Vec<Vec<Cell>>;

    fn rotate_90(&self) -> Box<dyn PieceOps>;
}

impl PartialEq for dyn PieceOps {
    fn eq(&self, other: &dyn PieceOps) -> bool {

    }
}

impl<const WIDTH: usize, const HEIGHT: usize> PieceOps for PieceDefinition<WIDTH, HEIGHT> {
    fn width(&self) -> usize {
        WIDTH
    }

    fn height(&self) -> usize {
        HEIGHT
    }

    fn cells(&self) -> Vec<Vec<Cell>> {
        self.cells.iter().cloned().map(|cells| cells.to_vec()).collect()
    }

    fn rotate_90(&self) -> Box<dyn PieceOps> {
        let mut rotated_piece_cells: [[MaybeUninit<Cell>; HEIGHT]; WIDTH] = [[MaybeUninit::uninit(); HEIGHT]; WIDTH];

        for y in 0..self.height() {
            for x in 0..self.width() {
                rotated_piece_cells[x][y] = MaybeUninit::new(self.cells[y][self.width() - x - 1].rotate_90_ccw());
            }
        }

        let rotated_piece_cells = unsafe {
            // std::mem::transmute(rotated_piece_cells)
            // Can not transmute between types with generic sizes
            // We get around this by casting pointers which is safe as
            // the layout of MaybeUninit<T> is the same as T
            rotated_piece_cells.as_ptr().cast::<[[Cell; HEIGHT]; WIDTH]>().read()
        };
       let new = PieceDefinition::<HEIGHT, WIDTH>{
           cells: rotated_piece_cells
       };
        Box::new(new)
    }
}

pub fn get_piece_domain() -> &'static Vec<&'static (dyn PieceOps + Sync)> {
    static PIECE_DOMAIN: LazyLock<Vec<&'static (dyn PieceOps + Sync)>> = LazyLock::new(|| {
        vec![
            P1.as_piece_ops(),
            P2.as_piece_ops(),
            P3.as_piece_ops(),
            P4.as_piece_ops(),
            P5.as_piece_ops(),
            P6.as_piece_ops(),
            P7.as_piece_ops(),
            P8.as_piece_ops(),
            P9.as_piece_ops(),
            P10_P11.as_piece_ops(),
            P12.as_piece_ops(),
            P13.as_piece_ops(),
            P14.as_piece_ops(),
        ]
    });
    PIECE_DOMAIN.deref()
}

pub fn get_full_cell_domain() -> &'static Vec<Cell> {
    static FULL_CELL_DOMAIN: LazyLock<Vec<Cell>> = LazyLock::new(|| {
        let piece_domain = get_piece_domain();
        let mut full_cell_domain = HashSet::new();

        for piece in piece_domain {
            for cell in piece.cells().into_iter().flatten() {
                full_cell_domain.insert(cell);
                full_cell_domain.insert(cell.rotate_90_ccw());
                full_cell_domain.insert(cell.rotate_90_ccw().rotate_90_ccw());
                full_cell_domain.insert(cell.rotate_90_ccw().rotate_90_ccw().rotate_90_ccw());
            }
        }

        full_cell_domain.into_iter().collect()
    });
    &FULL_CELL_DOMAIN
}
