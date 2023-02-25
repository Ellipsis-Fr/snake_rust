use crate::main_menu::MenuMaterials;

use super::{AppState, MenuData};

mod components;
use bevy::{prelude::*, ui::Interaction, app::AppExit};
use components::MenuButton;
use enum_index::EnumIndex;


// region:    --- Resource

#[derive(Resource)]
pub struct GameType {
	pub wall_type: usize,
    pub multiplier: u32
}

// endregion: --- Resource
pub struct SubMenuPlugin;

impl Plugin for SubMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(button_press_system)
            .add_system_set(SystemSet::on_enter(AppState::SubMenu).with_system(setup_system))
            .add_system_set(SystemSet::on_exit(AppState::SubMenu).with_system(cleanup_system));
    }
}

fn button_press_system(
    mut commands: Commands,
    buttons: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<State<AppState>>,
    mut exit: EventWriter<AppExit>
) {
    for (interaction, button) in buttons.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                MenuButton::Quit => exit.send(AppExit),
                MenuButton::WithoutWall(x) => commands.insert_resource(GameType {wall_type: button.enum_index(), multiplier: *x}),
                MenuButton::ExteriorWall(x) => commands.insert_resource(GameType {wall_type: button.enum_index(), multiplier: *x}),
                MenuButton::VerticalWall(x) => commands.insert_resource(GameType {wall_type: button.enum_index(), multiplier: *x}),
                MenuButton::HorizontalWall(x) => commands.insert_resource(GameType {wall_type: button.enum_index(), multiplier: *x}),
                MenuButton::VerticalAndHorizontalWall(x) => commands.insert_resource(GameType {wall_type: button.enum_index(), multiplier: *x}),
            };

            state.set(AppState::InGame).expect("Couldn't switch state to InGame");
        }
    }
}

fn root(materials: &Res<MenuMaterials>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        // material: materials.root.clone(),
        ..Default::default()
    }
}

fn border(materials: &Res<MenuMaterials>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(400.0), Val::Auto),
            border: UiRect::all(Val::Px(8.0)),
            ..Default::default()
        },
        // material: materials.border.clone(),
        ..Default::default()
    }
}

fn menu_background(materials: &Res<MenuMaterials>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::ColumnReverse,
            padding: UiRect::all(Val::Px(5.0)),
            ..Default::default()
        },
        // material: materials.menu.clone(),
        ..Default::default()
    }
}

fn button(materials: &Res<MenuMaterials>) -> ButtonBundle {
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

fn button_text(asset_server: &Res<AssetServer>, materials: &Res<MenuMaterials>, label: &str) -> TextBundle {
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
                color: materials.button_text.clone(),
            },
        ),
        ..Default::default()
    };
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: Res<MenuMaterials>,
    mut menu_data: ResMut<MenuData>
) {
    let ui_root = 
    commands
        .spawn(root(&materials))
            .with_children(|parent| {
                // left vertical fill (border)
                parent
                    .spawn(border(&materials))
                    .with_children(|parent| {
                        // left vertical fill (content)
                        parent
                            .spawn(menu_background(&materials))
                            .with_children(|parent| {
                                parent.spawn(button(&materials))
                                    .with_children(|parent| {
                                        parent.spawn(button_text(&asset_server,  &materials, "Quitter"));
                                    })
                                    .insert(MenuButton::Quit);
                                parent.spawn(button(&materials))
                                    .with_children(|parent| {
                                        parent.spawn(button_text(&asset_server, &materials, "Mur Vertical et Horizontal"));
                                    })
                                    .insert(MenuButton::VerticalAndHorizontalWall(5));
                                parent.spawn(button(&materials))
                                    .with_children(|parent| {
                                        parent.spawn(button_text(&asset_server, &materials, "Mur Extérieur"));
                                    })
                                    .insert(MenuButton::ExteriorWall(3));
                                parent.spawn(button(&materials))
                                    .with_children(|parent| {
                                        parent.spawn(button_text(&asset_server, &materials, "Mur Horizontal"));
                                    })
                                    .insert(MenuButton::HorizontalWall(2));
                                parent.spawn(button(&materials))
                                    .with_children(|parent| {
                                        parent.spawn(button_text(&asset_server, &materials, "Mur Vertical"));
                                    })
                                    .insert(MenuButton::VerticalWall(2));
                                parent.spawn(button(&materials))
                                    .with_children(|parent| {
                                        parent.spawn(button_text(&asset_server, &materials, "Sans Obsctacle"));
                                    })
                                    .insert(MenuButton::WithoutWall(1));
                                
                            });
                    });
            })
        .id();

    menu_data.ui_root = ui_root;
}


fn cleanup_system(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.ui_root).despawn_recursive();
    commands.entity(menu_data.camera_entity).despawn_recursive();

    commands.remove_resource::<MenuData>();
}