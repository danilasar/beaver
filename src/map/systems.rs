use std::collections::VecDeque;
use bevy::ecs::system::SystemState;
use bevy::math::Vec3;
use bevy::prelude::{Changed, Entity, Event, EntityWorldMut, Query, Sprite, With, World, EventReader, ResMut};
use bevy::utils::tracing::event;
use bevy_color::Color;
use bevy_mod_picking::prelude::{Click, ListenerInput, On, Over, Out, PickingInteraction, Pointer};
use log::info;
use crate::core::components::DefaultColor;
use crate::map::components::{Province, ProvinceBundle};
use crate::map::province_parser::parse_provinces;
use crate::map::resources::ProvincesCollection;

#[derive(Event)]
pub struct ProvinceHoverEvent(Entity);
#[derive(Event)]
pub struct ProvinceUnhoverEvent(Entity);
#[derive(Event)]
pub struct ProvinceClickEvent(Entity);

enum MouseEvent {
	Click,
	Hover,
	Unhover
}
trait EntityEvent {
	fn get_entity(&self) -> Entity;
}

impl From<ListenerInput<Pointer<Over>>> for ProvinceHoverEvent {
	fn from(event: ListenerInput<Pointer<Over>>) -> Self {
		ProvinceHoverEvent(event.target)
	}
}

impl EntityEvent for ProvinceHoverEvent {
	fn get_entity(&self) -> Entity {
		self.0
	}
}

impl From<ListenerInput<Pointer<Out>>> for ProvinceUnhoverEvent {
	fn from(event: ListenerInput<Pointer<Out>>) -> Self {
		ProvinceUnhoverEvent(event.target)
	}
}

impl EntityEvent for ProvinceUnhoverEvent {
	fn get_entity(&self) -> Entity {
		self.0
	}
}

impl From<ListenerInput<Pointer<Click>>> for ProvinceClickEvent {
	fn from(event: ListenerInput<Pointer<Click>>) -> Self {
		ProvinceClickEvent(event.target)
	}
}

impl EntityEvent for ProvinceClickEvent {
	fn get_entity(&self) -> Entity {
		self.0
	}
}

pub fn setup_provinces(world: &mut World) {
	let mut new_provinces_collection = ProvincesCollection::default();
	let provinces_data = parse_provinces();
	for province_data in provinces_data {
		let province = Province {
			id: province_data.id.clone(),
			victory_points: province_data.vp,
			terrain_type: province_data.terrain,
		};
		let entity = world.spawn((
			ProvinceBundle::new(
				province,
				Vec3::new(province_data.x, province_data.y, 0.0)
			),
			On::<Pointer<Over>>::send_event::<ProvinceHoverEvent>(),
			On::<Pointer<Click>>::send_event::<ProvinceClickEvent>(),
			On::<Pointer<Out>>::send_event::<ProvinceUnhoverEvent>()
		)).id();
		new_provinces_collection.list.insert(province_data.id.clone(), entity);
		new_provinces_collection.graph.insert(province_data.id, province_data.adjacent);
	}
	let _ = world.insert_resource::<ProvincesCollection>(new_provinces_collection);
}

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
		let res = res.unwrap();
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
		for entity in adjacent {
			let entity = world.entity_mut(entity);
			paint_entity(interaction, entity);
		}
	}
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
