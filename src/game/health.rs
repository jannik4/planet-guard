use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Component)]
pub struct Health {
    max: f32,
    current: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { max, current: max }
    }

    pub fn max(&self) -> f32 {
        self.max
    }

    pub fn current(&self) -> f32 {
        self.current
    }

    pub fn fraction(&self) -> f32 {
        self.current / self.max
    }

    pub fn damage(&mut self, damage: f32) {
        self.current = f32::max(0.0, self.current - damage);
    }

    pub fn set_dead(&mut self) {
        self.current = 0.0;
    }
}
