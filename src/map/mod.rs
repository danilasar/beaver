use bevy::app::App;
use bevy::prelude::{Startup, Update};

pub mod components;
mod systems;
pub struct Map;
impl bevy::prelude::Plugin for Map {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::setup_map)
            .add_systems(Update, systems::update_button_colors);
    }

    fn name(&self) -> &str {
        "World map"
    }

    fn is_unique(&self) -> bool {
       true
    }
}