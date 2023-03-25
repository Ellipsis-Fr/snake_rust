
use std::collections::VecDeque;

use bevy::{prelude::{Plugin, App, StartupStage, Commands, Color, Vec2, Res, Input, KeyCode, Query, With, Mut, Transform, Vec3, SystemSet, Entity}, sprite::{SpriteBundle, Sprite}};
use uuid::Uuid;
use super::AppState;

use super::{components::{SnakeHead, Velocity, Direction, Position, Size, SnakeBody}, SNAKE_HEAD_COLOR, SNAKE_BODY_COLOR, SNAKE_BODY_SIZE, SNAKE_HEAD_SIZE};

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
            SystemSet::on_enter(AppState::InGame)
            .with_system(snake_spawn_system)
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
            .with_system(keyboard_event_system)
        )
        .add_system_set(
            SystemSet::on_exit(AppState::InGame)
            .with_system(cleanup_snake_system)
        );
    }
}

fn snake_spawn_system(mut commands: Commands, /*win_size: Res<WinSize>*/) {
    let (init_x, init_y) = (3, 3);
    let init_direction = Direction::RIGHT;

    // add snake_body
    // first snake_body
    let snake_body = SnakeBody {
        is_tail: true,
        id: Uuid::new_v4()
    };

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: SNAKE_BODY_COLOR,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new((init_x - 1) as f32, init_y as f32, 10.),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(snake_body.clone()) 
    .insert(Position {x: init_x - 1, y: init_y})
    .insert(Size::square(SNAKE_BODY_SIZE));

    // add snake_head
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: SNAKE_HEAD_COLOR,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(init_x as f32, init_y as f32, 10.),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(SnakeHead { 
        moved: true, 
        ate: false,
        invincible: false,
        body_parts: VecDeque::from([snake_body])
    })
    .insert(init_direction)
    .insert(Position {x: init_x, y: init_y})
    .insert(Size::square(SNAKE_HEAD_SIZE));

    // // add snake_body
    // commands.spawn(SpriteBundle {
    //     sprite: Sprite {
    //         color: SNAKE_BODY_COLOR,
    //         ..Default::default()
    //     },
    //     transform: Transform {
    //         translation: Vec3::new(2., 3., 10.),
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // })
    // .insert(SnakeBody {is_tail: true }) 
    // .insert(Position {x: 2, y: 3})
    // .insert(Size::square(0.6));
}

fn keyboard_event_system(kb: Res<Input<KeyCode>>, mut query: Query<(&mut Direction, &mut SnakeHead), With<SnakeHead>>) {
    if kb.any_just_pressed(vec![KeyCode::Up, KeyCode::Down, KeyCode::Left, KeyCode::Right]) {
        if let Ok((mut direction, mut snake_head)) = query.get_single_mut() {
            if !snake_head.moved {
                return;
            }  
            edit_snake_direction(kb, direction);
            snake_head.moved = false;
        }
    }
}

fn edit_snake_direction(kb: Res<Input<KeyCode>>, mut direction: Mut<Direction>) {
    match *direction {
        Direction::UP | Direction::DOWN => {
            if kb.just_pressed(KeyCode::Left) {
                *direction = Direction::LEFT;
            } else if kb.just_pressed(KeyCode::Right) {
                *direction = Direction::RIGHT;
            }
        },
        Direction::LEFT | Direction::RIGHT => {
            if kb.just_pressed(KeyCode::Up) {
                *direction = Direction::UP;
            } else if kb.just_pressed(KeyCode::Down) {
                *direction = Direction::DOWN;
            }
        }
    }
}

fn cleanup_snake_system(
    mut commands: Commands,
    mut snake_head_query: Query<Entity, With<SnakeHead>>,
	mut snake_body_query: Query<Entity, With<SnakeBody>>,
) {
    if let Ok(entity) = snake_head_query.get_single_mut() {
        commands.entity(entity).despawn();
        // println!("snake_head_query");
    };

    for entity in snake_body_query.iter_mut() {
        commands.entity(entity).despawn();
        // println!("snake_body_query");
    }
}