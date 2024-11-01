use bevy::app::{App, Startup};
use bevy::prelude::OnEnter;
use crate::system::GameState;

pub mod province;

pub fn main(app: &mut App) {
	app
		.add_plugins(province::Provinces)
		// Insert as resource the initial value for the settings resources
		.add_systems(OnEnter(GameState::Game), province::systems::setup_provinces);
}
