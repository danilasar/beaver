use bevy::app::App;
use bevy::prelude::OnEnter;

pub mod components;
mod resources;

pub struct Map;
impl bevy::prelude::Plugin for Map {
	fn build(&self, app: &mut App) {
		app;
	}

	fn name(&self) -> &str {
		"World map"
	}

	fn is_unique(&self) -> bool {
		true
	}
}
