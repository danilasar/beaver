use bevy::app::App;
use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter, Startup, Update};
use super::{despawn_screen, DisplayQuality, GameState, Volume};

pub mod components;
mod systems;
pub struct Map;
impl bevy::prelude::Plugin for Map {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), systems::setup_map)
            .add_systems(Update, systems::setup_map.run_if(in_state(GameState::Game)))
            .add_systems(Update, systems::update_button_colors.run_if(in_state(GameState::Game)));
    }

    fn name(&self) -> &str {
        "World map"
    }

    fn is_unique(&self) -> bool {
       true
    }
}