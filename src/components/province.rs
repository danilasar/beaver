use bevy::math::Vec2;
use bevy::prelude::{default, Bundle, Component, Sprite, SpriteBundle, Transform, Vec3};
use bevy_color::Color;

const PROVINCE_SIZE: Vec2 = Vec2::new(20.0, 20.0);
const PROVINCE_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);

#[derive(Component)]
pub struct Province;

#[derive(Component)]
pub struct VictoryPoints(pub i8);

// https://hoi4.paradoxwikis.com/Terrain
#[derive(Component)]
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

#[derive(Bundle)]
pub struct ProvinceBundle {
    pub province: Province,
    pub victory_points: VictoryPoints,
    pub terrain_type: TerrainType,
    sprite: SpriteBundle,
}

impl ProvinceBundle {
    pub fn new(victory_points: VictoryPoints,
               terrain_type: TerrainType,
               translation: Vec3
    ) -> Self {
        Self {
            province: Province,
            victory_points,
            terrain_type,
            sprite: SpriteBundle {
                transform: Transform {
                    translation,
                    scale: PROVINCE_SIZE.extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: PROVINCE_COLOR,
                    ..default()
                },
                ..default()
            }
        }
    }
}