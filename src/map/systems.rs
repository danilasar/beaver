use bevy::math::Vec3;
use bevy::prelude::{Changed, Query, Sprite, With, World};
use bevy_color::Color;
use bevy_mod_picking::prelude::PickingInteraction;
use crate::map::components::{Province, ProvinceBundle};
use crate::map::province_parser::parse_provinces;
use crate::map::resources::ProvincesCollection;

pub fn setup_map(world: &mut World) {
    let mut new_provinces_collection = ProvincesCollection::default();
    let provinces_data = parse_provinces();
    for province_data in provinces_data {
        let province = Province {
            id: province_data.id.clone(),
            victory_points: province_data.vp,
            terrain_type: province_data.terrain,
        };
        let entity = world.spawn(
            ProvinceBundle::new(
                province,
                Vec3::new(province_data.x, province_data.y, 0.0)
            )
        ).id();
        new_provinces_collection.list.insert(province_data.id.clone(), entity);
        new_provinces_collection.graph.insert(province_data.id, province_data.adjacent);
    }
    let _ = world.insert_resource::<ProvincesCollection>(new_provinces_collection);
}

#[allow(clippy::type_complexity)]
/// Use the [`PickingInteraction`] state of each button to update its color.
pub fn update_button_colors(
    mut provinces: Query<
        (Option<&PickingInteraction>, &mut Sprite, &crate::core::components::DefaultColor), // TODO: bevy_picking_core::pointer::PointerPress может отслеживать конкретную кнопку мыши
        (With<Province>, Changed<PickingInteraction>),
    >,
) {
    for (interaction, mut sprite, default_color) in &mut provinces {
        sprite.color = match interaction {
            Some(PickingInteraction::Pressed) => Color::srgb(0.35, 0.75, 0.35),
            Some(PickingInteraction::Hovered) => Color::srgb(0.25, 0.25, 0.25),
            Some(PickingInteraction::None) | None => default_color.0
        };
    }
}