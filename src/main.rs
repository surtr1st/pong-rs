pub mod ball;
pub mod constants;
pub mod paddle;
use ball::Ball;
use constants::{CELL_SIZE, SCREEN_SIZE, TITLE};
use paddle::Paddle;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let (width, height) = SCREEN_SIZE;
    let window = video_subsystem
        .window(TITLE, width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build()?;
    let mut player1 = Paddle::new(25, 0);
    let mut player2 = Paddle::new(width as i32 - (CELL_SIZE * 2), 0);
    let mut ball = Ball::new(30, 30, 5, 5);

    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        canvas.clear();

        ball.move_around();

        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Q => break 'running,
                    Keycode::W => player1.move_up(),
                    Keycode::S => player1.move_down(),
                    Keycode::Up => player2.move_up(),
                    Keycode::Down => player2.move_down(),
                    _ => {}
                },
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::W => player1.move_up(),
                    Keycode::S => player1.move_down(),
                    Keycode::Up => player2.move_up(),
                    Keycode::Down => player2.move_down(),
                    _ => {}
                },
                _ => {}
            }
        }

        resolve_overflow(&mut player1);
        resolve_overflow(&mut player2);

        bounce(&mut player1, &mut ball);
        bounce(&mut player2, &mut ball);

        canvas.set_draw_color(Color::RED);
        canvas.fill_rect(ball.ball())?;

        canvas.set_draw_color(Color::WHITE);
        canvas.fill_rects(&player1.pad())?;

        canvas.set_draw_color(Color::WHITE);
        canvas.fill_rects(&player2.pad())?;

        canvas.set_draw_color(Color::BLACK);
        canvas.present();
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn resolve_overflow(target: &mut Paddle) {
    let (_, y) = target.position();
    let (_, fy) = target.first_position();
    let (_, ly) = target.last_position();
    let (_, height) = SCREEN_SIZE;

    let y0 = y + (height as i32 / 2) - (CELL_SIZE * 2);

    if fy < 0 {
        target.set_y(y0 * 10);
    }

    if ly > (height as i32) - CELL_SIZE {
        target.set_y((y0 / 2) - 12);
    }
}

fn bounce(paddle: &mut Paddle, ball: &mut Ball) {
    for i in 0..paddle.pad().len() {
        let part = paddle.pad()[i];
        if detect_collision(&ball.ball(), &part) {
            ball.set_vx(-ball.vx());
            ball.x += ball.vx();
        }
    }
}

fn detect_collision(target: &Rect, opponent: &Rect) -> bool {
    if target.has_intersection(*opponent) {
        true
    } else {
        false
    }
}
