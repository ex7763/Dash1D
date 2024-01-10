use std::iter::once;
use is_close::is_close;
use is_close::default;
use macroquad::ui::{hash, root_ui, widgets};
use macroquad::prelude::*;


fn window_conf() -> Conf {
    Conf {
        window_title: "Dash 1D".to_owned(),
        fullscreen: false,
        window_width: 512,
        window_height: 256,
        high_dpi: false,
        ..Default::default()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Status {
    Idle,
    Dash,
    Die,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut x = 256.0;
    let mut y = 128.0 - 12.0;

    let mut xa = 0.0;
    let eps = 1e-9;
    let eps = 1e-5;

    let mut status = Status::Idle;


    let mut boob_x = 400.0;
    let mut boob_r = 5.0;
    loop {
        clear_background(WHITE);

        draw_line(
            0.0, 156.0,
            512.0, 156.0,
            5.,
            BLACK,
        );

        draw_line(
            0.0, 100.0,
            512.0, 100.0,
            5.,
            BLACK,
        );

        draw_rectangle(x - 12.0, y, 24.0, 24.0, GREEN);
        draw_circle_lines(boob_x, 128.0, boob_r, 3.0, RED);
        boob_r += 0.1;

        if status == Status::Die {
            draw_text("You Die", 110.0, 100.0, 96.0, RED);
        }

        if status == Status::Idle {
            if (x < boob_x + boob_r + 3.0 && x > boob_x + boob_r - 3.0) ||
               (x < boob_x - boob_r + 3.0 && x > boob_x - boob_r - 3.0) {
                status = Status::Die;
                println!("Die");
            }
            else {
                match get_last_key_pressed() {
                    Some(key) => {
                        match key {
                            KeyCode::Right => {xa += 4.0;},
                            KeyCode::Left=> {xa -= 4.0;},
                            _ => {}
                        }
                        println!("{:?}", key);
                    }
                    _ => {}
                }
            }
        }

        if status != Status::Die {
            if is_close!(xa, 0.0, rel_tol=eps) {
                status = Status::Idle;
                xa = 0.0;
            }
            else {
                if xa > 0.1 {
                    xa -= 0.1;
                }
                else if xa < -0.1 {
                    xa += 0.1;
                }
                else {
                    xa = 0.0;
                }
                status = Status::Dash;
            }
        }

        x += xa;
        x = f32::max(f32::min(x, 512.0), 0.0);
        // println!("{x}, {xa}");


        next_frame().await;
    }
}