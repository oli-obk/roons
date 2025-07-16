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

    fn index(&self, x: f32, y: f32) -> (usize, usize) {
        let x = x / WIDTH;
        let y = y / HEIGHT;
        let x = x as usize;
        let y = y as usize;
        (x, y)
    }

    fn insert_ball(&mut self, x: f32, y: f32) {
        let (x, y) = self.index(x, y);
        let Some(row) = self.rows.get_mut(y) else {
            return;
        };
        // Balls are two cells wide, so we need to check that we don't collide with an existing ball left of us or right of us
        if row.cells[x.saturating_sub(1)..(x + 1).min(row.cells.len())]
            .iter()
            .any(|cell| cell.ball)
        {
            return;
        };
        row.cells[x].ball = true;
    }

    fn draw(&self, x_start: f32, y_start: f32) {
        let mut active = self.active;
        let highlight = color_u8!(0, 0, 255, 50);
        let y_end = HEIGHT * (self.rows.len() as f32) + y_start;
        let mut y = HEIGHT;

        let x_end = WIDTH * (self.rows[0].cells.len() as f32) + x_start;

        let mut x = x_start;
        for _ in 0..=32 {
            draw_line(x, y_start, x, y_end, 0.5, GRAY);
            x += WIDTH;
        }

        for row in self.rows.iter() {
            draw_line(x_start, y - y_start, x_end, y - y_start, 0.5, GRAY);
            if !active {
                draw_line(x_start, y, x_end, y, HEIGHT - 2., highlight);
            }

            let mut x = x_start + WIDTH / 2.;
            for cell in row.cells.iter() {
                draw_circle_lines(x, y - HEIGHT / 5., WIDTH / 6., 1.0, GRAY.with_alpha(0.5));
                draw_circle_lines(x, y + HEIGHT / 5., WIDTH / 6., 1.0, GRAY.with_alpha(0.5));
                x += WIDTH;
            }

            let mut x = x_start;
            for cell in row.cells.iter() {
                if cell.ball {
                    draw_circle(x + WIDTH, y, WIDTH - 1., RED);
                }
                x += WIDTH;
            }

            active = !active;
            y += HEIGHT;
        }
        draw_line(x_start, y - y_start, x_end, y - y_start, 1., GRAY);
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
struct Row {
    cells: Box<[Cell; 32]>,
}

#[derive(Copy, Clone, Default, Serialize, Deserialize)]
struct Cell {
    roon: Roon,
    // Whether this is a left edge of a ball
    ball: bool,
}

#[derive(Copy, Clone, Default, Serialize, Deserialize)]
enum Roon {
    Up,
    Down,
    Left,
    Right,
    #[default]
    Nop,
}
const HEIGHT: f32 = 20.;
const WIDTH: f32 = 10.;

#[macroquad::main("roons")]
async fn main() {
    let mut grid: Loom = std::fs::read("loom.json")
        .map(|v| serde_json::from_slice(&v).unwrap())
        .unwrap_or_default();

    set_window_size(33 * WIDTH as u32, 33 * HEIGHT as u32);
    prevent_quit();

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

        let offset_x = WIDTH as f32 / 2.;
        let offset_y = HEIGHT as f32 / 2.;

        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            grid.insert_ball(x - offset_x, y - offset_y);
        }

        grid.draw(offset_x, offset_y);

        next_frame().await
    }

    serde_json::to_writer(std::fs::File::create("loom.json").unwrap(), &grid).unwrap();
}
