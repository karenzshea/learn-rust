use rand;
use std::error;
use std::fmt;

use crate::lib::constants::{FOOD_CELL_COLOR, GRID_COLUMNS, GRID_ROWS};

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
            return;
        }

        let mut rng = rand::thread_rng();
        let xs: rand::seq::index::IndexVec =
            rand::seq::index::sample(&mut rng, GRID_ROWS as usize, missing as usize);
        let ys: rand::seq::index::IndexVec =
            rand::seq::index::sample(&mut rng, GRID_COLUMNS as usize, missing as usize);

        for food_iter in xs.iter().zip(ys.iter()) {
            // Q: can Cell{} be default constructed/copy constructed from FOOD_CELL_COLOR constant
            if self.grid[food_iter.0][food_iter.1].class == CellClass::Snake {
                continue;
            }
            self.grid[food_iter.0][food_iter.1] = Cell {
                red: FOOD_CELL_COLOR.red,
                green: FOOD_CELL_COLOR.green,
                blue: FOOD_CELL_COLOR.blue,
                class: CellClass::Food,
            };
            self.num_food += 1;
        }
    }
}

pub struct SnakeHead {
    pub color: Cell,
    pub body_positions: Vec<(i32, i32)>,
}

impl SnakeHead {
    pub fn move_forward(&mut self, direction: &(i32, i32)) {
        // body_positions vector looks like
        // tail      head
        //  <---------:>
        let mut new_row;
        let mut new_column;
        let head_pos = &self.body_positions.get(&self.body_positions.len() - 1);
        match head_pos {
            None => panic!("snake is empty!"),
            Some(v) => {
                new_row = v.0 + direction.0;
                new_column = v.1 + direction.1;
            }
        }

        // if snake hits wall, come out the other side
        if new_row < 0 {
            new_row = (GRID_ROWS - 1) as i32;
        } else if new_row == GRID_ROWS as i32 {
            new_row = 0;
        }

        if new_column < 0 {
            new_column = (GRID_COLUMNS - 1) as i32;
        } else if new_column == GRID_COLUMNS as i32 {
            new_column = 0;
        }

        &self.body_positions.push((new_row, new_column));
    }
}

#[derive(Debug, Clone)]
pub struct GameOverErr;

impl error::Error for GameOverErr {
    fn description(&self) -> &str {
        "SNAKE ATE ITSELF \nGAME OVER"
    }
}

impl fmt::Display for GameOverErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GameOverErr")
    }
}
