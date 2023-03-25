use std::collections::VecDeque;

use bevy::{prelude::{Component, Color}, time::{Timer, TimerMode}};
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

#[derive(Component)]
pub struct BonusTimer {
    pub color_timer: Timer,
    pub colors: Vec<Color>,
    pub index_color: usize,
    pub life_timer: Option<Timer>,
    pub life_cycle: usize
}

impl Default for BonusTimer {
    fn default() -> Self {
        Self {
            color_timer: Timer::from_seconds(0.05, TimerMode::Repeating),
            colors: vec![
                Color::rgb(1., 0., 0.), // Red
                Color::rgb(1., 0.5, 0.), // Orange
                Color::rgb(1., 1., 0.), // Yellow
                Color::rgb(0.5, 1., 0.), // Chartreuse
                Color::rgb(0., 1., 0.), // Green
                Color::rgb(0., 1., 0.5), // Spring Green
                Color::rgb(0., 1., 1.), // Cyan
                Color::rgb(0., 0.5, 1.), // Dodger Blue
                Color::rgb(0., 0., 1.), // Blue
                Color::rgb(0.5, 0., 1.), // Purple
                Color::rgb(1., 0., 1.), // Violet
                Color::rgb(1., 0., 0.5), // Magenta
            ],
            index_color: 0,
            life_timer: None,
            life_cycle: 0
        }
    }
}
// endregion: --- Common Component

// region:    --- Snake Component

#[derive(Component)]
pub struct SnakeHead {
    pub moved: bool,
    pub ate: bool,
    pub invincible: bool,
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