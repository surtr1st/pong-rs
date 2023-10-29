use sdl2::rect::Rect;

use crate::constants::{CELL_SIZE, SCREEN_SIZE};

pub struct Ball {
    pub x: i32,
    pub y: i32,
    vx: i32,
    vy: i32,
}

impl Ball {
    pub fn new(x: i32, y: i32, vx: i32, vy: i32) -> Self {
        Ball { x, y, vx, vy }
    }

    pub fn ball(&self) -> Rect {
        let size = CELL_SIZE as u32;
        Rect::new(self.x, self.y, size, size)
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

    pub fn vx(&self) -> i32 {
        self.vx
    }

    pub fn set_vx(&mut self, vx: i32) {
        self.vx = vx;
    }

    pub fn set_vy(&mut self, vy: i32) {
        self.vy = vy;
    }

    pub fn move_around(&mut self) {
        let (_, height) = SCREEN_SIZE;
        let h = height as i32;

        self.x = self.x + self.vx;
        self.y = self.y + self.vy;

        if self.y < 0 || self.y > h - CELL_SIZE {
            self.vy = -self.vy;
            self.y += self.vy;
        }
    }

    pub fn is_outside(&self) -> bool {
        let (width, _) = SCREEN_SIZE;
        if self.x < 0 || self.x > width as i32 - CELL_SIZE {
            return true;
        }
        false
    }
}
