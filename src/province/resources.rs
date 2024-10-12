use std::collections::HashMap;
use bevy::prelude::{Entity, Resource};
use crate::province::components::ProvinceId;

#[derive(Debug, Default, Resource)]
pub struct ProvincesCollection {
	pub list: HashMap<ProvinceId, Entity>,
	pub graph: HashMap<ProvinceId, Vec<ProvinceId>>
}
