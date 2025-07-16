use macroquad::{miniquad::window::set_window_size, prelude::*};
use serde::{Deserialize, Serialize};

enum Mode {
    Filled(Color),
    Line(Color),
}

fn draw_square(pos: Vec2, mode: Mode, s: f32) {
    match mode {
        Mode::Filled(color) => draw_rectangle_lines(pos.x, pos.y, s, s, 1.0, color),
        Mode::Line(color) => draw_rectangle(pos.x, pos.y, s, s, color),
    };
}

#[derive(Default, Serialize, Deserialize)]
struct Loom {
    rows: Box<[Row; 32]>,
    /// `false` if even rows are up, `true` if odd rows are up
    active: bool,
}

impl Loom {
    fn tick(&mut self) {
        let mut new = self.rows.clone();
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
struct Row {
    cells: Box<[Cell; 32]>,
    // left edge of a ball
    balls: Box<[bool; 32]>,
}

#[derive(Copy, Clone, Default, Serialize, Deserialize)]
enum Cell {
    Up,
    Down,
    Left,
    Right,
    #[default]
    Nop,
}

#[macroquad::main("roons")]
async fn main() {
    let mut grid: Loom = std::fs::read("loom.json")
        .map(|v| serde_json::from_slice(&v).unwrap())
        .unwrap_or_default();

    let height = 20;
    let width = 10;
    set_window_size(33 * width, 33 * height);
    prevent_quit();
    let height = height as f32;
    let width = width as f32;

    let mut last_tick = get_time();

    while !is_key_pressed(KeyCode::Escape) && !is_quit_requested() {
        clear_background(BLACK);

        if is_key_down(KeyCode::Space) {
            if get_time() - last_tick > 1. {
                grid.tick();
            }
        } else {
            last_tick = get_time();
        }

        let mut active = grid.active;
        let highlight = color_u8!(0, 0, 255, 50);
        let y_end = height * (grid.rows.len() as f32 + 0.5);
        let y_start = height / 2.;
        let mut y = height;

        let x_end = width * (grid.rows[0].cells.len() as f32 + 0.5);
        let x_start = width / 2.;

        let mut x = x_start;
        for _ in 0..=32 {
            draw_line(x, y_start, x, y_end, 0.5, GRAY);
            x += width;
        }

        for row in grid.rows.iter() {
            draw_line(x_start, y - y_start, x_end, y - y_start, 0.5, GRAY);
            if !active {
                draw_line(x_start, y, x_end, y, height - 2., highlight);
            }

            let mut x = x_start + width / 2.;
            for cell in row.cells.iter() {
                draw_circle_lines(x, y - height / 5., width / 6., 1.0, GRAY.with_alpha(0.5));
                draw_circle_lines(x, y + height / 5., width / 6., 1.0, GRAY.with_alpha(0.5));
                x += width;
            }

            active = !active;
            y += height;
        }
        draw_line(x_start, y - y_start, x_end, y - y_start, 1., GRAY);

        next_frame().await
    }

    serde_json::to_writer(std::fs::File::create("loom.json").unwrap(), &grid).unwrap();
}
