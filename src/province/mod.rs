mod systems;
pub mod components;
pub mod events;
pub mod resources;

use bevy::app::App;
use bevy::prelude::{in_state, on_event, IntoSystemConfigs, OnEnter, Update};
use crate::province::events::*;
use crate::province::resources::*;
use crate::province::systems::*;
use super::GameState;


pub struct Provinces;
impl bevy::prelude::Plugin for Provinces {
	fn build(&self, app: &mut App) {
		app
			.add_event::<ProvinceHoverEvent>()
			.add_event::<ProvinceUnhoverEvent>()
			.add_event::<ProvinceClickEvent>()
			.add_systems(Update, province_click_listener.run_if(on_event::<ProvinceClickEvent>()))
			.add_systems(Update, province_hover_listener.run_if(on_event::<ProvinceHoverEvent>()))
			.add_systems(Update, province_unhover_listener.run_if(on_event::<ProvinceUnhoverEvent>()));
	}

	fn name(&self) -> &str {
		"Provinces plugin"
	}

	fn is_unique(&self) -> bool {
		true
	}
}
