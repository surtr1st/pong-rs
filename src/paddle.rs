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

    pub fn first_position(&self) -> (i32, i32) {
        let first = self.pad()[0];
        (first.x(), first.y())
    }

    pub fn last_position(&self) -> (i32, i32) {
        let last_index = self.pad().len() - 1;
        let last = self.pad()[last_index];
        (last.x(), last.y())
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
        self.y -= CELL_SIZE;
    }

    pub fn move_down(&mut self) {
        self.y += CELL_SIZE;
    }
}
