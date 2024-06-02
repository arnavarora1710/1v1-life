use macroquad::prelude::*;

const CELL_DIM : f32 = 10.0;

struct Cell {
    x: f32,
    y: f32,
}

impl Cell {
    fn new (x : f32, y : f32) -> Cell {
        Cell { x, y }
    }

    fn draw (&self) {
        draw_rectangle_lines(self.x, self.y, CELL_DIM, CELL_DIM, 1.0, BLACK);
    }
}

pub fn draw_grid() {
    let x_max = screen_width() / CELL_DIM;
    let y_max = screen_height() / CELL_DIM;
    for i in 0..=x_max as i32 {
        for j in 0..=y_max as i32 {
            let cell = Cell::new(i as f32 * CELL_DIM, j as f32 * CELL_DIM);
            cell.draw();
        }
    }
}

#[macroquad::main("breakout")]
async fn main() {
    loop {
        clear_background(WHITE);
        draw_grid();
        next_frame().await
    }
}