use super::types::{ Cell, CellClass };

pub const CANVAS_WIDTH: u32 = 720;
pub const CANVAS_HEIGHT: u32 = 720;

pub const GRID_ROWS: u32 = 60;
pub const GRID_COLUMNS: u32 = 60;

pub const CELL_WIDTH: u32 = 12;

pub const GRID_CELL_COLOR: Cell = Cell {
    red: 35_u8,
    green: 15_u8,
    blue: 13_u8,
    class: CellClass::Empty,
};

pub const FOOD_CELL_COLOR: Cell = Cell {
    red: 255_u8,
    green: 0_u8,
    blue: 0_u8,
    class: CellClass::Food,
};
