use bevy::prelude::Entity;

enum MouseEvent {
	Click,
	Hover,
	Unhover
}
pub(crate) trait EntityEvent {
	fn get_entity(&self) -> Entity;
}
