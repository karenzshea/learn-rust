use rand;
use std::error;
use std::fmt;
use std::option::Option;

use crate::lib::constants::{FOOD_CELL_COLOR, GRID_COLUMNS, GRID_ROWS};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CellClass {
    Snake,
    Food,
    Empty,
}

#[derive(Clone, Copy)]
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
    pub fn update_cell(&mut self, coord: &(usize, usize), cell: &Cell) -> Option<InvalidCellErr> {
        if coord.0 >= GRID_ROWS as usize || coord.1 >= GRID_COLUMNS as usize {
            return Some(InvalidCellErr);
        }

        self.grid[coord.0 as usize][coord.1 as usize] = *cell;
        None
    }
    pub fn update_cells(
        &mut self,
        coords: &Vec<(usize, usize)>,
        cell: &Cell,
    ) -> Option<InvalidCellErr> {
        for coord in coords {
            match self.update_cell(coord, cell) {
                Some(InvalidCellErr) => return Some(InvalidCellErr),
                _ => {}
            }
        }
        None
    }
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
            if self.grid[food_iter.0][food_iter.1].class == CellClass::Snake {
                continue;
            }
            self.update_cell(
                &(food_iter.0 as usize, food_iter.1 as usize),
                &FOOD_CELL_COLOR,
            );
            self.num_food += 1;
        }
    }
}

pub struct SnakeHead {
    pub cell: Cell,
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

#[derive(Debug, Clone)]
pub struct InvalidCellErr;

impl error::Error for InvalidCellErr {
    fn description(&self) -> &str {
        "Provided cell is invalid on this grid"
    }
}

impl fmt::Display for InvalidCellErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InvalidCellErr")
    }
}
