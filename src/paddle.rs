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

    pub fn pad(&self) -> Vec<Rect> {
        let size = CELL_SIZE as u32;

        let (_, height) = SCREEN_SIZE;

        let y1 = self.y + (height as i32 / 2) - (CELL_SIZE * 2);
        let y2 = self.y + (height as i32 / 2) - CELL_SIZE;
        let y3 = self.y + (height as i32 / 2);
        let y4 = self.y + CELL_SIZE + (height as i32 / 2);

        vec![
            Rect::new(self.x, y1, size, size),
            Rect::new(self.x, y2, size, size),
            Rect::new(self.x, y3, size, size),
            Rect::new(self.x, y4, size, size),
        ]
    }
}
