use super::{AppState, Score};

use bevy::{prelude::*, ui::Interaction, app::AppExit};


// region:    --- Resource

#[derive(Resource)]
struct MenuData {
    camera_entity: Entity,
    ui_root: Entity,
}

// endregion: --- Resource
pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_update(AppState::GameOver(true)).with_system(button_press_system))
            .add_system_set(SystemSet::on_update(AppState::GameOver(true)).with_system(keyboard_event_system))
            .add_system_set(SystemSet::on_enter(AppState::GameOver(true)).with_system(setup_system).with_system(score_system))
            .add_system_set(SystemSet::on_exit(AppState::GameOver(true)).with_system(cleanup_system))
            .add_system_set(SystemSet::on_update(AppState::GameOver(false)).with_system(button_press_system))
            .add_system_set(SystemSet::on_update(AppState::GameOver(false)).with_system(keyboard_event_system))
            .add_system_set(SystemSet::on_enter(AppState::GameOver(false)).with_system(setup_system).with_system(score_system))
            .add_system_set(SystemSet::on_exit(AppState::GameOver(false)).with_system(cleanup_system));
    }
}

fn button_press_system(
    buttons: Query<(&Interaction), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<State<AppState>>,
    mut exit: EventWriter<AppExit>
) {
    for (interaction) in buttons.iter() {
        if *interaction == Interaction::Clicked {
            state.set(AppState::MainMenu).expect("Couldn't switch state to MainMenu");
        }
    }
}

fn keyboard_event_system(mut keys: ResMut<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

fn root() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        ..Default::default()
    }
}

fn border() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(400.0), Val::Auto),
            border: UiRect::all(Val::Px(8.0)),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn menu_background() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::ColumnReverse,
            padding: UiRect::all(Val::Px(5.0)),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        // material: materials.button.clone(),
        ..Default::default()
    }
}

fn button_text(asset_server: &Res<AssetServer>, label: &str) -> TextBundle {
    return TextBundle {
        style: Style {
            margin: UiRect::all(Val::Px(10.0)),
            ..Default::default()
        },
        text: Text::from_section(
            label,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: Color::BLACK,
            },
        ),
        ..Default::default()
    };
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera_entity = commands.spawn(Camera2dBundle::default()).id();

    let ui_root = 
    commands
        .spawn(root())
            .with_children(|parent| {
                // left vertical fill (border)
                parent
                    .spawn(border())
                    .with_children(|parent| {
                        // left vertical fill (content)
                        parent
                            .spawn(menu_background())
                            .with_children(|parent| {
                                parent.spawn(button())
                                    .with_children(|parent| {
                                        parent.spawn(button_text(&asset_server, "Menu"));
                                    });                                
                            });
                    });
            })
        .id();

    commands.insert_resource(MenuData {
        camera_entity,
        ui_root,
    });
}

fn score_system(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	score: Res<Score>,
    app_state: Res<State<AppState>>,
) {
	let font = asset_server.load("fonts/FiraSans-Bold.ttf");
	let text_style = TextStyle {
        font,
        font_size: 40.,
        color: Color::WHITE,
    };

    let text;

    match app_state.current() {
        AppState::GameOver(true) => text = "Gagné !\n".to_owned() + &score.0.to_string(),
        _ => text = "Perdu !\n".to_owned() + &score.0.to_string()
    }
	
	commands.spawn(Text2dBundle {
		text: Text::from_section(
			text,
			text_style.clone()
		)
        .with_alignment(TextAlignment::TOP_CENTER),
        transform: Transform {
            translation: Vec3::new(0., 100., 1.),
            ..Default::default()
        },
		..default()
	});
}

fn cleanup_system(mut commands: Commands, menu_data: Res<MenuData>, mut query: Query<Entity, (With<Text>, Without<Style>)>) {
    // "Without<Style>" car cela comprenait le texte présent dans le bouton, ce qui posait pb pour mon précédent code... 
    if let Ok(entity) = query.get_single() {
		commands.entity(entity).despawn_recursive();
	}

    commands.entity(menu_data.ui_root).despawn_recursive();
    // println!("ui_root");
    commands.entity(menu_data.camera_entity).despawn_recursive();
    // println!("camera");
    commands.remove_resource::<Score>();
    // println!("score");
    commands.remove_resource::<MenuData>();
    // println!("menudata");
}