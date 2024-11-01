use std::collections::VecDeque;
use bevy::ecs::system::SystemState;
use bevy::prelude::{Entity, EntityWorldMut, Event, EventReader, Sprite, World};
use bevy_color::Color;
use bevy_mod_picking::focus::PickingInteraction;
use crate::utils::components::DefaultColor;
use crate::utils::events::EntityEvent;
use crate::simulation::province::components::province::Province;
use crate::simulation::province::resources::ProvincesCollection;
use crate::ui::province::events::{ProvinceClickEvent, ProvinceHoverEvent, ProvinceUnhoverEvent};

pub fn province_mouse_listener<T: Event + EntityEvent>(world: &mut World, picking_interaction: PickingInteraction) { // mut events: EventReader<ProvinceClickEvent>,
	let entities = get_enitites_by_event::<T>(world);
	let mut next_entities: VecDeque<Entity> = VecDeque::new();
	for entity in entities {
		next_entities.push_back(entity);
		// дальше собираем и раскрашиваем соседей
		let entity = world.get_entity(entity);
		if entity.is_none() {
			continue
		}
		let entity = entity.unwrap();
		let province = entity.get::<Province>();
		if province.is_none() {
			continue
		}
		let province = province.unwrap().id.clone();
		let provinces_collection = world.get_resource::<ProvincesCollection>();
		if provinces_collection.is_none() {
			continue
		}
		let provinces_collection = provinces_collection.unwrap();
		let provinces = provinces_collection.graph.get(&province);
		if provinces.is_none() {
			continue
		}
		let provinces = provinces.unwrap();
		for province in provinces {
			let entity = provinces_collection.list.get(province);
			if entity.is_none() {
				continue
			}
			let entity = entity.unwrap().clone();
			next_entities.push_back(entity);
		}
	}
	for entity in next_entities {
		let entity_mut = world.get_entity_mut(entity);
		if entity_mut.is_none() {
			continue
		}
		let entity_mut = entity_mut.unwrap();
		paint_entity(picking_interaction, entity_mut);
	}
}

pub fn province_click_listener(world: &mut World) {
	province_mouse_listener::<ProvinceClickEvent>(world, PickingInteraction::Pressed);
}

pub fn province_hover_listener(world: &mut World) {
	province_mouse_listener::<ProvinceHoverEvent>(world, PickingInteraction::Hovered);
}

pub fn province_unhover_listener(world: &mut World) {
	province_mouse_listener::<ProvinceUnhoverEvent>(world, PickingInteraction::None);
}

pub fn get_enitites_by_event<T: Event + EntityEvent>(world: &mut World) -> VecDeque<Entity> {
	let mut system_state: SystemState<EventReader<T>> = SystemState::new(world);
	let mut events = system_state.get(world);
	let mut entities : VecDeque<Entity> = VecDeque::new();
	for event in events.read() {
		entities.push_back(event.get_entity());
	}
	entities
}

fn paint_entity(interaction: PickingInteraction, mut entity: EntityWorldMut) {
	let default_color = entity.get::<DefaultColor>();
	if default_color.is_none() {
		return;
	}
	let default_color = default_color.unwrap().clone();
	let sprite = entity.get_mut::<Sprite>();
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
