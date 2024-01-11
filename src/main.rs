use std::time::SystemTime;
use is_close::is_close;
use macroquad::prelude::*;
use macroquad::rand::{srand, gen_range};
// use rand::Rng;


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
    // let mut rng = rand::thread_rng();
    let d = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Duration since UNIX_EPOCH failed");
    srand(d.as_secs());

    let mut x: f32 = gen_range(200.0, 512.0 - 200.0);
    let mut y = 128.0 - 12.0;

    let mut score: u32 = 0;
    let mut speed = 3.0;
    let mut xv = speed;
    let eps = 1e-5;

    let mut status = Status::Idle;


    let mut bomb_x =  gen_range(400.0, 500.0);
    let mut bomb_r = 5.0;
    let mut bomb_speed = 1.5;

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
        draw_circle_lines(bomb_x, 128.0, bomb_r, 3.0, RED);

        if status == Status::Die {
            draw_text("You Die", 110.0, 152.0, 92.0, RED);
        }

        if status == Status::Idle {
            if (x < bomb_x + bomb_r + 3.0 && x > bomb_x + bomb_r - 3.0) ||
               (x < bomb_x - bomb_r + 3.0 && x > bomb_x - bomb_r - 3.0) {
                status = Status::Die;
                println!("Die");
            }
            else {
                match get_last_key_pressed() {
                    Some(key) => {
                        match key {
                            KeyCode::Space=> {
                                if xv == speed {
                                    xv += 15.0;
                                }
                                if xv == -speed {
                                    xv -= 15.0;
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
                    xv = f32::max(speed, xv - 3.0);
                }
                else if xv < -speed {
                    xv = f32::min(-speed, xv + 3.0);
                }
                status = Status::Dash;
            }
            score += 1;

            x += xv;
            x = f32::max(f32::min(x, 512.0), 0.0);

            if x == 512.0 {
                xv = -speed;
            }
            if x == 0.0 {
                xv = speed;
            }
        }
        // println!("{x}, {xv}");


        // Update Bomb
        if bomb_r >= 100.0 {
            bomb_x =  gen_range(2.0, 512.0 - 20.0);
            while f32::abs(bomb_x - x) < 100.0 {
                bomb_x =  gen_range(20.0, 512.0 - 20.0);
            }
            bomb_r = 5.0;
            bomb_speed += 0.1;
            speed += 0.3;
            if xv > 0.0 {
                xv = speed;
            }
            else {
                xv = -speed;
            }
        }
        else {
            bomb_r += bomb_speed;
        }


        let time_passed = get_time() - last_time;
        if time_passed < desired_frame_time {
            // println!("{time_passed}");

            let time_to_sleep = (desired_frame_time - time_passed) * 1000.;
            // println!("Sleep for {}ms", time_to_sleep);
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));

            next_frame().await;
        }
        last_time = get_time();
    }
}