use std::collections::VecDeque;

use bevy::{prelude::Component, time::{Timer, TimerMode}};
use uuid::Uuid;

// region:    --- Common Component
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

// endregion: --- Common Component

// region:    --- Snake Component

#[derive(Component)]
pub struct SnakeHead {
    pub moved: bool,
    pub ate: bool,
    pub body_parts: VecDeque<SnakeBody>
}

#[derive(Component, Clone)]
pub struct SnakeBody {
    pub is_tail: bool,
    pub id: Uuid
}

#[derive(Component, Debug)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

// endregion: --- Snake Component

// region:    --- Food Component

pub enum FoodType {
    Simple,
    Gold,
    Bonus
}

#[derive(Component)]
pub struct Food(pub FoodType);

#[derive(Component)]
pub struct FoodTimer(pub Timer);

impl Default for FoodTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(8., TimerMode::Once))
    }
}
// endregion: --- Food Component

// region:    --- Wall Component

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Collision;

// endregion: --- Wall Component