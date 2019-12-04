use std::thread;
use std::time;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::lib::constants;
use crate::lib::snake;
use crate::lib::types::{Cell, CellClass, GameOverErr, SnakeHead, Grid, Direction};

pub mod lib;

fn main() {
    let (mut canvas, mut events) = lib::init(constants::CANVAS_WIDTH, constants::CANVAS_HEIGHT);

    let mut grid = Grid::new(constants::GRID_COLUMNS, constants::GRID_ROWS);
    let mut direction = Direction::Left;
    let mut snakehead = SnakeHead {
        body_positions: vec![(
            constants::GRID_ROWS / 2,
            constants::GRID_COLUMNS / 2,
        )],
        cell: Cell {
            red: 0,
            green: 0,
            blue: 255,
            class: CellClass::Snake,
        },
    };

    snake::update_snakehead_in_grid(&mut grid, &mut snakehead);

    'game: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => direction = Direction::Up,
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => direction = Direction::Down,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => direction = Direction::Left,
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => direction = Direction::Right,
                _ => continue 'game,
            }
        }
        // randomly add food to grid
        grid.replenish_food();

        snakehead.move_forward(&direction);

        match snake::update_snakehead_in_grid(&mut grid, &mut snakehead) {
            Some(GameOverErr) => panic!("{}", GameOverErr),
            None => {}
        }
        lib::display_frame(
            &mut canvas,
            &grid,
            &constants::GRID_ROWS,
            &constants::GRID_COLUMNS,
            &constants::CELL_WIDTH,
        );
        thread::sleep(time::Duration::from_millis(100));
    }
}
