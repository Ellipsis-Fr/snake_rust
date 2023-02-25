#![allow(unused)]
mod game;
use game::GamePlugin;

mod main_menu;
use main_menu::MainMenuPlugin;

// use main_menu::sub_menu;

use bevy::{prelude::*, time::FixedTimestep, text::Text2dBounds, ecs::query};
use iyes_loopless::{prelude::AppLooplessStateExt, state::{CurrentState, NextState}};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    SubMenu,
    InGame,
	Pause,
	GameOver(bool)
}

// region:    --- Game Constants

const WINDOW_WIDTH: f32 = 600.;
const WINDOW_HEIGHT: f32 = 600.;
const UPPER_EDGE : f32 = 0.1;

const ARENA_WIDTH: u32 = 20;
const ARENA_HEIGHT: u32 = 20;

// endregion: --- Game Constants


// TODO : adapter la taille de la fenêtre en fonction du type de partie
// TODO : ajouter gestion de fin de partie (si jamais le serpent = (W x H) - 1) : Manque l'effacement du texte
// TODO : ajouter des bonus :
// TODO : - un permettant de temporairement désactiver les murs (inactif dans une partie sans mur)
// TODO : - un donnant un bonus de points, il aura une durée de vie de 4s seulement
// TODO : Constat d'une erreur de despawn certainement du à la simultanéité du fin de tps de vie de la nourriture et du fait que le serpent l'ai mangée

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04,0.04,0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
			window: WindowDescriptor {
				title: "Snake".to_string(),
				width: WINDOW_WIDTH + (WINDOW_WIDTH / ARENA_WIDTH as f32) * 3., // 600 + 2 * 6 + 12 // taille initiale + epaisseur mur + espace libre
				height: WINDOW_HEIGHT + (WINDOW_HEIGHT * UPPER_EDGE) + (WINDOW_HEIGHT / ARENA_HEIGHT as f32) * 2., // 600 + 600 * 0.1 + 2 * 6 + 12 // taille initiale + espace de score + epaisseur mur + espace libre
				resizable: false,
				..Default::default()
			},
			..Default::default()
		}))
		.add_state(AppState::MainMenu)
		.add_plugin(GamePlugin)
        .add_plugin(MainMenuPlugin)
		.run();
}