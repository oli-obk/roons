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
    set_window_size(33 * height, 33 * height);
    prevent_quit();
    let height = height as f32;

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
        let end = height * (grid.rows.len() as f32 + 0.5);
        let start = height / 2.;
        let mut y = height;

        let mut x = start;
        for _ in 0..=32 {
            draw_line(x, start, x, end, 1., GRAY);
            x += height;
        }

        for row in grid.rows.iter() {
            draw_line(start, y - start, end, y - start, 1., GRAY);
            if !active {
                draw_line(start, y, end, y, height - 2., highlight);
            }

            let mut x = start;
            for cell in row.cells.iter() {}

            active = !active;
            y += height;
        }
        draw_line(start, y - start, end, y - start, 1., GRAY);

        next_frame().await
    }

    serde_json::to_writer(std::fs::File::create("loom.json").unwrap(), &grid).unwrap();
}
