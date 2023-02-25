use super::AppState;
use bevy::{prelude::*, app::AppExit};
mod components;
use components::MenuButton;

pub(crate) mod sub_menu;
use sub_menu::SubMenuPlugin;

pub struct MainMenuPlugin;


// region:    --- Resources

#[derive(Resource)]
struct MenuData {
    camera_entity: Entity,
    ui_root: Entity,
}

#[derive(Resource)]
struct MenuMaterials {
    root: Handle<ColorMaterial>,
    border: Handle<ColorMaterial>,
    menu: Handle<ColorMaterial>,
    button: Handle<ColorMaterial>,
    button_hovered: Handle<ColorMaterial>,
    button_pressed: Handle<ColorMaterial>,
    button_text: Color,
}

// endregion: --- Resources

impl FromWorld for MenuMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        MenuMaterials {
            root: materials.add(Color::NONE.into()),
            border: materials.add(Color::rgb(0.65, 0.65, 0.65).into()),
            menu: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            button: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            button_hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            button_pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
            button_text: Color::BLACK,
        }
    }
}

fn button_system(
    materials: Res<MenuMaterials>,
    mut buttons: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<Button>),
    >
) {
    for (interaction, mut material) in buttons.iter_mut() {
        match *interaction {
            Interaction::Clicked => *material = materials.button_pressed.clone(),
            Interaction::Hovered => *material = materials.button_hovered.clone(),
            Interaction::None => *material = materials.button.clone(),
        }
    }
}

fn button_press_system(
    buttons: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<State<AppState>>,
    mut exit: EventWriter<AppExit>
) {
    for (interaction, button) in buttons.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                MenuButton::Play => state
                    .set(AppState::SubMenu)
                    .expect("Couldn't switch state to SubMenu"),
                MenuButton::Quit => exit.send(AppExit),
            };
        }
    }
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuMaterials>()
            .add_plugin(SubMenuPlugin)
            .add_system(button_system)
            .add_system(button_press_system)
            .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup_system))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(cleanup_system));
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
) {
    let camera_entity = commands.spawn(Camera2dBundle::default()).id();

    let ui_root = commands
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
                                    parent.spawn(button_text(&asset_server, &materials, "Quitter"));
                                })
                                .insert(MenuButton::Quit);
                            parent.spawn(button(&materials))
                                .with_children(|parent| {
                                    parent.spawn(button_text(&asset_server, &materials, "Nouvelle Partie"));
                                })
                                .insert(MenuButton::Play);
                        });
                });
        })
        .id();

    commands.insert_resource(MenuData {
        camera_entity,
        ui_root,
    });
}

fn cleanup_system(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.ui_root).despawn_recursive();
}