use bevy::prelude::*;
use super::{despawn_screen, GameState};

// This plugin will contain the game. In this case, it's just be a screen that will
// display the current settings for 5 seconds before returning to the menu
pub fn game_plugin(app: &mut App) {
	app.add_plugins(crate::province::Provinces)
		.add_plugins(crate::map::Map)
		.add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;
