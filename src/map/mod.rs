use bevy::app::App;
use bevy::prelude::{in_state, on_event, IntoSystemConfigs, OnEnter, Update};
use super::GameState;

pub mod components;
mod systems;
mod province_parser;
mod resources;

pub struct Map;
impl bevy::prelude::Plugin for Map {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Game), systems::setup_provinces);
    }

    fn name(&self) -> &str {
        "World map"
    }

    fn is_unique(&self) -> bool {
       true
    }
}
