use bevy::prelude::Entity;

enum MouseEvent {
	Click,
	Hover,
	Unhover
}
pub trait EntityEvent {
	fn get_entity(&self) -> Entity;
}
