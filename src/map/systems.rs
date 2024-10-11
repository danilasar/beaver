use std::any::Any;
use std::collections::VecDeque;
use std::ops::Deref;
use bevy::ecs::component::ComponentInfo;
use bevy::ecs::observer::TriggerTargets;
use bevy::ecs::system::SystemState;
use bevy::math::Vec3;
use bevy::prelude::{Changed, Entity, Mut, Query, Sprite, With, World};
use bevy::tasks::futures_lite::stream::iter;
use bevy_color::Color;
use bevy_mod_picking::prelude::PickingInteraction;
use crate::core::components::DefaultColor;
use crate::map::components::{Province, ProvinceBundle, ProvinceId};
use crate::map::province_parser::parse_provinces;
use crate::map::resources::ProvincesCollection;

pub fn setup_provinces(world: &mut World) {
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
pub fn update_province_colors(world: &mut World) {
    // Создаём SystemState для вашего запроса
    let mut system_state: SystemState<Query<(Option<&PickingInteraction>, &Province, &mut Sprite, &crate::core::components::DefaultColor), // TODO: bevy_picking_core::pointer::PointerPress может отслеживать конкретную кнопку мыши
        (With<Province>, Changed<PickingInteraction>)>> = SystemState::new(world);
    // Получаем доступ к данным из world
    let mut provinces = system_state.get_mut(world);
    let mut queue : VecDeque<(Option<PickingInteraction>, ProvinceId, DefaultColor)> = VecDeque::new();
    // Итерируемся по результатам запроса
    for (interaction, province, mut sprite, default_color) in &mut provinces { // (interaction, province, mut sprite, default_color)
        let interaction = match interaction {
            None => continue,
            Some(it) => Some(it.clone())
        };

        let province = province.id.clone();
        println!("{}", province.0.clone());
        let default_color = default_color.clone();
        queue.push_back((interaction, province, default_color));
    }
    if queue.len() == 0 {
        return;
    }

    let mut res = world.get_resource::<ProvincesCollection>();
    if res.is_none() {
        return;
    }
    let mut res = res.unwrap();
    let mut queue2: VecDeque<(Option<PickingInteraction>, ProvinceId, DefaultColor, Entity)> = VecDeque::new();
    for x in queue.pop_front() {
        let entity = res.list.get(&x.1.clone());
        let adjacent = res.graph.get(&x.1.clone());
        if entity.is_none() || adjacent.is_none() {
            continue;
        }
        queue2.push_back((x.0, x.1, x.2, entity.unwrap().clone()));
    }
    for (interaction, province, default_color, entity) in queue2.pop_back() {
        /*sprite.color = match interaction {
            Some(PickingInteraction::Pressed) => Color::srgb(0.35, 0.75, 0.35),
            Some(PickingInteraction::Hovered) => Color::srgb(0.25, 0.25, 0.25),
            Some(PickingInteraction::None) | None => default_color.0
        };*/
        let mut tmp = world.entity_mut(entity.clone());
        let mut sprite = tmp.get_mut::<Sprite>();
        if sprite.is_some() {
            sprite.unwrap().color = match interaction {
                Some(PickingInteraction::Pressed) => Color::srgb(0.35, 0.75, 0.35),
                Some(PickingInteraction::Hovered) => Color::srgb(0.25, 0.25, 0.25),
                Some(PickingInteraction::None) | None => default_color.0
            };
        }
    }
}