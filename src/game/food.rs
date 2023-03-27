use std::{time::Duration, collections::HashSet};

use bevy::{prelude::{Plugin, App, SystemSet, Commands, Query, Transform, Res, ResMut, Vec3, With, Entity, State, IntoSystemDescriptor, CoreStage, Or, Vec2, Color, Mesh, Assets, shape, Handle, Without}, time::{FixedTimestep, Time, Timer, TimerMode}, sprite::{SpriteBundle, Sprite, MaterialMesh2dBundle, ColorMaterial, Material2d}, ecs::schedule::ShouldRun};
use iyes_loopless::prelude::{IntoConditionalSystem, ConditionHelpers, AppLooplessFixedTimestepExt, ConditionSet};
use rand::Rng;
use crate::{main_menu::sub_menu::GameType, game::components::BonusTimer};

use super::{AppState, setup_system, components::{SnakeHead, SnakeBody, FoodType}, PositionsAvailable, GOLD_FOOD_COLOR, snake, GameTextures};

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
            .add_fixed_timestep(
                Duration::from_millis(30000),
                // give it a label
                "gold_food_spawn_time",
            )
            .add_fixed_timestep(
                Duration::from_millis(40000),
                // give it a label
                "bonus_food_spawn_time",
            )
            .add_fixed_timestep_system(
                "food_spawn_time",
                0,
                food_spawn_system
                .run_in_bevy_state(AppState::InGame)
            )
            .add_fixed_timestep_system(
                "gold_food_spawn_time",
                0,
                gold_food_spawn_system
                .run_in_bevy_state(AppState::InGame)
            )
            .add_fixed_timestep_system(
                "bonus_food_spawn_time",
                0,
                bonus_food_spawn_system
                .run_in_bevy_state(AppState::InGame).run_if(is_game_with_wall).run_if(snake_is_not_too_big)
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                ConditionSet::new()
                // .run_not_in_bevy_state(AppState::MainMenu)
                // .run_not_in_bevy_state(AppState::Pause)
                .run_in_bevy_state(AppState::InGame)
                .with_system(food_timer_system)
                .with_system(bonus_color_timer_system).into()
            )
            .add_system_set(
                SystemSet::on_exit(AppState::InGame)
                .with_system(cleanup_food_system)
            );
    }
}

// fn check_time_passed(time: Res<Time>, app_state: Res<State<AppState>>) -> ShouldRun {
//     match app_state.current() {
//         AppState::InGame => {
//             if time.elapsed_seconds() as u32 % 5 == 0 {
//                 return ShouldRun::Yes;
//             }
//         },
//         _ => {}
//     }
    
//     ShouldRun::No
// }

fn is_lucky(luck: f64) -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_bool(1. / luck)
}

fn is_game_with_wall(game_type: Res<GameType>) -> bool {
    if game_type.wall_type == 0 as usize {
        return false;
    }    
    true
}

fn snake_is_not_too_big(query: Query<(&SnakeHead), With<SnakeHead>>, game_type: Res<GameType>) -> bool {
    if let Ok(snake) = query.get_single() {
        let snake_size = 1 + snake.body_parts.len() as u32;
        let maximun_snake_size;
        match game_type.wall_type {
            4 => maximun_snake_size = (ARENA_WIDTH - 1) * (ARENA_HEIGHT - 1),
            2 | 3 => maximun_snake_size = (ARENA_WIDTH - 1) * ARENA_HEIGHT,
            _ => maximun_snake_size = ARENA_WIDTH * ARENA_HEIGHT,
        }
        
        // let mut x = 1;
        // x = (maximun_snake_size * 75) / 100 - 1;
        // let snake_size = x + snake.body_parts.len() as u32;
        // println!("");
        // println!("");
        // println!("===================================================");
        // println!("===================================================");
        // println!("===================================================");
        // println!("taille du serpent {}", snake_size);
        // println!("taille maximal du serpent {}", maximun_snake_size);
        // println!("rario taille du serpent / taille max {}", snake_size as f32 / maximun_snake_size as f32);
        
        if snake_size as f32 / maximun_snake_size as f32 > 0.75 {
            return false;
        }
    }  
    true
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
    
    if positions_available_depending_snake_and_food.is_empty() {
        return;
    }

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
    .insert(Food(FoodType::Simple)) 
    .insert(new_position)
    .insert(Size::square(FOOD_SIZE))
    .insert(FoodTimer::default());

    food_count.0 += 1; 
}

fn gold_food_spawn_system(
    mut commands: Commands,
    arena_size: Res<ArenaSize>,
    positions_available: Res<PositionsAvailable>,
    query: Query<(&Position), Or<(With<SnakeHead>, With<SnakeBody>, With<Food>)>>,
) {
    if !is_lucky(3.) {
        return;
    }

    let mut positions_available_depending_snake_and_food: Vec<Position> = get_available_positions_depending_snake_and_food(positions_available.0.clone(), query);
    
    if positions_available_depending_snake_and_food.is_empty() {
        return;
    }

    let new_position = get_new_food_position(positions_available_depending_snake_and_food);

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: GOLD_FOOD_COLOR,
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
    .insert(Food(FoodType::Gold)) 
    .insert(new_position)
    .insert(Size::square(FOOD_SIZE))
    .insert(FoodTimer(Timer::from_seconds(6., TimerMode::Once)));
}

fn bonus_food_spawn_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    arena_size: Res<ArenaSize>,
    positions_available: Res<PositionsAvailable>,
    // game_texture: Res<GameTextures>, Pas utilisé
    query: Query<(&Position), Or<(With<SnakeHead>, With<SnakeBody>, With<Food>)>>,
) {
    if !is_lucky(20.) {
        return;
    }

    let mut positions_available_depending_snake_and_food: Vec<Position> = get_available_positions_depending_snake_and_food(positions_available.0.clone(), query);
    
    if positions_available_depending_snake_and_food.is_empty() {
        return;
    }
    let new_position = get_new_food_position(positions_available_depending_snake_and_food);
    // systeme de changement de couleur à utiliser https://github.com/bevyengine/bevy/discussions/2869

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(10.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::RED)),
        transform: Transform::from_translation(Vec3::new(
            convert(new_position.x as f32, arena_size.px_width, arena_size.tile_width as f32),
            convert(new_position.y as f32, arena_size.px_height, arena_size.tile_height as f32),
            0.0,
        )),
        ..Default::default()
    })
    .insert(Food(FoodType::Bonus)) 
    .insert(new_position)
    // .insert(Size::square(FOOD_SIZE))
    .insert(FoodTimer(Timer::from_seconds(6., TimerMode::Once)))
    .insert(BonusTimer::default());

    /* Spawn a sprite with img
    Je conserve pour potentiel réutilisation dans d'autres codes 
    commands.spawn(SpriteBundle {
        sprite: Sprite { 
            // custom_size: Some(Vec2::new(5.0, 5.0) * 7.),
            color: Color::rgb(0.3, 0.3, 0.3),
            // color: Color::hsla(180., 0.3, 0.2, 0.92),
            ..Default::default()
         },
        texture: game_texture.bonus_star.clone(),
        transform: Transform {
            translation: Vec3::new(
                convert(new_position.x as f32, arena_size.width, ARENA_WIDTH as f32),
                convert(new_position.y as f32, arena_size.height, ARENA_HEIGHT as f32),
                0.0,
            ),
            scale: Vec3::new(0.5, 0.5, 1.),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Food(FoodType::Bonus)) 
    .insert(new_position)
    // .insert(Size::square(FOOD_SIZE))
    .insert(FoodTimer(Timer::from_seconds(6., TimerMode::Once)))
    .insert(BonusTimer::default()); */
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
    mut query: Query<(Entity, &mut FoodTimer, &Food), With<Food>>,
) {
    for (entity, mut timer, food) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            commands.entity(entity).despawn();

            match food.0 {
                FoodType::Simple => food_count.0 -= 1,
                _ => ()
            }
        }
    }
}

fn bonus_color_timer_system(
    time: Res<Time>,
    mut snake_head_bonus_timer_query: Query<(&mut Sprite, &mut BonusTimer), (With<BonusTimer>, With<SnakeHead>)>,
    mut color_materials_bonus_timer_query: Query<(&Handle<ColorMaterial>, &mut BonusTimer), (With<BonusTimer>, With<Food>, Without<SnakeHead>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (mut sprite, mut bonus_timer) in snake_head_bonus_timer_query.iter_mut() {
        bonus_timer.color_timer.tick(time.delta());
        if bonus_timer.color_timer.finished() {
            bonus_timer.index_color += 1 as usize;
            if bonus_timer.index_color >= bonus_timer.colors.len() { bonus_timer.index_color = 0; }
            sprite.color = bonus_timer.colors.get(bonus_timer.index_color).unwrap().clone();
        }
    }

    if let Ok((color_material, mut bonus_timer)) = color_materials_bonus_timer_query.get_single_mut() {
        bonus_timer.color_timer.tick(time.delta());
        if bonus_timer.color_timer.finished() {
            bonus_timer.index_color += 1 as usize;
            if bonus_timer.index_color >= bonus_timer.colors.len() { bonus_timer.index_color = 0; }
            let mut material = materials.get_mut(color_material).unwrap();
            material.color = bonus_timer.colors.get(bonus_timer.index_color).unwrap().clone();
        }
    }
}

/* // Utiliser avec le sprite
fn bonus_color_timer_system(time: Res<Time>, mut query: Query<(&mut Sprite, &mut BonusTimer), With<BonusTimer>>,) {
    for (mut sprite, mut bonus_timer) in query.iter_mut() {
        bonus_timer.color_timer.tick(time.delta());
        if bonus_timer.color_timer.finished() {
            bonus_timer.index_color += 1 as usize;
            if bonus_timer.index_color >= bonus_timer.colors.len() { bonus_timer.index_color = 0; }
            sprite.color = bonus_timer.colors.get(bonus_timer.index_color).unwrap().clone();
        }
    }
} */

fn cleanup_food_system(mut commands: Commands, mut query: Query<Entity, With<Food>>) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn();
        // println!("food");
    }
}