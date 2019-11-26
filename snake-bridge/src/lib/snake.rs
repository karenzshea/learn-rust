use crate::lib::constants::{GRID_CELL_COLOR, GRID_COLUMNS, GRID_ROWS};
use crate::lib::types::{Cell, Grid, SnakeHead, CellClass};

pub fn update_snakehead_location(head: &mut SnakeHead, direction: (i32, i32)) {
    // body_positions vector looks like
    // tail      head
    //  <---------:>
    let mut new_row;
    let mut new_column;
    let head_pos = head.body_positions.get(head.body_positions.len() - 1);
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
}

pub fn update_snakehead_in_grid(grid: &mut Grid, head: &mut SnakeHead) {
    let mut clear_tail = true;

    let head_pos = head.body_positions.last();
    match head_pos {
        None => panic!("snake is empty!"),
        Some(v) => {
            if grid.grid[v.0 as usize][v.1 as usize].class == CellClass::Food {
                clear_tail = false;
                grid.num_food -= 1;
            }

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
}
