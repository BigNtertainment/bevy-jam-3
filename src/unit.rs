use bevy::prelude::*;

#[derive(Reflect, Component, Copy, Clone, Debug, PartialEq)]
#[reflect(Component)]
pub struct Health {
    health: f32,
    max_health: f32,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            health: 100.0,
            max_health: 100.0,
        }
    }
}

#[derive(Deref, DerefMut, Copy, Clone, Debug, PartialEq, Eq)]
pub struct HealthReachedZero(bool);

#[allow(unused)]
impl Health {
    pub fn new(max_health: f32) -> Self {
        Self {
            health: max_health,
            max_health,
        }
    }

    /// # Returns
    /// True if the health reached zero.
    #[warn(unused_must_use)]
    #[must_use]
    pub fn take_damage(&mut self, amount: f32) -> HealthReachedZero {
        self.health -= amount;

        HealthReachedZero(self.health <= 0.0)
    }

    pub fn heal(&mut self, amount: f32) {
        self.health = (self.health + amount).clamp(1.0, self.max_health);
    }

    pub fn get_health(&self) -> f32 {
        self.health
    }

    pub fn get_max_health(&self) -> f32 {
        self.max_health
    }

    pub fn set_health(&mut self, hp: f32) {
        self.health = hp;
    }
}

#[derive(Default, Reflect, Component, Copy, Clone, Debug, PartialEq)]
#[reflect(Component)]
pub struct Movement {
    pub speed: f32,
}

#[derive(Default, Reflect, Component, Copy, Clone, Debug, PartialEq, Eq)]
#[reflect(Component)]
pub enum Direction {
    #[default]
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
