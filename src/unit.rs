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
    Up,
    Down,
    Left,
    Right,
}

#[derive(Deref, DerefMut, Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Euler(pub f32);

impl Euler {
    pub fn from_radians(radians: f32) -> Self {
        Self(radians.to_degrees().rem_euclid(360.))
    }
}

impl From<Direction> for Euler {
    fn from(direction: Direction) -> Self {
        Euler(match direction {
            Direction::Up => 0.0,
            Direction::Right => 90.0,
            Direction::Down => 180.0,
            Direction::Left => 270.0,
        })
    }
}

impl From<Euler> for Direction {
    fn from(angle: Euler) -> Self {
        match angle.0 {
            angle if (315.0..=360.0).contains(&angle) || (0.0..=45.0).contains(&angle) => Direction::Up,
            angle if (45.0..=135.0).contains(&angle) => Direction::Right,
            angle if (135.0..=225.0).contains(&angle) => Direction::Down,
            angle if (225.0..=315.0).contains(&angle) => Direction::Left,
            _ => Direction::Up,
        }
    }
}
