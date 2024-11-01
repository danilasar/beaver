use std::collections::HashMap;
use bevy::prelude::{Entity, Resource};
use crate::simulation::province::components::province::ProvinceId;

#[derive(Debug, Default, Resource)]
pub struct ProvincesCollection {
	pub list: HashMap<ProvinceId, Entity>,
	pub graph: HashMap<ProvinceId, Vec<ProvinceId>>
}
