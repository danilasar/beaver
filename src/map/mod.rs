use bevy::app::App;
use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter, Update};
use super::GameState;

pub mod components;
mod systems;
mod province_parser;
mod resources;

pub struct Map;
impl bevy::prelude::Plugin for Map {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Game), systems::setup_provinces)
            .add_systems(Update, systems::update_province_colors.run_if(in_state(GameState::Game)));
    }

    fn name(&self) -> &str {
        "World map"
    }

    fn is_unique(&self) -> bool {
       true
    }
}