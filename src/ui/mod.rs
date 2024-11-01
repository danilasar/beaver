use bevy::prelude::{App, Component, Resource, Startup};

mod camera;
mod map;
mod province;

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum DisplayQuality {
	Low,
	Medium,
	High,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume(pub u32);

pub fn main(app: &mut App) {
	app
		// Insert as resource the initial value for the settings resources
		.insert_resource(DisplayQuality::Medium)
		.insert_resource(Volume(7))
		.add_systems(Startup, camera::startup::startup)
		.add_plugins(province::Provinces)
		.add_plugins(map::Map);

}