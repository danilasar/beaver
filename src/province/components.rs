use bevy::math::Vec2;
use bevy::prelude::{default, Bundle, Component, Sprite, SpriteBundle, Transform, Vec3};
use bevy_color::Color;
use bevy_mod_picking::prelude::{Click, On, Out, Over, Pointer};
use serde::Deserialize;
use crate::province::events::{ProvinceClickEvent, ProvinceHoverEvent, ProvinceUnhoverEvent};

const PROVINCE_SIZE: Vec2 = Vec2::new(20.0, 20.0);
const PROVINCE_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);

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

#[derive(Bundle)]
pub struct ProvinceBundle {
	pub province: Province,
	pub sprite: SpriteBundle,
	pub default_color: crate::core::components::DefaultColor,
	hover_event: On::<Pointer<Over>>,
	click_event: On::<Pointer<Click>>,
	out_event: On::<Pointer<Out>>
}

impl ProvinceBundle {
	pub fn new(province: Province,
			   translation: Vec3
	) -> Self {
		Self {
			province,
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
			},
			default_color: crate::core::components::DefaultColor(PROVINCE_COLOR),
			hover_event: On::<Pointer<Over>>::send_event::<ProvinceHoverEvent>(),
			click_event: On::<Pointer<Click>>::send_event::<ProvinceClickEvent>(),
			out_event:   On::<Pointer<Out>>::send_event::<ProvinceUnhoverEvent>()
		}
	}
}