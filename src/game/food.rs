use std::{time::Duration, collections::HashSet};

use bevy::{prelude::{Plugin, App, SystemSet, Commands, Query, Transform, Res, ResMut, Vec3, With, Entity, State, IntoSystemDescriptor, CoreStage, Or}, time::{FixedTimestep, Time}, sprite::{SpriteBundle, Sprite}, ecs::schedule::ShouldRun};
use iyes_loopless::prelude::{IntoConditionalSystem, ConditionHelpers, AppLooplessFixedTimestepExt, ConditionSet};
use rand::Rng;
use super::{AppState, setup_system, components::{SnakeHead, SnakeBody}, PositionsAvailable};

use super::{components::{Position, Size, Food, FoodTimer}, ArenaSize, FoodCount, FOOD_MAX, ARENA_WIDTH, ARENA_HEIGHT, FOOD_COLOR, FOOD_SIZE, UPPER_EDGE};

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_fixed_timestep(
                Duration::from_millis(5000),
                // give it a label
                "food_spawn_time",
            )
            .add_fixed_timestep_system(
                "food_spawn_time",
                0,
                food_spawn_system
                .run_in_bevy_state(AppState::InGame)
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                ConditionSet::new()
                // .run_not_in_bevy_state(AppState::MainMenu)
                // .run_not_in_bevy_state(AppState::Pause)
                .run_in_bevy_state(AppState::InGame)
                .with_system(food_timer_system).into()
            )
            .add_system_set(
                SystemSet::on_exit(AppState::InGame)
                .with_system(cleanup_food_system)
            );
    }
}

fn check_time_passed(time: Res<Time>, app_state: Res<State<AppState>>) -> ShouldRun {
    match app_state.current() {
        AppState::InGame => {
            if time.elapsed_seconds() as u32 % 5 == 0 {
                return ShouldRun::Yes;
            }
        },
        _ => {}
    }
    
    ShouldRun::No
}

fn food_spawn_system(
    mut commands: Commands,
    arena_size: Res<ArenaSize>,
    mut food_count: ResMut<FoodCount>,
    positions_available: Res<PositionsAvailable>,
    query: Query<(&Position), Or<(With<SnakeHead>, With<SnakeBody>, With<Food>)>>,
) {
    if food_count.0 >= FOOD_MAX {
        return;
    }

    // let mut occupied_positions: HashSet<i32> = get_occupied_positions(query);
    // if occupied_positions.len() as u32 == ARENA_WIDTH * ARENA_HEIGHT {
    //     // Scénario qui n'est pas censé arrivé, la fin de partie doit être prononcée avant
    //     return;
    // }

    // let (x, y) = get_new_food_position(occupied_positions);

    let mut positions_available_depending_snake_and_food: Vec<Position> = get_available_positions_depending_snake_and_food(positions_available.0.clone(), query);
    

    let new_position = get_new_food_position(positions_available_depending_snake_and_food);

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: FOOD_COLOR,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(
                convert(new_position.x as f32, arena_size.px_width, arena_size.tile_width as f32),
                convert(new_position.y as f32, arena_size.px_height, arena_size.tile_height as f32),
                0.0,
            ),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Food) 
    .insert(new_position)
    .insert(Size::square(FOOD_SIZE))
    .insert(FoodTimer::default());

    food_count.0 += 1; 
}

// fn get_occupied_positions(query: Query<(&Position)>) -> HashSet<i32> {
//     let mut occupied_positions: HashSet<i32>= HashSet::new();

//     for position in query.iter() {
//         let x = position.x;
//         let y = position.y;

//         let mut occupied_position = get_couple_x_y(x, y);        
//         occupied_positions.insert(occupied_position);
//     }

//     occupied_positions
// }

fn get_available_positions_depending_snake_and_food(mut positions_available: HashSet<Position>, query: Query<(&Position), Or<(With<SnakeHead>, With<SnakeBody>, With<Food>)>>) -> Vec<Position> {
    for position in query.iter() {
        positions_available.remove(position);
    }

    positions_available.into_iter().collect()
}

fn get_couple_x_y(x: i32, y: i32) -> i32 {
    let mut x_y = x * y;
    if x == 0 || y == 0 {
        x_y = x + y;
    } 
    if x < y {
        x_y *= -1;
    }
    x_y
}

// fn get_new_food_position(occupied_positions: HashSet<i32>) -> (i32, i32) {
//     let mut rng = rand::thread_rng();

//     let mut x;
//     let mut y;

//     loop {
//         x = rng.gen_range(0..ARENA_WIDTH as i32);
//         y = rng.gen_range(0..ARENA_HEIGHT as i32);

//         let x_y = get_couple_x_y(x, y);
//         if !occupied_positions.contains(&x_y) {
//             break;
//         }
//     }

//     (x, y)
// }

fn get_new_food_position(positions_available_depending_snake_and_food: Vec<Position>) -> Position {
    let mut rng = rand::thread_rng();

    let index = rng.gen_range(0..positions_available_depending_snake_and_food.len());
    *positions_available_depending_snake_and_food.get(index).expect("index non trouvé dans la liste")
}

fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
	let tile_size = bound_window / bound_game;
	pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
}

fn food_timer_system(
    mut commands: Commands,
    time: Res<Time>,
    mut food_count: ResMut<FoodCount>,
    mut query: Query<(Entity, &mut FoodTimer), With<Food>>,
) {
    if food_count.0 < 1 {
        return;
    }

    for (entity, mut timer) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            commands.entity(entity).despawn();
            food_count.0 -= 1;
        }
    }
}

fn cleanup_food_system(mut commands: Commands, mut query: Query<Entity, With<Food>>) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn();
        // println!("food");
    }
}