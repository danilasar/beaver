use bevy::app::App;
use bevy::prelude::{in_state, on_event, IntoSystemConfigs, OnEnter, Update};
use super::GameState;

pub mod components;
pub mod systems; // todo убрать публичность после выделения событий в отдельный модуль
mod province_parser;
mod resources;

pub struct Map;
impl bevy::prelude::Plugin for Map {
    fn build(&self, app: &mut App) {
        app
            .add_event::<systems::ProvinceHoverEvent>()
            .add_event::<systems::ProvinceUnhoverEvent>()
            .add_event::<systems::ProvinceClickEvent>()
            .add_systems(OnEnter(GameState::Game), systems::setup_provinces)
            //.add_systems(Update, systems::update_province_colors.run_if(in_state(GameState::Game)))
            //.add_systems(Update, systems::province_hover_listener.run_if(in_state(GameState::Game)))
            .add_systems(Update, systems::province_click_listener.run_if(on_event::<systems::ProvinceClickEvent>()))
            .add_systems(Update, systems::province_hover_listener.run_if(on_event::<systems::ProvinceHoverEvent>()))
            .add_systems(Update, systems::province_unhover_listener.run_if(on_event::<systems::ProvinceUnhoverEvent>()));
    }

    fn name(&self) -> &str {
        "World map"
    }

    fn is_unique(&self) -> bool {
       true
    }
}
