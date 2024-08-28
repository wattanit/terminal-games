mod minesweeper;
extern crate ncurses;

use ncurses::*;
use std::env;
use minesweeper::Game;

const DEFAULT_GRID_WIDTH: i32 = 10;
const DEFAULT_GRID_HEIGHT: i32 = 10;
const DEFAULT_MINE_COUNT: i32 = 10;

fn setup_ncurses() {
    initscr();
    raw();
    keypad(stdscr(), true);
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
}

fn cleanup_ncurses() {
    endwin();
}

fn main() {
    // read input arguments
    let args: Vec<String> = env::args().collect();
    let width = args.get(1)
                    .and_then(|arg| arg.parse::<i32>().ok())
                    .unwrap_or(DEFAULT_GRID_WIDTH); // Default to 10 if not provided or invalid
    let height = args.get(2)
                    .and_then(|arg| arg.parse::<i32>().ok())
                    .unwrap_or(DEFAULT_GRID_HEIGHT); // Default to 10 if not provided or invalid
    let mine_count = args.get(3)
                    .and_then(|arg| arg.parse::<i32>().ok())
                    .unwrap_or(DEFAULT_MINE_COUNT); // Default to 10 if not provided or invalid

    setup_ncurses();
    let mut game = Game::new(width, height, mine_count);
    game.run();
    cleanup_ncurses();
}