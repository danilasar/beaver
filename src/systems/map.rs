use bevy::math::Vec3;
use bevy::prelude::{default, Commands, Sprite, SpriteBundle, Transform};
use crate::components::province::{ProvinceBundle, Province, TerrainType, VictoryPoints};

pub fn setup_map(mut commands: Commands) {
    commands.spawn(ProvinceBundle::new(VictoryPoints(0), TerrainType::Hills, Vec3::new(0.0, -200.0, 0.0)));
}