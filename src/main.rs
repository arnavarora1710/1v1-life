use macroquad::prelude::*;

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
    let mut count : u16 = 0;
    let mut curGameState = GameState::StartScreen;
    loop {
        clear_background(WHITE);
        // Draw grid after 15 frames
        if count <= 15 {
            grid = grid_init();
            count += 1;
        } else {
            if curGameState == GameState::StartScreen {
                draw_text("Press Enter to Start", screen_width() / 3.0, screen_height() / 2.0, 30.0, BLACK);
                if is_key_pressed(KeyCode::Enter) {
                    curGameState = GameState::SelectScreenP1;
                }
            } else if curGameState == GameState::SelectScreenP1 {
                grid.draw();
                if is_mouse_button_pressed(MouseButton::Left) {
                    let x = mouse_position().0;
                    let y = mouse_position().1;
                    let x_index = (x / CELL_DIM) as i32;
                    let y_index = (y / CELL_DIM) as i32;
                    grid.cells[x_index as usize][y_index as usize].player = P1;
                }
                if is_key_pressed(KeyCode::Enter) {
                    curGameState = GameState::SelectScreenP2;
                }
            } else if curGameState == GameState::SelectScreenP2 {
                grid.draw();
                if is_mouse_button_pressed(MouseButton::Left) {
                    let x = mouse_position().0;
                    let y = mouse_position().1;
                    let x_index = (x / CELL_DIM) as i32;
                    let y_index = (y / CELL_DIM) as i32;
                    grid.cells[x_index as usize][y_index as usize].player = P2;
                }
                if is_key_pressed(KeyCode::Enter) {
                    curGameState = GameState::PlayScreen;
                }
            } else if curGameState == GameState::PlayScreen {
                grid.draw();
            } else if curGameState == GameState::GameOverScreen {
                draw_text("Game Over", 100.0, 100.0, 30.0, RED);
            }
        }
        next_frame().await
    }
}