use ncurses::*;
use std::cmp::{max, min};

pub struct Game {
    grid: Vec<Vec<char>>,
    bomb_grid: Vec<Vec<bool>>,
    cursor_x: i32,
    cursor_y: i32,
    grid_width: i32,
    grid_height: i32,
    number_of_mines: i32,
    initialized: bool,
}

impl Game {
    pub fn new(width: i32, height: i32, mine_count: i32) -> Self {
        let grid = vec![vec!['.'; width as usize]; height as usize];
        let bomb_grid = vec![vec![false; width as usize]; height as usize];
        Game {
            grid,
            bomb_grid,
            cursor_x: 0,
            cursor_y: 0,
            grid_width: width,
            grid_height: height,
            number_of_mines: mine_count,
            initialized: false,
        }
    }

    fn get_cell_coordinates(&self) -> Vec<(i32, i32)> {
        let mut coordinates = Vec::new();
        for i in 0..self.grid_height {
            for j in 0..self.grid_width {
                coordinates.push((i, j));
            }
        }
        coordinates
    }

    fn draw(&self) {
        clear();

        // draw the grid
        for i in 0..self.grid_height {
            for j in 0..self.grid_width {
                // if the cursor is on this cell, highlight it
                if i == self.cursor_y && j == self.cursor_x {
                    self.highlight_cell(i, j);
                } else {
                    mvprintw(i, j * 2, &self.grid[i as usize][j as usize].to_string());
                }
            }
        }

         // Print the quit instruction below the grid
         mvprintw(self.grid_height+1, 0, "Press 'space' to open | 'b' to flag mine | 'q' to quit.");

        refresh();
    }

    fn highlight_cell(&self, row: i32, col: i32) {
        attron(A_REVERSE());
        mvprintw(row, col * 2, &self.grid[row as usize][col as usize].to_string());
        attroff(A_REVERSE());
    }

    fn move_cursor(&mut self, direction: i32) {
        match direction {
            KEY_UP => {
                self.cursor_y = max(self.cursor_y - 1, 0);
            }
            KEY_DOWN => {
                self.cursor_y = min(self.cursor_y + 1, self.grid_height - 1);
            }
            KEY_LEFT => {
                self.cursor_x = max(self.cursor_x - 1, 0);
            }
            KEY_RIGHT => {
                self.cursor_x = min(self.cursor_x + 1, self.grid_width - 1);
            }
            _ => {}
        }
    }

    fn init_game(&mut self) {
        // Initialize the game
        self.initialized = true;

        // Place mines
        let available_cells = self.get_cell_coordinates();

        // remove all cells with manhattan distance less than half the width of the grid from the cursor
        let cursor_x = self.cursor_x;
        let cursor_y = self.cursor_y;
        let available_cells = available_cells.into_iter().filter(|(x, y)| {
            let dx = (cursor_x - x).abs();
            let dy = (cursor_y - y).abs();
            dx + dy > self.grid_width / 2
        }).collect::<Vec<(i32, i32)>>();
        
        // shuffle the available cells

        // place mines

    }

    fn open_cell(&mut self) {
        if !self.initialized {
            // Initialize the game
            self.init_game()
        }

        // Open the cell
        let cell = self.grid[self.cursor_y as usize][self.cursor_x as usize];
        if cell == '.' {
            // open the unopened cell
            self.grid[self.cursor_y as usize][self.cursor_x as usize] = ' ';
        } else if cell == 'F' {
            // Do nothing if the cell is flagged
        } else {
            // check for conditions to open surrounding cells
        }
    }

    fn flag_mine(&mut self) {
        // Flag the cell
        let cell = self.grid[self.cursor_y as usize][self.cursor_x as usize];
        if cell == 'F' {
            // flag the unopened cell
            self.grid[self.cursor_y as usize][self.cursor_x as usize] = '.';
        } else if cell == '.' {
            // unflag the unopened cell
            self.grid[self.cursor_y as usize][self.cursor_x as usize] = 'F';
        } else {
            // Do nothing if the cell is already opened
        }
    }

    pub fn run(&mut self) {
        loop {
            self.draw();
            let ch = getch();

            match ch {
                113 => break, // 'q' key to quit
                32 => self.open_cell(), // 'space' key to open cell
                98 => self.flag_mine(), // 'b' key to flag mine
                102 => self.flag_mine(), // 'f' key to flag mine
                _ => self.move_cursor(ch),
            }
        }
    }
}