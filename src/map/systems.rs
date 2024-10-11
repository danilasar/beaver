use std::any::Any;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use bevy::ecs::component::ComponentInfo;
use bevy::ecs::observer::TriggerTargets;
use bevy::ecs::system::SystemState;
use bevy::math::Vec3;
use bevy::prelude::{Changed, Entity, EntityWorldMut, Mut, Query, Sprite, With, World};
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
        let mut system_state: SystemState<Query<(Option<&PickingInteraction>, &Province, &crate::core::components::DefaultColor), // TODO: bevy_picking_core::pointer::PointerPress может отслеживать конкретную кнопку мыши
            (With<Province>, Changed<PickingInteraction>)>> = SystemState::new(world);
        // Получаем доступ к данным из world
        let provinces = system_state.get(world);
        let mut first_queue: VecDeque<(PickingInteraction, Entity, Vec<Entity>)> = VecDeque::new();
        // Итерируемся по результатам запроса
        for (interaction, province, default_color) in &provinces { // (interaction, province, mut sprite, default_color)
            let interaction = match interaction {
                None => continue,
                Some(it) => it.clone()
            };
            let province = province.id.clone();
            let default_color = default_color.clone();
            let res = world.get_resource::<ProvincesCollection>();
            if res.is_none() {
                return;
            }
            let mut res = res.unwrap();
            let entity = res.list.get(&province.clone());
            let adjacent = res.graph.get(&province.clone());
            if entity.is_none() || adjacent.is_none() {
                continue;
            }
            let entity = entity.unwrap().clone();
            let adjacent_ids = adjacent.unwrap().clone();
            let adjacent = adjacent_ids.iter().map(|i| *res.list.get(&i).unwrap()).collect();

            first_queue.push_back((interaction, entity, adjacent));
        }
    while let Some((interaction, entity, adjacent)) = first_queue.pop_front() {
        let entity = world.entity_mut(entity);
        paint_entity(interaction.clone(), entity);
        /*for entity in adjacent {
            let entity = world.entity_mut(entity);
            paint_entity(interaction, entity);
        }*/
    }
}

fn paint_entity(interaction: PickingInteraction, mut entity: EntityWorldMut) {
    let default_color = entity.get::<DefaultColor>();
    if default_color.is_none() {
        return;
    }
    let default_color = default_color.unwrap().clone();
    let mut sprite = entity.get_mut::<Sprite>();
    if sprite.is_none() {
        return;
    }
    let mut sprite = sprite.unwrap();
    sprite.color = match interaction {
        PickingInteraction::Pressed => Color::srgb(0.35, 0.75, 0.35),
        PickingInteraction::Hovered => Color::srgb(0.25, 0.25, 0.25),
        PickingInteraction::None => default_color.0
    };
}