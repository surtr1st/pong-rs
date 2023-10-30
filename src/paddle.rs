use sdl2::rect::Rect;

use crate::constants::{CELL_SIZE, SCREEN_SIZE};

pub struct Paddle {
    x: i32,
    y: i32,
}

impl Paddle {
    pub fn new(x: i32, y: i32) -> Self {
        Paddle { x, y }
    }

    pub fn pad(&self) -> Rect {
        let size = CELL_SIZE as u32;

        let (_, height) = SCREEN_SIZE;

        let y = self.y + (height as i32 / 2) - (CELL_SIZE * 5);

        Rect::new(self.x, y, size, size * 10)
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    pub fn move_up(&mut self) {
        self.y -= CELL_SIZE * 2;
    }

    pub fn move_down(&mut self) {
        self.y += CELL_SIZE * 2;
    }
}
