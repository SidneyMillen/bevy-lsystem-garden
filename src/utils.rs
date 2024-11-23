use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Counter(pub usize);

impl Counter {
    pub fn reset(mut self) {
        self.0 = 0;
    }
    pub fn incr(mut self) {
        self.0 += 1;
    }
    pub fn decr(mut self) {
        self.0 -= 1;
    }
}

#[derive(Component, Debug)]
pub struct Level(pub usize);

impl Level {
    pub fn reset(mut self) {
        self.0 = 0;
    }
    pub fn incr(mut self) {
        self.0 += 1;
    }
    pub fn decr(mut self) {
        self.0 -= 1;
    }
}
