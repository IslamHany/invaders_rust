use std::time::Duration;

use rusty_time::Timer;

use crate::frame::Drawable;

pub struct Shot{
    pub x: usize,
    pub y: usize,
    pub explooding: bool,
    timer: Timer
}

impl Shot {
    pub fn new (x: usize, y: usize) -> Self{
        return Shot{
            x: x,
            y: y,
            explooding: false,
            timer: Timer::from_millis(50)
        };
    }

    pub fn update(&mut self, delta: Duration){
        self.timer.update(delta);

        if self.timer.ready && !self.explooding {
            if self.y > 0 {
                self.y = self.y - 1;
            }
            self.timer.reset();
        }
    }

    pub fn explode(&mut self){
        self.explooding = true;
        self.timer = Timer::from_millis(250);
    }

    pub fn dead(&self) -> bool{
        return (self.explooding && self.timer.ready) || (self.y == 0);
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        frame[self.x][self.y] = if self.explooding {"*"} else {"|"};
    }
}