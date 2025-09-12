use crate::constraint_solver::PieceRotation;
use crate::piece::ConnectionType::{Double, NoConnection, Straight};
use crate::piece::VisualCell::{CellEmpty, CellNCenter, CellNLeft, CellNRight, CellStraight, CellWeird1, CellWeird2};
use std::collections::HashSet;
use std::fmt::Debug;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::sync::LazyLock;

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

impl Default for Cell {
    fn default() -> Self {
        Cell { right: NoConnection, top: NoConnection, left: NoConnection, bottom: NoConnection }
    }
}

impl Cell {
    pub const fn rotate_90_ccw(&self) -> Cell {
        Cell { right: self.bottom, top: self.right, left: self.top, bottom: self.left }
    }
}

// #[derive(Clone, Copy, Debug)]
// pub struct VisualCell {
//     image: &'static str,
// }
// pub type VisualCell = &'static str;
#[derive(Clone, Copy, Debug)]
pub enum VisualCell {
    CellEmpty,
    CellNCenter(u32),
    CellNLeft(u32),
    CellNRight(u32),
    CellStraight(u32),
    CellWeird1(u32),
    CellWeird2(u32),
    Other(&'static str, u32),
}

impl VisualCell {
    pub fn get_image_path(&self) -> &'static str {
        match self {
            VisualCell::CellEmpty => "resources/cell_empty.png",
            VisualCell::CellNCenter(_) => "resources/cell_n_center.png",
            VisualCell::CellNLeft(_) => "resources/cell_n_left.png",
            VisualCell::CellNRight(_) => "resources/cell_n_right.png",
            VisualCell::CellStraight(_) => "resources/cell_straight.png",
            VisualCell::CellWeird1(_) => "resources/cell_weird_1.png",
            VisualCell::CellWeird2(_) => "resources/cell_weird_2.png",
            VisualCell::Other(path, _) => path,
        }
    }

    pub const fn rotate_90_ccw(mut self) -> Self {
        match &mut self {
            VisualCell::CellEmpty => (),
            VisualCell::CellNCenter(angle) => *angle += 90,
            VisualCell::CellNLeft(angle) => *angle += 90,
            VisualCell::CellNRight(angle) => *angle += 90,
            VisualCell::CellStraight(angle) => *angle += 90,
            VisualCell::CellWeird1(angle) => *angle += 90,
            VisualCell::CellWeird2(angle) => *angle += 90,
            VisualCell::Other(_, angle) => *angle += 90,
        }
        self
    }

    pub const fn angle(&self) -> u32 {
        match self {
            VisualCell::CellEmpty => 0,
            VisualCell::CellNCenter(angle) => *angle,
            VisualCell::CellNLeft(angle) => *angle,
            VisualCell::CellNRight(angle) => *angle,
            VisualCell::CellStraight(angle) => *angle,
            VisualCell::CellWeird1(angle) => *angle,
            VisualCell::CellWeird2(angle) => *angle,
            VisualCell::Other(_, angle) => *angle,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PieceDefinition<const WIDTH: usize, const HEIGHT: usize> {
    piece_id: usize,
    logical_cells: [[Cell; WIDTH]; HEIGHT],
    visual_cells: [[VisualCell; WIDTH]; HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize> PieceDefinition<WIDTH, HEIGHT> {
    pub const fn new(piece_id: usize, logical_cells: [[Cell; WIDTH]; HEIGHT], visual_cells: [[VisualCell; WIDTH]; HEIGHT]) -> Self {
        Self { piece_id, logical_cells, visual_cells }
    }

    pub fn as_piece_ops(&self) -> &(dyn PieceOps) {
        self
    }
}

pub static CELL_EMPTY: Cell = Cell { right: NoConnection, top: NoConnection, left: NoConnection, bottom: NoConnection };

pub static P1: PieceDefinition<4, 1> = PieceDefinition::new(
    1,
    [[
        CELL_EMPTY,
        CELL_EMPTY,
        Cell { right: NoConnection, top: NoConnection, left: NoConnection, bottom: Double },
        Cell { right: NoConnection, top: NoConnection, left: NoConnection, bottom: Double },
    ]],
    [[CellEmpty, CellEmpty, CellNRight(0), CellNLeft(0)]],
);
pub static P2: PieceDefinition<1, 3> = PieceDefinition::new(
    2, //
    [
        [Cell { right: NoConnection, top: Double, left: NoConnection, bottom: NoConnection }], //
        [Cell { right: NoConnection, top: NoConnection, left: Double, bottom: NoConnection}],                                                                     //
        [Cell { right: Double, top: NoConnection, left: NoConnection, bottom: Straight }],                                                                      //
    ], //
    [[CellNCenter(180)], [CellNLeft(270)], [CellNLeft(90)]],
);
pub static P3: PieceDefinition<1, 3> = PieceDefinition::new(
    3,                                                //
    [
        [Cell { right: NoConnection, top: Double, left: Straight, bottom: NoConnection }], //
        [CELL_EMPTY], //
        [CELL_EMPTY], //
    ], //
    [[CellNRight(180)], [CellEmpty], [CellEmpty]],
);
pub static P4: PieceDefinition<1, 3> = PieceDefinition::new(
    4,                                                    //
    [
        [Cell { right: Double, top: NoConnection, left: NoConnection, bottom: NoConnection }], //
        [CELL_EMPTY], //
        [Cell { right: NoConnection, top: NoConnection, left: NoConnection, bottom: Double}], //
    ], //
    [[CellNLeft(90)], [CellStraight(0)], [CellNCenter(0)]],
);
pub static P5: PieceDefinition<3, 1> = PieceDefinition::new(
    5,                                              //
    [[Cell {right: NoConnection, top: NoConnection, left: NoConnection, bottom: Double}, Cell {right: NoConnection, top: Double, left: NoConnection, bottom: NoConnection}, CELL_EMPTY]], //
    [[CellNRight(0), CellNRight(180), CellEmpty]],
);
pub static P6: PieceDefinition<3, 1> = PieceDefinition::new(
    6,                                            //
    [[Cell {right: NoConnection, top: Double, left: NoConnection, bottom: NoConnection}, Cell {right: NoConnection, top: NoConnection, left: NoConnection, bottom: Double}, CELL_EMPTY]], //
    [[CellNLeft(180), CellNLeft(0), CellEmpty]],
);
pub static P7: PieceDefinition<2, 1> = PieceDefinition::new(
    7,                              //
    [[Cell{right: NoConnection, top: Straight, left: Double, bottom: NoConnection}, Cell{ right: NoConnection, top: Straight, left: NoConnection, bottom: Double }]], //
    [[CellNLeft(270), CellNCenter(0)]],
);
pub static P8: PieceDefinition<1, 3> = PieceDefinition::new(
    8,                                                          //
    [
        [Cell{right: NoConnection, top: Double, left: NoConnection, bottom: NoConnection}], //
        [CELL_EMPTY], //
        [Cell {right: NoConnection, top: NoConnection, left: Straight, bottom: NoConnection}]], //
    [[CellNCenter(180)], [CellNCenter(0)], [CellNRight(180)]],
);
pub static P9: PieceDefinition<2, 1> = PieceDefinition::new(
    9,                          //
    [[CELL_EMPTY, CELL_EMPTY]], //
    [[CellEmpty, CellEmpty]],   //
);
pub static P10_P11: PieceDefinition<2, 2> = PieceDefinition::new(
    10,                                                           //
    [
        [Cell{right: NoConnection, top: Double, left: NoConnection, bottom: NoConnection}, Cell{right: Straight, top: NoConnection, left: NoConnection, bottom: NoConnection}],//
        [CELL_EMPTY, CELL_EMPTY],//
    ],   //
    [[CellWeird1(0), CellNRight(0)], [CellEmpty, CellWeird2(0)]], //
);
pub static P12: PieceDefinition<1, 2> = PieceDefinition::new(
    12,                                 //
    [
        [Cell {right: NoConnection, top: NoConnection, left: Straight, bottom: NoConnection}], //
        [Cell {right: Straight, top: NoConnection, left: NoConnection, bottom: NoConnection}],//
    ], //
    [[CellNLeft(0)], [CellNLeft(180)]], //
);
pub static P13: PieceDefinition<2, 1> = PieceDefinition::new(
    13,                                     //
    [[Cell{right: NoConnection, top: Straight, left: NoConnection, bottom: NoConnection}, Cell{right: Straight, top: NoConnection, left: NoConnection, bottom: NoConnection}]], //
    [[CellNRight(90), CellNCenter(270)]],   //
);
pub static P14: PieceDefinition<2, 1> = PieceDefinition::new(
    14,                                   //
    [[Cell{right: NoConnection, top: Straight, left: NoConnection, bottom: NoConnection}, Cell{right: NoConnection, top: Straight, left: NoConnection, bottom: NoConnection}]], //
    [[CellNRight(90), CellNLeft(270)]],   //
);

pub trait PieceOps: Debug + Sync {
    fn piece_id(&self) -> usize;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn cells(&self) -> Vec<Vec<Cell>>;

    fn cells_flat(&self) -> &[Cell];

    fn visual_cells(&self) -> Vec<Vec<VisualCell>>;

    fn visual_cells_flat(&self) -> &[VisualCell];

    fn rotate_90(&self) -> Box<dyn PieceOps>;

    fn rotate(&self, rotation: PieceRotation) -> Box<dyn PieceOps>;
}

impl PartialEq for &dyn PieceOps {
    fn eq(&self, other: &&dyn PieceOps) -> bool {
        self.piece_id() == other.piece_id()
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> PieceOps for PieceDefinition<WIDTH, HEIGHT> {
    fn piece_id(&self) -> usize {
        self.piece_id
    }

    fn width(&self) -> usize {
        WIDTH
    }

    fn height(&self) -> usize {
        HEIGHT
    }

    fn cells(&self) -> Vec<Vec<Cell>> {
        self.logical_cells.iter().cloned().map(|cells| cells.to_vec()).collect()
    }

    fn cells_flat(&self) -> &[Cell] {
        self.logical_cells.as_slice().as_flattened()
    }
    
    fn visual_cells(&self) -> Vec<Vec<VisualCell>> {
        self.visual_cells.iter().cloned().map(|cells| cells.to_vec()).collect()
    }

    fn visual_cells_flat(&self) -> &[VisualCell] {
        self.visual_cells.as_slice().as_flattened()
    }

    fn rotate_90(&self) -> Box<dyn PieceOps> {
        let mut rotated_logical_cells: [[MaybeUninit<Cell>; HEIGHT]; WIDTH] = [[MaybeUninit::uninit(); HEIGHT]; WIDTH];
        let mut rotated_visual_cells: [[MaybeUninit<VisualCell>; HEIGHT]; WIDTH] = [[MaybeUninit::uninit(); HEIGHT]; WIDTH];

        for y in 0..self.height() {
            for x in 0..self.width() {
                rotated_logical_cells[x][y] = MaybeUninit::new(self.logical_cells[y][self.width() - x - 1].rotate_90_ccw());
                rotated_visual_cells[x][y] = MaybeUninit::new(self.visual_cells[y][self.width() - x - 1].rotate_90_ccw())
            }
        }

        let rotated_logical_cells = unsafe {
            // std::mem::transmute(rotated_piece_cells)
            // Can not transmute between types with generic sizes
            // We get around this by casting pointers which is safe as
            // the layout of MaybeUninit<T> is the same as T
            rotated_logical_cells.as_ptr().cast::<[[Cell; HEIGHT]; WIDTH]>().read()
        };
        let rotated_visual_cells = unsafe {
            // std::mem::transmute(rotated_piece_cells)
            // Can not transmute between types with generic sizes
            // We get around this by casting pointers which is safe as
            // the layout of MaybeUninit<T> is the same as T
            rotated_visual_cells.as_ptr().cast::<[[VisualCell; HEIGHT]; WIDTH]>().read()
        };
        let new = PieceDefinition::<HEIGHT, WIDTH> { piece_id: self.piece_id, logical_cells: rotated_logical_cells, visual_cells: rotated_visual_cells };
        Box::new(new)
    }

    fn rotate(&self, rotation: PieceRotation) -> Box<dyn PieceOps> {
        match rotation {
            PieceRotation::CCW0 => Box::new(self.clone()),
            PieceRotation::CCW90 => self.rotate_90(),
            PieceRotation::CCW180 => self.rotate_90().rotate_90(),
            PieceRotation::CCW270 => self.rotate_90().rotate_90().rotate_90(),
        }
    }
}

pub fn get_piece_domain() -> &'static Vec<&'static (dyn PieceOps)> {
    static PIECE_DOMAIN: LazyLock<Vec<&'static (dyn PieceOps)>> = LazyLock::new(|| {
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
