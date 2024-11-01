use bevy::app::App;
use bevy::prelude::{AppExtStates, OnExit, States};
use crate::{despawn_screen, scenes};
use crate::scenes::{menu, splash};

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
	#[default]
	Splash,
	Menu,
	Game,
}

pub fn main(app: &mut App) {
	app
		// Declare the game state, whose starting value is determined by the `Default` trait
		.init_state::<GameState>()
		// Adds the plugins for each state
		.add_plugins((splash::splash_plugin, menu::menu_plugin, scenes::game_plugin));
}
