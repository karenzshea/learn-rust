use rand;
use std::error;
use std::fmt;

use crate::lib::constants::{FOOD_CELL_COLOR, GRID_COLUMNS, GRID_ROWS};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CellClass {
    Snake,
    Food,
    Empty,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub class: CellClass,
}

pub struct Grid {
    // TODO: expose method for getting cell value within grid
    pub grid: Vec<Vec<Cell>>,
    max_food: u32,
    num_food: u32,
}

impl Grid {
    pub fn new(nx_cells: u32, ny_cells: u32) -> Self {
        let mut grid_vector = Vec::new();

        for row in 0..ny_cells {
            grid_vector.push(Vec::new());
            for _column in 0..nx_cells {
                grid_vector[row as usize].push(Cell {
                    red: 35_u8,
                    green: 15_u8,
                    blue: 13_u8,
                    class: CellClass::Empty,
                });
            }
        }
        let max_food: u32 =
            ((GRID_ROWS * GRID_COLUMNS) as f32 * 0.005).floor() as u32;
        let grid = Grid {
            grid: grid_vector,
            max_food: max_food,
            num_food: 0,
        };

        grid
    }

    pub fn decrease_food(&mut self, by: u32) {
        self.num_food -= by
    }

    pub fn update_cell(&mut self, coord: &(usize, usize), cell: &Cell) -> Result<(), InvalidCellErr> {
        if coord.0 >= GRID_ROWS as usize || coord.1 >= GRID_COLUMNS as usize {
            return Err(InvalidCellErr);
        }

        self.grid[coord.0 as usize][coord.1 as usize] = *cell;
        Ok(())
    }
    pub fn update_cells(
        &mut self,
        coords: &Vec<(usize, usize)>,
        cell: &Cell,
    ) -> Result<(), InvalidCellErr> {
        for coord in coords {
            // ? operator, try's function call, returns err if err, unwraps result on success;
            self.update_cell(coord, cell)?;
        }
        Ok(())
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
            let update_result = self.update_cell(
                &(food_iter.0 as usize, food_iter.1 as usize),
                &FOOD_CELL_COLOR,
            );
            match update_result {
                Err(err) => {
                    panic!("Could not update cell {}", err)
                }
                _ => {}
            };
            self.num_food += 1;
        }
    }
}

pub struct SnakeHead {
    pub cell: Cell,
    pub body_positions: Vec<(u32, u32)>,
}

impl SnakeHead {
    pub fn move_forward(&mut self, direction: &Direction) {
        // body_positions vector looks like
        // tail      head
        //  <---------:>
        let mut new_row;
        let mut new_column;
        let head_pos = &self.body_positions.get(&self.body_positions.len() - 1);
        match head_pos {
            None => panic!("snake is empty!"),
            Some(v) => {
                new_row = v.0;
                new_column = v.1;
                match direction {
                    Direction::Up => {
                        new_row = (v.0 + GRID_ROWS - 1) % GRID_ROWS;
                    },
                    Direction::Down => {
                        new_row = (v.0 + GRID_ROWS + 1) % GRID_ROWS;
                    },
                    Direction::Left => {
                        new_column = (v.1 + GRID_COLUMNS - 1) % GRID_COLUMNS;
                    },
                    Direction::Right => {
                        new_column = (v.1 + GRID_COLUMNS + 1) % GRID_COLUMNS;
                    },
                }
            }
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
