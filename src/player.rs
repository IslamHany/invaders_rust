use std::time::Duration;

use crate::{NUM_COLS, NUM_ROWS, frame::{Drawable, Frame}, shot::Shot, SHOTS_CNT, SHOT_OFFSET};

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>
} 

impl Player {
    pub fn new() -> Self {
        return Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
            shots: Vec::new()
        };
    }

    pub fn move_left(&mut self) {
        if self.x > 0{
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1{
            self.x += 1;
        }
    }

    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < SHOTS_CNT{
            self.shots.push(Shot::new(self.x, self.y - SHOT_OFFSET));
            return true;
        }

        return false;
    }

    pub fn update(&mut self, delta: Duration){
        for shot in self.shots.iter_mut(){
            shot.update(delta);
        }

        self.shots.retain(|shot| !shot.dead());
    }
}

impl Drawable for Player{
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A";
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}