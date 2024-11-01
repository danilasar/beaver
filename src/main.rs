mod utils;
mod scenes;
mod world;
mod ui;
mod system;
mod simulation;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::fmt::Debug;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;
use scenes::menu;


fn main() {
	App::new()
		.add_plugins((
			DefaultPlugins.set(low_latency_window_plugin()),
			DefaultPickingPlugins,
			ShapePlugin
		))
		.add_plugins(system::main)
		.add_plugins(WorldInspectorPlugin::new())
		.insert_resource(DebugPickingMode::Normal)
		.run();
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
	for entity in &to_despawn {
		commands.entity(entity).despawn_recursive();
	}
}
