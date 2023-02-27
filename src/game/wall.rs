use bevy::{prelude::{Plugin, App, StartupStage, Commands, Res, Transform, Vec3, SystemSet, Query, Entity, With, CoreStage, ResMut}, sprite::{SpriteBundle, Sprite}};
use iyes_loopless::prelude::{IntoConditionalSystem, ConditionSet};

use crate::{AppState, main_menu::sub_menu::GameType};

use super::{ArenaSize, ARENA_HEIGHT, WALL_COLOR, ARENA_WIDTH, EXTERIOR_WALL_OFFSET, EXTERIOR_WALL_THICKNESS_COEFF, EXTERIOR_WALL_LENGTH_COEFF, components::{Wall, Collision, Position}, GamePlugin, INTERIOR_WALL_THICKNESS_COEFF, INTERIOR_WALL_LENGTH_COEFF, PositionsAvailable};



pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        // app.add_startup_system_to_stage(StartupStage::PostStartup, exterior_walls_spawn_system);
        app
            .add_system_set(
                ConditionSet::new()
                    .run_if(wall_not_exists)
                    .run_in_bevy_state(AppState::InGame)
                    .with_system(exterior_walls_spawn_system)
                    .with_system(interior_walls_spawn_system).into()
            )
            // .add_startup_system_set_to_stage(
            //     CoreStage::PostStartup,
            //     ConditionSet::new()
            //         .
            //         .run_in_bevy_state(AppState::InGame)
            //         .with_system(exterior_walls_spawn_system).into()
            // )
            .add_system_set(
                SystemSet::on_exit(AppState::InGame)
                .with_system(cleanup_wall_system)
            );;
    }
}

fn wall_not_exists(query: Query<&Wall>) -> bool {
    if query.iter().count() > 0 {
        return false;
    }
    true
}

fn exterior_walls_spawn_system(
    mut commands: Commands,
    arena_size: Res<ArenaSize>,
    game_type: Res<GameType>,
) {
    let multiplier_wall_thickness = match game_type.wall_type {
        1 => 3,
        _ => 1
    };
    
    exterior_walls_spawn_by_axe(&mut commands, &arena_size, multiplier_wall_thickness, "vertical");
    exterior_walls_spawn_by_axe(&mut commands, &arena_size, multiplier_wall_thickness, "horizontal");    
    exterior_wall_corners_spawn(&mut commands, &arena_size);
}

fn exterior_walls_spawn_by_axe(mut commands: &mut Commands, arena_size: &Res<ArenaSize>, multiplier_wall_thickness: u32, axe: &str) {
    match axe {
        "vertical" => {
            for y in 0..arena_size.tile_height {
                // bord gauche
                let left_brick = SpriteBundle {
                    sprite: Sprite {
                        color: WALL_COLOR,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(
                            convert(-EXTERIOR_WALL_OFFSET, arena_size.px_width, arena_size.tile_width as f32) - (EXTERIOR_WALL_THICKNESS_COEFF / arena_size.tile_width as f32 * arena_size.px_width),
                            convert(y as f32, arena_size.px_height, arena_size.tile_height as f32),
                            0.0,
                        ),
                        scale: Vec3::new(
                            EXTERIOR_WALL_THICKNESS_COEFF * (multiplier_wall_thickness as f32) / arena_size.tile_width as f32 * arena_size.px_width,
                            EXTERIOR_WALL_LENGTH_COEFF / arena_size.tile_height as f32 * arena_size.px_height,
                            1.0,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                };
        
                // bord droit
                let right_brick = SpriteBundle {
                    sprite: Sprite {
                        color: WALL_COLOR,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(
                            convert(arena_size.tile_width as f32 - EXTERIOR_WALL_OFFSET, arena_size.px_width, arena_size.tile_width as f32) + (EXTERIOR_WALL_THICKNESS_COEFF / arena_size.tile_width as f32 * arena_size.px_width),
                            convert(y as f32, arena_size.px_height, arena_size.tile_height as f32),
                            0.0,
                        ),
                        scale: Vec3::new(
                            EXTERIOR_WALL_THICKNESS_COEFF * (multiplier_wall_thickness as f32) / arena_size.tile_width as f32 * arena_size.px_width,
                            EXTERIOR_WALL_LENGTH_COEFF / arena_size.tile_height as f32 * arena_size.px_height,
                            1.0,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                };

                if multiplier_wall_thickness != 1 {
                    commands.spawn(left_brick)
                        .insert(Position {x: - (EXTERIOR_WALL_OFFSET * 2.) as i32, y: y as i32})
                        .insert(Wall)
                        .insert(Collision);
                    commands.spawn(right_brick)
                        .insert(Position {x: (arena_size.tile_width as f32) as i32, y: y as i32})
                        .insert(Wall)
                        .insert(Collision);
                } else {
                    commands.spawn(left_brick)
                        .insert(Position {x: - (EXTERIOR_WALL_OFFSET * 2.) as i32, y: y as i32})
                        .insert(Wall);
                    commands.spawn(right_brick)
                        .insert(Position {x: (arena_size.tile_width as f32 ) as i32, y: y as i32})
                        .insert(Wall);
                }
            }
        },
        "horizontal" => {
            for x in 0..arena_size.tile_width {
                // bord superieur
                let top_brick = SpriteBundle {
                    sprite: Sprite {
                        color: WALL_COLOR,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(
                            convert(x as f32, arena_size.px_width, arena_size.tile_width as f32),
                            convert(arena_size.tile_height as f32 - EXTERIOR_WALL_OFFSET, arena_size.px_height, arena_size.tile_height as f32) + (EXTERIOR_WALL_THICKNESS_COEFF / arena_size.tile_width as f32 * arena_size.px_width),
                            0.0,
                        ),
                        scale: Vec3::new(
                            EXTERIOR_WALL_LENGTH_COEFF / arena_size.tile_width as f32 * arena_size.px_width,
                            EXTERIOR_WALL_THICKNESS_COEFF * (multiplier_wall_thickness as f32) / arena_size.tile_height as f32 * arena_size.px_height,
                            1.0,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                };
        
                // bord inferieur
                let bottom_brick = SpriteBundle {
                    sprite: Sprite {
                        color: WALL_COLOR,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(
                            convert(x as f32, arena_size.px_width, arena_size.tile_width as f32),
                            convert(-EXTERIOR_WALL_OFFSET, arena_size.px_height, arena_size.tile_height as f32) - (EXTERIOR_WALL_THICKNESS_COEFF / arena_size.tile_width as f32 * arena_size.px_width),
                            0.0,
                        ),
                        scale: Vec3::new(
                            EXTERIOR_WALL_LENGTH_COEFF / arena_size.tile_width as f32 * arena_size.px_width,
                            EXTERIOR_WALL_THICKNESS_COEFF * (multiplier_wall_thickness as f32) / arena_size.tile_height as f32 * arena_size.px_height,
                            1.0,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                };
                
                if multiplier_wall_thickness != 1 {
                    commands.spawn(top_brick)
                        .insert(Position {x: x as i32, y: (arena_size.tile_height as f32) as i32})
                        .insert(Wall)
                        .insert(Collision);
                    commands.spawn(bottom_brick)
                        .insert(Position {x: x as i32, y: - (EXTERIOR_WALL_OFFSET * 2.) as i32})
                        .insert(Wall)
                        .insert(Collision);
                } else {
                    commands.spawn(top_brick)
                        .insert(Position {x: x as i32, y: (arena_size.tile_height as f32) as i32})
                        .insert(Wall);
                    commands.spawn(bottom_brick)
                        .insert(Position {x: x as i32, y: - (EXTERIOR_WALL_OFFSET * 2.) as i32})
                        .insert(Wall);
                }
            }

        },
        _ => ()
    }
}

fn exterior_wall_corners_spawn(mut commands: &mut Commands, arena_size: &Res<ArenaSize>) {
    // (x, y, decalage x, decalage y)
    let corner_positions: Vec<(f32,f32,f32,f32)> = vec![
        (-EXTERIOR_WALL_OFFSET, -EXTERIOR_WALL_OFFSET, -EXTERIOR_WALL_THICKNESS_COEFF / arena_size.tile_width as f32 * arena_size.px_width, -EXTERIOR_WALL_THICKNESS_COEFF / arena_size.tile_height as f32 * arena_size.px_height), // coin inférieur gauche
        (-EXTERIOR_WALL_OFFSET, arena_size.tile_height as f32 - EXTERIOR_WALL_OFFSET, -EXTERIOR_WALL_THICKNESS_COEFF / arena_size.tile_width as f32 * arena_size.px_width, EXTERIOR_WALL_THICKNESS_COEFF / arena_size.tile_height as f32 * arena_size.px_height), // coin supérieur gauche
        (arena_size.tile_width as f32 - EXTERIOR_WALL_OFFSET, -EXTERIOR_WALL_OFFSET, EXTERIOR_WALL_THICKNESS_COEFF / arena_size.tile_width as f32 * arena_size.px_width, -EXTERIOR_WALL_THICKNESS_COEFF / arena_size.tile_height as f32 * arena_size.px_height), // coin inférieur droit
        (arena_size.tile_width as f32 - EXTERIOR_WALL_OFFSET, arena_size.tile_height as f32 - EXTERIOR_WALL_OFFSET, EXTERIOR_WALL_THICKNESS_COEFF / arena_size.tile_width as f32 * arena_size.px_width, EXTERIOR_WALL_THICKNESS_COEFF / arena_size.tile_height as f32 * arena_size.px_height) // coin supérieur droit
    ];

    for corner_position in corner_positions.into_iter() {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(
                    convert(corner_position.0, arena_size.px_width, arena_size.tile_width as f32) + corner_position.2,
                    convert(corner_position.1, arena_size.px_height, arena_size.tile_height as f32) + corner_position.3,
                    0.0,
                ),
                scale: Vec3::new(
                    EXTERIOR_WALL_THICKNESS_COEFF / arena_size.tile_width as f32 * arena_size.px_width,
                    EXTERIOR_WALL_THICKNESS_COEFF / arena_size.tile_height as f32 * arena_size.px_height,
                    1.0,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Position {x: corner_position.0 as i32, y: corner_position.1 as i32})
        .insert(Wall);
    }
}

fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
	let tile_size = bound_window / bound_game;
	pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
}

fn interior_walls_spawn_system(
    mut commands: Commands,
    arena_size: Res<ArenaSize>,
    game_type: Res<GameType>,
    mut positions_available: ResMut<PositionsAvailable>
) {
    match game_type.wall_type {
        2 => interior_walls_spawn(&mut commands, &arena_size, &mut positions_available, "vertical"),
        3 => interior_walls_spawn(&mut commands, &arena_size, &mut positions_available, "horizontal"),
        4 => {
            interior_walls_spawn(&mut commands, &arena_size, &mut positions_available, "vertical");
            interior_walls_spawn(&mut commands, &arena_size, &mut positions_available, "horizontal");
        }
        _ => return,
    }
}

fn interior_walls_spawn(mut commands: &mut Commands, arena_size: &Res<ArenaSize>, mut positions_available: &mut ResMut<PositionsAvailable>, axe: &str) {
    match axe {
        "vertical" => {
            let middle = get_middle(arena_size.tile_width);

            for y in 0..arena_size.tile_height {

                let wall_position = Position {x: middle as i32, y: y as i32};
                positions_available.0.remove(&wall_position);

                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: WALL_COLOR,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(
                            convert(middle as f32, arena_size.px_width, arena_size.tile_width as f32),
                            convert(y as f32, arena_size.px_height, arena_size.tile_height as f32),
                            0.0,
                        ),
                        scale: Vec3::new(
                            INTERIOR_WALL_THICKNESS_COEFF / arena_size.tile_width as f32 * arena_size.px_width,
                            INTERIOR_WALL_LENGTH_COEFF / arena_size.tile_height as f32 * arena_size.px_height,
                            1.0,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(wall_position)
                .insert(Wall)
                .insert(Collision);
            }
        },
        "horizontal" => {
            let middle = get_middle(arena_size.tile_height);

            for x in 0..arena_size.tile_width {

                let wall_position = Position {x: x as i32, y: middle as i32};
                positions_available.0.remove(&wall_position);
                
                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: WALL_COLOR,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(
                            convert(x as f32, arena_size.px_width, arena_size.tile_width as f32),
                            convert(middle as f32 , arena_size.px_height, arena_size.tile_height as f32),
                            0.0,
                        ),
                        scale: Vec3::new(
                            INTERIOR_WALL_LENGTH_COEFF / arena_size.tile_width as f32 * arena_size.px_width,
                            INTERIOR_WALL_THICKNESS_COEFF/ arena_size.tile_height as f32 * arena_size.px_height,
                            1.0,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(wall_position)
                .insert(Wall)
                .insert(Collision);
            }

        },
        _ => ()
    }
}

fn get_middle(size: u32) -> u32 {
    match size % 2 {
        0 => {
            (size + 1) / 2
        },
        _ => size / 2
    }
}

fn cleanup_wall_system(mut commands: Commands, mut query: Query<Entity, With<Wall>>) {
    // let mut count = 0;
    
    for entity in query.iter_mut() {
        // print!("wall ");
        commands.entity(entity).despawn();
        // count += 1;
        // println!("{}", count);
    }
    // println!("mur fini");
}