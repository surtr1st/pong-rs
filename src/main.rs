pub mod ball;
pub mod constants;
pub mod paddle;
use ball::Ball;
use constants::{CELL_SIZE, SCREEN_SIZE, TITLE};
use paddle::Paddle;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::{FontStyle, Sdl2TtfContext};
use sdl2::video::{Window, WindowContext};
use std::path::Path;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let ttf_context = sdl2::ttf::init()?;
    let video_subsystem = sdl_context.video()?;

    let (width, height) = SCREEN_SIZE;
    let window = video_subsystem
        .window(TITLE, width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build()?;
    let mut texture_creator = canvas.texture_creator();
    let mut player1 = Paddle::new(25, 0);
    let mut player2 = Paddle::new(width as i32 - (CELL_SIZE * 2), 0);
    let mut ball = Ball::new(30, 30, 5, 5);
    let mut scores = (0, 0);
    let mut rng = rand::thread_rng();

    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        canvas.clear();

        draw_text(
            &mut canvas,
            &mut texture_creator,
            &ttf_context,
            format!("{} | {}", scores.0, scores.1),
        )?;

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

        if ball.is_outside_left() {
            scores.1 += 1;
            ball = Ball::new(
                width as i32 - CELL_SIZE,
                rng.gen_range(0..24) * CELL_SIZE,
                -5,
                5,
            );
        }
        if ball.is_outside_right() {
            scores.0 += 1;
            ball = Ball::new(0, rng.gen_range(0..24) * CELL_SIZE, 5, 5);
        }

        canvas.set_draw_color(Color::RED);
        canvas.fill_rect(ball.ball())?;

        canvas.set_draw_color(Color::WHITE);
        canvas.fill_rect(player1.pad())?;

        canvas.set_draw_color(Color::WHITE);
        canvas.fill_rect(player2.pad())?;

        canvas.set_draw_color(Color::BLACK);
        canvas.present();
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn resolve_overflow(target: &mut Paddle) {
    let (_, y) = target.position();
    let (_, height) = SCREEN_SIZE;

    let y0 = (height as i32 / 2) - (CELL_SIZE * 5);

    if y <= -y0 {
        target.set_y(-y0);
    }

    if y > y0 {
        target.set_y(y0);
    }
}

fn bounce(paddle: &mut Paddle, ball: &mut Ball) {
    if detect_collision(&ball.ball(), &paddle.pad()) {
        ball.set_vx(-ball.vx());
        ball.x += ball.vx();
    }
}

fn detect_collision(target: &Rect, opponent: &Rect) -> bool {
    if target.has_intersection(*opponent) {
        true
    } else {
        false
    }
}

fn draw_text(
    canvas: &mut Canvas<Window>,
    texture_creator: &mut TextureCreator<WindowContext>,
    ttf_context: &Sdl2TtfContext,
    content: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let font_path = Path::new("fonts/Inter-V.ttf");
    let mut font = ttf_context.load_font(font_path, 128)?;
    font.set_style(FontStyle::BOLD);

    let surface = font.render(&content).blended(Color::WHITE)?;
    let texture = texture_creator.create_texture_from_surface(surface)?;
    let target = Rect::new((SCREEN_SIZE.0 / 2) as i32 - CELL_SIZE - 10, 0, 90, 50);

    canvas.copy(&texture, None, Some(target))?;

    Ok(())
}
