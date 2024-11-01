pub mod components;
pub mod resources;
pub mod systems;

use bevy::app::App;
use bevy::prelude::{on_event, IntoSystemConfigs, Update};


pub struct Provinces;
impl bevy::prelude::Plugin for Provinces {
	fn build(&self, app: &mut App) {
		app;
	}

	fn name(&self) -> &str {
		"Provinces plugin"
	}

	fn is_unique(&self) -> bool {
		true
	}
}
