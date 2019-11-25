use crate::lib::constants::{GRID_CELL_COLOR, GRID_COLUMNS, GRID_ROWS};
use crate::lib::types::{Cell, Grid, SnakeHead, CellClass};

pub fn update_snakehead_location(head: &mut SnakeHead, direction: (i32, i32)) {
    let head_pos = head.body_positions.last();
    let mut new_row;
    let mut new_column;
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

    head.body_positions.push((new_row, new_column));
    head.last_tail_position = head.body_positions.remove(0);
}

pub fn update_snakehead_in_grid(grid: &mut Grid, head: &SnakeHead) {
    let head_pos = head.body_positions.last();
    match head_pos {
        None => panic!("snake is empty!"),
        Some(v) => {
            grid.grid[v.0 as usize][v.1 as usize] = Cell {
                red: head.color.red,
                green: head.color.green,
                blue: head.color.blue,
                class: CellClass::Snake,
            };
        }
    }

    grid.grid[head.last_tail_position.0 as usize][head.last_tail_position.1 as usize] =
        GRID_CELL_COLOR;
}
