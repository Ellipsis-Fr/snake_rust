mod components;
use std::{time::Duration, collections::HashSet};

use components::{SnakeHead, Direction, Size, Position, SnakeBody, Food, FoodTimer, Wall, Collision};
mod snake;
use iyes_loopless::prelude::{IntoConditionalSystem, ConditionSet, AppLooplessFixedTimestepExt};
use snake::SnakePlugin;
mod food;
use food::FoodPlugin;
mod wall;
use wall::WallPlugin;
mod game_over;
use game_over::GameOverPlugin;

use bevy::{prelude::*, time::FixedTimestep, text::Text2dBounds, ecs::query};
use uuid::Uuid;
use crate::{WINDOW_WIDTH, ARENA_WIDTH, WINDOW_HEIGHT, UPPER_EDGE, ARENA_HEIGHT, main_menu::sub_menu::GameType};

use self::components::FoodType;

use super::AppState;

// region:    --- Game Constants

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const SNAKE_HEAD_SIZE: f32 = 0.8;

const SNAKE_BODY_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
const SNAKE_BODY_SIZE: f32 = 0.6;

const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);
const GOLD_FOOD_COLOR: Color = Color::rgb(1.0, 0.84, 0.);
const FOOD_SIZE: f32 = 0.8;

const TIME_STEP: f32 = 1./60.;
const BASE_SPEED: f32 = 60.;

const FOOD_MAX: u32 = 3;

const WALL_COLOR: Color = Color::rgb(1., 1., 1.);
const EXTERIOR_WALL_OFFSET: f32 = 0.5;
const EXTERIOR_WALL_THICKNESS_COEFF: f32 = 0.125;
const EXTERIOR_WALL_LENGTH_COEFF: f32 = 1.;
const INTERIOR_WALL_THICKNESS_COEFF: f32 = 0.5;
const INTERIOR_WALL_LENGTH_COEFF: f32 = 1.;
// endregion: --- Game Constants

// region:    --- Resources

#[derive(Resource)]
pub struct WinSize {
	pub width: f32,
	pub height: f32,
}

#[derive(Resource)]
pub struct ArenaSize {
	pub width: f32,
	pub height: f32,
}

#[derive(Resource)]
struct FoodCount(u32);

#[derive(Resource)]
struct GameTextures {
	bonus_star: Handle<Image>,
}

#[derive(Resource)]
struct Score(u32);

#[derive(Resource)]
struct Camera(Entity);

#[derive(Resource)]
struct PositionsAvailable(HashSet<Position>);
// endregion: --- Resources

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
		// .insert_resource(PositionsAvailable(HashSet::new()))
        .add_plugin(SnakePlugin)
		.add_plugin(FoodPlugin)
		.add_plugin(WallPlugin)
		.add_plugin(GameOverPlugin)
        .add_system_set(
			SystemSet::on_enter(AppState::InGame)
			.with_system(setup_system)
		)
		.add_fixed_timestep(
			Duration::from_millis(150),
			// give it a label
			"snake_move_time",
		)
		.add_fixed_timestep_system(
			"snake_move_time",
			0,
			snake_movement_system
				.run_in_bevy_state(AppState::InGame)
		)
		.add_system_set_to_stage(
			CoreStage::PostUpdate,
			ConditionSet::new()
				.run_in_bevy_state(AppState::InGame)
				.with_system(position_translation_system)
				.with_system(size_scaling_system).into(),
		)
		// .add_system_set_to_stage(
		// 	CoreStage::Last,
		// 	ConditionSet::new()
		// 		.run_in_bevy_state(AppState::InGame)
		// 		.with_system(check_correct_snake_head_position_system).into(),
		// )
		.add_system(check_correct_snake_head_position_system.run_in_bevy_state(AppState::InGame))
		.add_system(snake_ate_food_system.run_in_bevy_state(AppState::InGame))
		.add_system(score_system.run_in_bevy_state(AppState::InGame))
		.add_system(check_end_of_game_system.run_in_bevy_state(AppState::InGame))
		// .add_system_set_to_stage(
		// 	CoreStage::PostUpdate,
		// 	ConditionSet::new()
		// 		.run_in_bevy_state(AppState::InGame)
		// 		.with_system(score_system).into()
		// 	)
		.add_system_set(SystemSet::on_update(AppState::InGame).with_system(back_to_main_menu_controls_system))
		.add_system_set(
            SystemSet::on_update(AppState::Pause)
                .with_system(back_to_main_menu_controls_system)
        )
		.add_system_set(
			SystemSet::on_exit(AppState::InGame)
			.with_system(cleanup_system)
		);
    }
}

fn setup_system(
	mut commands: Commands,
	mut windows: ResMut<Windows>,
	asset_server: Res<AssetServer>,
	game_type: Res<GameType>,
	// mut positions_available: ResMut<PositionsAvailable>
) {
	// camera
	let camera = commands.spawn(Camera2dBundle {
		transform: Transform::from_xyz(0., WINDOW_HEIGHT * UPPER_EDGE / 2., 1000. - 0.1),
		..Default::default()
	}).id();

	commands.insert_resource(Camera(camera));

	// capture window size
	let window = windows.get_primary_mut().unwrap();
	let (win_w, win_h) = (window.width(), window.height());


	// position window (for tutorial)
	// window.set_position(MonitorSelection::Current, IVec2::new(win_w as i32, -win_h as i32));

	// add WinSize resource
	commands.insert_resource(WinSize { width: win_w, height: win_h });

	// add ArenaSize resource
	commands.insert_resource(ArenaSize { width: WINDOW_WIDTH, height: WINDOW_HEIGHT });


	// add count food resource
	commands.insert_resource(FoodCount(0));

	// add GameTextures resource
	commands.insert_resource(GameTextures {bonus_star: asset_server.load("star.png")});
	
	// add score resource
	commands.insert_resource(Score(0));

	// init positions available
	commands.insert_resource(PositionsAvailable(get_all_arena_positions()));

	// positions_available.0.extend(get_all_arena_positions());

	// add wall
	// let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    // let wall_thickness = 10.0;
    // let bounds = Vec2::new(900.0, 600.0);

    // commands
    //     // left
    //     .spawn(SpriteBundle {
    //         material: wall_material.clone(),
    //         transform: Transform::from_xyz(-bounds.x / 2.0, 0.0, 0.0),
    //         sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
    //         ..Default::default()
    //     })
    //     .with(Collider::Solid)
    //     // right
    //     .spawn(SpriteBundle {
    //         material: wall_material.clone(),
    //         transform: Transform::from_xyz(bounds.x / 2.0, 0.0, 0.0),
    //         sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
    //         ..Default::default()
    //     })
    //     .with(Collider::Solid)
    //     // bottom
    //     .spawn(SpriteBundle {
    //         material: wall_material.clone(),
    //         transform: Transform::from_xyz(0.0, -bounds.y / 2.0, 0.0),
    //         sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
    //         ..Default::default()
    //     })
    //     .with(Collider::Solid)
    //     // top
    //     .spawn(SpriteBundle {
    //         material: wall_material,
    //         transform: Transform::from_xyz(0.0, bounds.y / 2.0, 0.0),
    //         sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
    //         ..Default::default()
    //     })
    //     .with(Collider::Solid);

	// add coordinates on map
	// map_coordinates(commands, asset_server, win_w, win_h);
}

fn get_all_arena_positions() -> HashSet<Position> {
	let mut all_arena_positions = HashSet::new();

	for y in 0..ARENA_HEIGHT {
		for x in 0..ARENA_WIDTH {
			all_arena_positions.insert(Position{x: x as i32, y: y as i32});
		}
	}

	all_arena_positions
}

/* fn map_coordinates(mut commands: Commands, asset_server: Res<AssetServer>, win_w: f32, win_h: f32) {

	let font = asset_server.load("fonts/FiraSans-Bold.ttf");
	let text_style = TextStyle {
        font,
        font_size: 10.,
        color: Color::WHITE,
    };

	for y in 0..ARENA_HEIGHT {
		let mut ordinate_name = char::from_u32(y + 65).unwrap();
		for x in 0..ARENA_WIDTH {
			let mut abscissa_name = (x + 1).to_string();
			ordinate_name.to_string().push_str(&abscissa_name);

			commands.spawn(Text2dBundle {
				text: Text::from_section(
					ordinate_name,
					text_style.clone()
				),
				// text_2d_bounds: Text2dBounds {
				// 	size: Vec2::new(win_w / ARENA_WIDTH as f32, (win_h / (1. + UPPER_EDGE)) / ARENA_HEIGHT as f32)
				// },
				transform: Transform {
					translation: Vec3::new(
						convert(x as f32, win_w, ARENA_WIDTH as f32),
						convert(y as f32, (win_h / (1. + UPPER_EDGE)), ARENA_HEIGHT as f32),
						1.
					),
					// scale: Vec3::new(100., 20., 1.),
					..Default::default()
				},
				..default()
			});
		}
	}	
} */

fn snake_movement_system(
	mut commands: Commands,
	game_type: Res<GameType>,
	mut snake_head_query: Query<(&Direction, &mut Position, &mut SnakeHead)>,
	mut snake_body_query: Query<(Entity, &mut SnakeBody)>,
) {
	if let Ok((snake_direction, mut snake_position, mut snake_head)) = snake_head_query.get_single_mut() {
		let snake_head_actual_position = snake_position.clone();

		update_snake_head_position(snake_direction, &mut snake_position, game_type.wall_type);
		snake_head.moved = true;
		
		add_new_body_part(&mut commands, snake_head_actual_position, &mut *snake_head);

		if snake_head.ate {
			snake_head.ate = false;
		} else {
			remove_body_part(snake_head, snake_body_query, &mut commands);
		}
	}
}

fn update_snake_head_position(snake_direction: &Direction, mut snake_position: &mut Mut<Position>, wall_type: usize) {
	let can_pass = wall_type != 1;
	
	match snake_direction {
		Direction::UP => {
			snake_position.y += 1;
			if snake_position.y == ARENA_HEIGHT as i32 && can_pass {
				snake_position.y = 0;
			}
		},
		Direction::DOWN => {
			snake_position.y -= 1;
			if snake_position.y < 0 && can_pass {
				snake_position.y = ARENA_HEIGHT as i32 - 1;
			}
		},
		Direction::LEFT => {
			snake_position.x -= 1;
			if snake_position.x < 0 && can_pass {
				snake_position.x = ARENA_WIDTH as i32 - 1;
			}
		},
		Direction::RIGHT => {
			snake_position.x += 1;
			if snake_position.x == ARENA_WIDTH as i32 && can_pass {
				snake_position.x = 0;
			}
		},
	}
}

fn add_new_body_part(commands: &mut Commands, snake_head_actual_position: Position, snake_head: &mut SnakeHead) {
	// add snake_body
    // first snake_body
    let snake_body = SnakeBody {
        is_tail: snake_head.body_parts.len() == 1 && !snake_head.ate,
        id: Uuid::new_v4()
    };

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: SNAKE_BODY_COLOR,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(snake_head_actual_position.x as f32, snake_head_actual_position.y as f32, 10.),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(snake_body.clone()) 
    .insert(Position {x: snake_head_actual_position.x, y: snake_head_actual_position.y})
    .insert(Size::square(0.6));

	snake_head.body_parts.push_back(snake_body);
}

fn remove_body_part(mut snake_head: Mut<SnakeHead>, mut snake_body_query: Query<(Entity, &mut SnakeBody)>, commands: &mut Commands) {
    let uuid_ex_body_part = &snake_head.body_parts.pop_front().unwrap().id;
	snake_head.body_parts[0].is_tail = true;
	let uuid_new_tail_body_part = &snake_head.body_parts[0].id;

    for (entity, mut snake_body) in snake_body_query.iter_mut() {
		if snake_body.is_tail && &snake_body.id == uuid_ex_body_part {
			commands.entity(entity).despawn();
		}
		if &snake_body.id == uuid_new_tail_body_part {
			snake_body.is_tail = true;
		}
	}
}

fn size_scaling_system(arena_size: Res<ArenaSize>, mut q: Query<(&Size, &mut Transform)>) {
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * arena_size.width,
            sprite_size.height / ARENA_HEIGHT as f32 * arena_size.height,
            1.0,
        );
    }
}

fn position_translation_system(
	arena_size: Res<ArenaSize>,
	mut snake_head_query: Query<(&Position, &mut Transform), With<SnakeHead>>,
	mut snake_body_query: Query<(&Position, &mut Transform), (With<SnakeBody>, Without<SnakeHead>)>
) {
	if let Ok((snake_position, mut snake_transform)) = snake_head_query.get_single_mut() {
		snake_transform.translation = Vec3::new(
			convert(snake_position.x as f32, arena_size.width, ARENA_WIDTH as f32),
			convert(snake_position.y as f32, arena_size.height, ARENA_HEIGHT as f32),
			0.0,
		);
	}
	
	for (snake_body_position, mut snake_body_transform) in snake_body_query.iter_mut() {
		snake_body_transform.translation = Vec3::new(
			convert(snake_body_position.x as f32, arena_size.width, ARENA_WIDTH as f32),
			convert(snake_body_position.y as f32, arena_size.height, ARENA_HEIGHT as f32),
			0.0,
		);
	}
}

fn check_correct_snake_head_position_system(
	snake_head_query: Query<&Position, With<SnakeHead>>,
	snake_body_query: Query<&Position, With<SnakeBody>>,
	wall_query: Query<&Position, (With<Wall>, With<Collision>)>,
	mut app_state: ResMut<State<AppState>>
) {
	if let Ok(snake_head_position) = snake_head_query.get_single() {
		if collide_with_body(snake_head_position, snake_body_query) || collide_with_wall(snake_head_position, wall_query) {
			app_state.set(AppState::GameOver(false));
		}
	}
}

fn collide_with_body(snake_head_position: &Position, snake_body_query: Query<&Position, With<SnakeBody>>) -> bool {
	for snake_body_position in snake_body_query.iter() {
		if is_same_position(snake_head_position, snake_body_position) {
			return true;
		}
	}
	false
}

fn collide_with_wall(snake_head_position: &Position, wall_query: Query<&Position, (With<Wall>, With<Collision>)>) -> bool {
	for wall_position in wall_query.iter() {
		if is_same_position(snake_head_position, wall_position) {
			return true;
		}
	}
	false
}


fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
	let tile_size = bound_window / bound_game;
	pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
}

fn snake_ate_food_system(
	mut commands: Commands,
	mut food_count: ResMut<FoodCount>,
	mut score: ResMut<Score>,
	mut snake_head_query: Query<(&Position, &mut SnakeHead), With<SnakeHead>>,
	food_query: Query<(Entity, &Position, &FoodTimer, &Food), With<Food>>,
	game_type: Res<GameType>
) {
	if let Ok((snake_position, mut snake_head)) = snake_head_query.get_single_mut() {
		for (food_entity, food_position, food_timer, food) in food_query.iter() {
			if is_same_position(snake_position, food_position) {
				snake_head.ate = true;

				match food.0 {
					FoodType::Simple => {
						food_count.0 -= 1;
						score.0 += (get_points(food_timer.0.duration().as_secs(), food_timer.0.elapsed().as_secs()) * game_type.multiplier);
					},
					FoodType::Gold => score.0 += (10 * game_type.multiplier),
					_ => ()
				}
				
				commands.entity(food_entity).despawn();
			}
		}
	}
}

fn is_same_position(position: &Position, other_position: &Position) -> bool {
	position.x == other_position.x && position.y == other_position.y
}

fn get_points(duration_secs: u64, elapsed_secs: u64) -> u32 {
	let portion_of_time = ((elapsed_secs as f64 / duration_secs as f64) * 100.) as u32;
		
	match portion_of_time {
		..=25 => 4,
		26..=50 => 3,
		51..=75 => 2,
		_ => 1
	}
}

fn score_system(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	win_size: Res<WinSize>,
	score: Res<Score>,
	mut query: Query<Entity, With<Text>>
) {
	let position_text_y = win_size.height / 2. + WINDOW_HEIGHT * UPPER_EDGE / 2.;
	let font = asset_server.load("fonts/FiraSans-Bold.ttf");
	let text_style = TextStyle {
        font,
        font_size: 40.,
        color: Color::WHITE,
    };

	for entity in query.iter() {
		// println!("destruction avt re-creation");
		// dbg!(entity);
		commands.entity(entity).despawn();
	}
	
	let entity = commands.spawn(Text2dBundle {
		text: Text::from_section(
			score.0.to_string(),
			text_style.clone()
		),
		// .with_alignment(TextAlignment::TOP_CENTER),
        transform: Transform {
            translation: Vec3::new(0., position_text_y - 10., 1.),
            ..Default::default()
        },
		
		// text_2d_bounds: Text2dBounds {
		// 	size: Vec2::new(win_w / ARENA_WIDTH as f32, (win_h / (1. + UPPER_EDGE)) / ARENA_HEIGHT as f32)
		// },
		// transform: Transform {
		// 	translation: Vec3::new(
		// 		convert(x as f32, win_w, ARENA_WIDTH as f32),
		// 		convert(y as f32, (win_h / (1. + UPPER_EDGE)), ARENA_HEIGHT as f32),
		// 		1.
		// 	),
		// 	// scale: Vec3::new(100., 20., 1.),
		// 	..Default::default()
		// },
		..default()
	}).id();
	// println!("creation");
	// dbg!(entity);
}

fn check_end_of_game_system(
	snake_head_query: Query<(&SnakeHead)>,
	mut app_state: ResMut<State<AppState>>,
	game_type: Res<GameType>
) {
	let maximun_snake_size;

	match game_type.wall_type {
		4 => maximun_snake_size = (ARENA_WIDTH - 1) * (ARENA_HEIGHT - 1),
		2 | 3 => maximun_snake_size = (ARENA_WIDTH - 1) * ARENA_HEIGHT,
		_ => maximun_snake_size = ARENA_WIDTH * ARENA_HEIGHT,
	}
	
	let snake_size = 1 + snake_head_query.get_single().unwrap().body_parts.len() as u32;

	if snake_size == maximun_snake_size {
		println!("snake size {}, taille max {}", snake_size, maximun_snake_size);
		app_state.set(AppState::GameOver(true));
	}
}

fn back_to_main_menu_controls_system(mut keys: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if *app_state.current() == AppState::InGame {
        if keys.just_pressed(KeyCode::Escape) {
            app_state.set(AppState::MainMenu).unwrap();
            keys.reset(KeyCode::Escape);
        } else if keys.just_pressed(KeyCode::Space) {
            app_state.push(AppState::Pause).unwrap();
            keys.reset(KeyCode::Space);
			// println!("{:?}", app_state.current());
        }
    } else if *app_state.current() == AppState::Pause {
        // if keys.just_pressed(KeyCode::Escape) {
		// 	app_state.overwrite_set(AppState::MainMenu).unwrap();
        //     keys.reset(KeyCode::Escape);
        // } else if keys.just_pressed(KeyCode::Space) {
        //     app_state.pop().unwrap();
        //     keys.reset(KeyCode::Space);
        // }
		if keys.just_pressed(KeyCode::Space) {
            app_state.pop().unwrap();
            keys.reset(KeyCode::Space);
        }
    }
}

fn cleanup_system(mut commands: Commands, camera: Res<Camera>, mut query: Query<Entity, With<Text>>) {
	if let Ok(entity) = query.get_single() {
		// println!("destruction final");
		// dbg!(entity);
		commands.entity(entity).despawn();
	}
	
	commands.remove_resource::<GameType>();
	// println!("GameType");
	commands.remove_resource::<ArenaSize>();
	// println!("ArenaSize");
	commands.remove_resource::<WinSize>();
	// println!("WinSize");
	commands.remove_resource::<FoodCount>();
	// println!("FoodCount");
	commands.remove_resource::<PositionsAvailable>();
	// println!("PositionsAvailable");
	commands.entity(camera.0).despawn_recursive();
	// println!("camera");
	commands.remove_resource::<Camera>();
	// println!("Camera");
}