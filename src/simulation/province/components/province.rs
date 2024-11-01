use bevy::math::Vec2;
use bevy::prelude::{default, Bundle, Component, Sprite, SpriteBundle, Transform, Vec3};
use bevy_color::Color;
use bevy_mod_picking::prelude::{Click, On, Out, Over, Pointer};
use serde::Deserialize;

#[derive(Component)]
pub struct Province {
	pub id: ProvinceId,
	pub victory_points: VictoryPoints,
	pub terrain_type: TerrainType
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
pub struct ProvinceId(pub u64);

#[derive(Debug, Deserialize)]
pub struct VictoryPoints(pub i8);

// https://hoi4.paradoxwikis.com/Terrain
#[derive(Debug, Deserialize)]
pub enum TerrainType {
	Desert,
	Forest,
	Hills,
	Jungle,
	Marsh,
	Mountain,
	Plains,
	Urban,
	Lake
}

// здесь также должны быть столица, строения, погода

