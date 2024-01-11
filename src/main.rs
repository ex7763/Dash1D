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

    let mut score: u32 = 0;
    let speed = 1.0;
    let mut xv = speed;
    let mut xa = 0.0;
    let eps = 1e-9;
    let eps = 1e-5;

    let mut status = Status::Idle;


    let mut boob_x = 400.0;
    let mut boob_r = 5.0;

    let mut last_time = get_time();
    let desired_frame_time: f64 = 1.0 / 60.0;  // 60 fps

    loop {
        clear_background(WHITE);

        draw_text(format!("Score: {}", score).leak(), 0.0, 32.0, 32.0, BLACK);

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
        boob_r += 0.3;

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
                            KeyCode::Space=> {
                                if xv == speed {
                                    xv += 5.0;
                                }
                                if xv == -speed {
                                    xv -= 5.0;
                                }
                            },
                            _ => {}
                        }
                        println!("{:?}", key);
                    }
                    _ => {}
                }
            }
        }

        if status != Status::Die {
            if is_close!(xv, speed, rel_tol=eps) {
                status = Status::Idle;
                xv = speed;
            }
            else if is_close!(xv, -speed, rel_tol=eps) {
                status = Status::Idle;
                xv = -speed;
            }
            else {
                if xv > speed {
                    xv = f32::max(speed, xv - 0.3);
                }
                else if xv < -speed {
                    xv = f32::min(-speed, xv + 0.3);
                }
                status = Status::Dash;
            }
        }

        x += xv;
        x = f32::max(f32::min(x, 512.0), 0.0);

        if x == 512.0 {
            xv = -speed;
        }
        if x == 0.0 {
            xv = speed;
        }
        // println!("{x}, {xv}");

        score += 1;

        let time_passed = get_time() - last_time;
        // println!("{time_passed}");
        // if time_passed > desired_frame_time {
        //     println!("{time_passed}");
        //     last_time = get_time();
        //
        //     let time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
        //     println!("Sleep for {}ms", time_to_sleep);
        //     std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        //
        //     next_frame().await;
        // }

        last_time = get_time();
        next_frame().await;
    }
}