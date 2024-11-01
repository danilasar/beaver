use bevy::app::Update;
use bevy::prelude::{on_event, App, IntoSystemConfigs};
use events::{ ProvinceHoverEvent, ProvinceUnhoverEvent, ProvinceClickEvent };
use systems::{province_click_listener, province_hover_listener, province_unhover_listener};

mod components;
mod events;
mod systems;

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
