use bevy::math::Vec3;
use bevy::prelude::{default, Button, Changed, Commands, Query, Sprite, SpriteBundle, Transform, With};
use bevy_color::Color;
use bevy_mod_picking::prelude::{Drag, On, PickingInteraction, Pointer};
use crate::map::components::{Province, ProvinceBundle, TerrainType, VictoryPoints};

pub fn setup_map(mut commands: Commands) {
    commands.spawn((ProvinceBundle::new(VictoryPoints(0), TerrainType::Hills, Vec3::new(0.0, -200.0, 0.0))
                   ));
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