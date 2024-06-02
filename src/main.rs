use macroquad::prelude::*;
use scene::clear;

const CELL_DIM : f32 = 10.0;
const DEAD : u8 = 0;
const P1 : u8 = 1;
const P2 : u8 = 2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameState {
    StartScreen,
    SelectScreenP1,
    SelectScreenP2,
    PlayScreen,
    GameOverScreen
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    x: f32,
    y: f32,
    player : u8
}

impl Cell {
    pub fn new (x : f32, y : f32, player : u8) -> Cell {
        Cell { x, y, player }
    }

    pub fn draw (&self) {
        if self.player == DEAD {
            draw_rectangle_lines(self.x, self.y, CELL_DIM, CELL_DIM, 1.0, BLACK);
        } else if self.player == P1 {
            draw_rectangle(self.x, self.y, CELL_DIM, CELL_DIM, RED);
        } else if self.player == P2 {
            draw_rectangle(self.x, self.y, CELL_DIM, CELL_DIM, BLUE);
        }
    }
}

pub struct Grid {
    cells : Vec<Vec<Cell>>
}

impl Grid {
    pub fn new (cells : Vec<Vec<Cell>>) -> Grid {
        Grid { cells }
    }

    pub fn draw (&self) {
        for row in &self.cells {
            for cell in row {
                cell.draw();
            }
        }
    }

    pub fn sim (&mut self) {
        for i in 0..self.cells.len() {
            for j in 0..self.cells[i].len() {
                let mut p1_count = 0;
                let mut p2_count = 0;
                for x in -1..=1 {
                    for y in -1..=1 {
                        if x == 0 && y == 0 {
                            continue;
                        }
                        let new_x = i as i32 + x;
                        let new_y = j as i32 + y;
                        if new_x < 0 || new_x >= self.cells.len() as i32 || new_y < 0 || new_y >= self.cells[i].len() as i32 {
                            continue;
                        }
                        if self.cells[new_x as usize][new_y as usize].player == P1 {
                            p1_count += 1;
                        } else if self.cells[new_x as usize][new_y as usize].player == P2 {
                            p2_count += 1;
                        }
                    }
                }
                // p2life rules
                // https://www.dcs.bbk.ac.uk/~gr/pdf/p2life.pdf
                if self.cells[i][j].player == P1 {
                    let diff : i32 = p1_count as i32 - p2_count as i32;
                    if diff == 2 || diff == 3 || (diff == 1 && p1_count >= 2) {
                        self.cells[i][j].player = P1;
                    } else {
                        self.cells[i][j].player = DEAD;
                    }
                } else if self.cells[i][j].player == P2 {
                    let diff : i32 = p2_count as i32 - p1_count as i32;
                    if diff == 2 || diff == 3 || (diff == 1 && p2_count >= 2) {
                        self.cells[i][j].player = P2;
                    } else {
                        self.cells[i][j].player = DEAD;
                    }
                } else {
                    if p1_count == 3 && p2_count != 3 {
                        self.cells[i][j].player = P1;
                    } else if p2_count == 3 && p1_count != 3 {
                        self.cells[i][j].player = P2;
                    } else if p1_count == 3 && p2_count == 3 {
                        // flip a coin to decide if P1 or P2
                        let coin = rand::gen_range(0, 2);
                        if coin == 0 {
                            self.cells[i][j].player = P1;
                        } else {
                            self.cells[i][j].player = P2;
                        }
                    }
                }
            }
        }
    }
}

pub fn grid_init() -> Grid {
    let x_max = screen_width() / CELL_DIM;
    let y_max = screen_height() / CELL_DIM;
    let mut cells = Vec::new();
    for i in 0..=x_max as i32 {
        let mut row = Vec::new();
        for j in 0..=y_max as i32 {
            let cell = Cell::new(i as f32 * CELL_DIM, j as f32 * CELL_DIM, DEAD);
            row.push(cell);
        }
        cells.push(row);
    }
    Grid::new(cells)
}

#[macroquad::main("breakout")]
async fn main() {
    let mut grid : Grid = grid_init();
    let mut frame_count : i64 = 0;
    let mut grid_count : u16 = 0;
    let mut cur_game_state = GameState::StartScreen;
    loop {
        clear_background(WHITE);
        // Draw grid after 15 frames
        if grid_count <= 15 {
            grid = grid_init();
            grid_count += 1;
        } else {
            if cur_game_state == GameState::StartScreen {
                draw_text("Press Enter to Start", screen_width() / 3.0, screen_height() / 2.0, 30.0, BLACK);
                if is_key_pressed(KeyCode::Enter) {
                    cur_game_state = GameState::SelectScreenP1;
                }
            } else if cur_game_state == GameState::SelectScreenP1 {
                grid.draw();
                if is_mouse_button_pressed(MouseButton::Left) {
                    let x = mouse_position().0;
                    let y = mouse_position().1;
                    let x_index = (x / CELL_DIM) as i32;
                    let y_index = (y / CELL_DIM) as i32;
                    grid.cells[x_index as usize][y_index as usize].player = P1;
                }
                if is_key_pressed(KeyCode::Enter) {
                    cur_game_state = GameState::SelectScreenP2;
                }
            } else if cur_game_state == GameState::SelectScreenP2 {
                grid.draw();
                if is_mouse_button_pressed(MouseButton::Left) {
                    let x = mouse_position().0;
                    let y = mouse_position().1;
                    let x_index = (x / CELL_DIM) as i32;
                    let y_index = (y / CELL_DIM) as i32;
                    grid.cells[x_index as usize][y_index as usize].player = P2;
                }
                if is_key_pressed(KeyCode::Enter) {
                    cur_game_state = GameState::PlayScreen;
                }
            } else if cur_game_state == GameState::PlayScreen {
                grid.draw();
                if frame_count % 60 == 0 {
                    grid.sim();
                }
                if is_key_pressed(KeyCode::Enter) {
                    cur_game_state = GameState::GameOverScreen;
                }
            } else if cur_game_state == GameState::GameOverScreen {
                draw_text("Game Over!", screen_width() / 2.5, screen_height() / 2.0, 30.0, RED);
            }
        }
        frame_count += 1;
        next_frame().await
    }
}