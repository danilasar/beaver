use bevy::math::{Vec2, Vec3};
use bevy::prelude::{default, Sprite, SpriteBundle, Transform};
use bevy_color::Color;
use bevy_mod_picking::events::{Click, Out, Over, Pointer};
use bevy_mod_picking::prelude::On;
use crate::simulation::province::components::province::ProvinceId;
use crate::ui::province::events::{ProvinceClickEvent, ProvinceHoverEvent, ProvinceUnhoverEvent};

const PROVINCE_SIZE: Vec2 = Vec2::new(20.0, 20.0);
const PROVINCE_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);

pub struct ProvinceSprite {
	pub province_id: ProvinceId
}

pub struct ProvinceSpriteBundle {
	pub province_sprite: ProvinceSprite,
	pub sprite: SpriteBundle,
	pub default_color: crate::utils::components::DefaultColor,
	hover_event: On::<Pointer<Over>>,
	click_event: On::<Pointer<Click>>,
	out_event: On::<Pointer<Out>>
}

impl ProvinceSpriteBundle {
	pub fn new(province_id: ProvinceId,
			   translation: Vec3
	) -> Self {
		Self {
			province_sprite: ProvinceSprite { province_id },
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
			default_color: crate::utils::components::DefaultColor(PROVINCE_COLOR),
			hover_event: On::<Pointer<Over>>::send_event::<ProvinceHoverEvent>(),
			click_event: On::<Pointer<Click>>::send_event::<ProvinceClickEvent>(),
			out_event:   On::<Pointer<Out>>::send_event::<ProvinceUnhoverEvent>()
		}
	}
}
