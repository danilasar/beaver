use std::collections::HashMap;
use bevy::prelude::{Entity, Resource};
use crate::map::components::ProvinceId;
use crate::map::province_parser;

#[derive(Debug, Default, Resource)]
pub struct ParsedProvincesFile(Vec<province_parser::ProvinceData>);

#[derive(Debug, Default, Resource)]
pub struct ProvincesCollection {
    pub list: HashMap<ProvinceId, Entity>,
    pub graph: HashMap<ProvinceId, Vec<ProvinceId>>
}