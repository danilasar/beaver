use std::collections::VecDeque;
use bevy::ecs::system::SystemState;
use bevy::math::Vec3;
use bevy::prelude::{Changed, Entity, Event, EntityWorldMut, Query, Sprite, With, World, EventReader, ResMut};
use bevy::utils::tracing::event;
use bevy_color::Color;
use bevy_mod_picking::prelude::{Click, ListenerInput, On, Over, Out, PickingInteraction, Pointer};
use log::info;
use crate::core::components::DefaultColor;
use crate::core::events::EntityEvent;

#[derive(Event)]
pub struct ProvinceHoverEvent(Entity);
#[derive(Event)]
pub struct ProvinceUnhoverEvent(Entity);
#[derive(Event)]
pub struct ProvinceClickEvent(Entity);


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
