pub struct Cell {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub struct Grid {
    pub grid: Vec<Vec<Cell>>,
}

pub struct SnakeHead {
    pub color: Cell,
    pub body_positions: Vec<(i32, i32)>,
    pub last_tail_position: (i32, i32),
}
// TODO make location update a SnakeHead implementation
