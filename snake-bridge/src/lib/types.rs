use rand;

use crate::lib::constants::{ GRID_ROWS, GRID_COLUMNS, FOOD_CELL_COLOR };

#[derive(Debug, PartialEq)]
pub enum CellClass {
    Snake,
    Food,
    Empty,
}

pub struct Cell {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub class: CellClass,
}

pub struct Grid {
    pub grid: Vec<Vec<Cell>>,
    // Q: how to make this non-public but still accessible during construction?
    pub max_food: u32,
    pub num_food: u32,
}

impl Grid {
    pub fn replenish_food(&mut self) {
        let missing = self.max_food - self.num_food;

        if missing <= 0 {
            return
        }

        let mut rng = rand::thread_rng();
        let xs: rand::seq::index::IndexVec = rand::seq::index::sample(&mut rng, GRID_ROWS as usize, missing as usize);
        let ys: rand::seq::index::IndexVec = rand::seq::index::sample(&mut rng, GRID_COLUMNS as usize, missing as usize);

        for food_iter in xs.iter().zip(ys.iter()) {
            // Q: can Cell{} be default constructed/copy constructed from FOOD_CELL_COLOR constant
            self.grid[food_iter.0][food_iter.1] = Cell { red: FOOD_CELL_COLOR.red, green: FOOD_CELL_COLOR.green, blue: FOOD_CELL_COLOR.blue, class: CellClass::Food };
            self.num_food += 1;
        }
    }
}

pub struct SnakeHead {
    pub color: Cell,
    pub body_positions: Vec<(i32, i32)>,
    pub last_tail_position: (i32, i32),
}
// TODO make location update a SnakeHead implementation
