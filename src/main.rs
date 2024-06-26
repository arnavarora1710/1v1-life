use macroquad::prelude::*;

const CELL_DIM : f32 = 75.0;
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

#[derive(Clone, Debug)]
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
    pub fn check_winner(&self) -> u8 {
        let mut p1_count = 0;
        let mut p2_count = 0;
        for i in 0..self.cells.len() {
            for j in 0..self.cells[i].len() {
                if self.cells[i][j].player == P1 {
                    p1_count += 1;
                } else if self.cells[i][j].player == P2 {
                    p2_count += 1;
                }
            }
        }
        if p1_count == 0 {
            P2
        } else if p2_count == 0 {
            P1
        } else {
            DEAD
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
    let mut count_selected_p1 = 0;
    let mut count_selected_p2 = 0;
    let mut winner : u8 = DEAD;
    let mut last_few_frames = 0;
    loop {
        clear_background(WHITE);
        // Draw grid after 15 frames
        if grid_count <= 15 {
            grid = grid_init();
            grid_count += 1;
        } else {
            if cur_game_state == GameState::StartScreen {
                // add game customization options and a dialog box that displays rules
                if is_key_down(KeyCode::R) {
                    // Display rules dialog box
                    draw_rectangle(screen_width() / 4.0, screen_height() / 4.0, screen_width() / 2.0, screen_height() / 2.0, WHITE);
                    draw_text("Game Rules:", screen_width() / 3.5, screen_height() / 3.5, 20.0, BLACK);
                    draw_text("- Each player can select up to 10 cells on the grid.", screen_width() / 4.0, screen_height() / 3.0, 15.0, BLACK);
                    draw_text("- The game starts with the Start Screen.", screen_width() / 4.0, screen_height() / 2.8, 15.0, BLACK);
                    draw_text("- Press Enter to move to the Select Screen for Player 1.", screen_width() / 4.0, screen_height() / 2.6, 15.0, BLACK);
                    draw_text("- Left-click on cells to select or deselect them.", screen_width() / 4.0, screen_height() / 2.4, 15.0, BLACK);
                    draw_text("- Press Enter to move to the Select Screen for Player 2.", screen_width() / 4.0, screen_height() / 2.2, 15.0, BLACK);
                    draw_text("- The game progresses to the Play Screen.", screen_width() / 4.0, screen_height() / 2.0, 15.0, BLACK);
                    draw_text("- The grid is updated every 60 frames.", screen_width() / 4.0, screen_height() / 1.8, 15.0, BLACK);
                    draw_text("- Press Enter to move to the Game Over Screen.", screen_width() / 4.0, screen_height() / 1.6, 15.0, BLACK);
                    draw_text("- The Game Over Screen displays 'Game Over!'.", screen_width() / 4.0, screen_height() / 1.5, 15.0, BLACK);
                } else {
                    draw_text("Press Enter to Start", screen_width() / 3.0, screen_height() / 2.0, 30.0, BLACK);
                    draw_text("Press R to view rules", screen_width() / 3.0, screen_height() / 1.8, 20.0, BLACK);
                }
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
                    let player : u8 = grid.cells[x_index as usize][y_index as usize].player;
                    if player == P1 {
                        grid.cells[x_index as usize][y_index as usize].player = DEAD;
                        count_selected_p1 -= 1;
                    } else if count_selected_p1 < 10 {
                        grid.cells[x_index as usize][y_index as usize].player = P1;
                        count_selected_p1 += 1;
                    }
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
                    let player : u8 = grid.cells[x_index as usize][y_index as usize].player;
                    if player == P2 {
                        grid.cells[x_index as usize][y_index as usize].player = DEAD;
                        count_selected_p2 -= 1;
                    } else if player == DEAD && count_selected_p2 < 10 {
                        grid.cells[x_index as usize][y_index as usize].player = P2;
                        count_selected_p2 += 1;
                    }
                }
                if is_key_pressed(KeyCode::Enter) {
                    cur_game_state = GameState::PlayScreen;
                }
            } else if cur_game_state == GameState::PlayScreen {
                grid.draw();
                let old_grid = grid.clone();
                if frame_count % 30 == 0 {
                    grid.sim();
                }
                winner = grid.check_winner();
                if winner != DEAD || old_grid.cells == grid.cells {
                    last_few_frames += 1;
                }
                if last_few_frames == 120 {
                    cur_game_state = GameState::GameOverScreen;
                }
            } else if cur_game_state == GameState::GameOverScreen {
                if winner == P1 {
                    draw_text("Player 1 Wins!", screen_width() / 2.5, screen_height() / 2.5, 30.0, RED);
                } else if winner == P2 {
                    draw_text("Player 2 Wins!", screen_width() / 2.5, screen_height() / 2.5, 30.0, BLUE);
                } else {
                    draw_text("It's a Draw!", screen_width() / 2.5, screen_height() / 2.5, 30.0, BLACK);
                }
                draw_text("Game Over!", screen_width() / 2.5, screen_height() / 2.0, 30.0, GREEN);
            }
        }
        frame_count += 1;
        next_frame().await
    }
}