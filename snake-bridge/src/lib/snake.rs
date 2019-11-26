use crate::lib::constants::{GRID_CELL_COLOR};
use crate::lib::types::{Cell, CellClass, GameOverErr, Grid, SnakeHead};
use std::option::Option;

pub fn update_snakehead_in_grid(grid: &mut Grid, head: &mut SnakeHead) -> Option<GameOverErr> {
    let mut clear_tail = true;

    let head_pos = head.body_positions.last();
    match head_pos {
        None => panic!("snake is empty!"),
        Some(v) => {
            match grid.grid[v.0 as usize][v.1 as usize].class {
                CellClass::Food => {
                    clear_tail = false;
                    grid.num_food -= 1;
                }
                CellClass::Snake => {
                    return Some(GameOverErr);
                }
                _ => {}
            }
            // Q: is there a better way to copy construct this Cell
            grid.grid[v.0 as usize][v.1 as usize] = Cell {
                red: head.color.red,
                green: head.color.green,
                blue: head.color.blue,
                class: CellClass::Snake,
            };
        }
    }

    if head.body_positions.len() <= 1 {
        clear_tail = false;
    }

    if clear_tail {
        let tail_pos = head.body_positions.get(0);
        match tail_pos {
            None => panic!("snake is empty!"),
            Some(v) => {
                grid.grid[v.0 as usize][v.1 as usize] = GRID_CELL_COLOR;
                head.body_positions.remove(0);
            }
        }
    }

    None
}
