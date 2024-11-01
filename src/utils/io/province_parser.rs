use log::error;
use serde::Deserialize;
use crate::simulation::province::components::province::{ProvinceId, TerrainType, VictoryPoints};
use crate::utils::io::read_toml::read_toml;

#[derive(Debug, Deserialize)]
pub struct ProvinceData {
	pub id: ProvinceId,
	pub x: f32,
	pub y: f32,
	pub terrain: TerrainType,
	pub vp: VictoryPoints,
	pub adjacent: Vec<ProvinceId>
}

#[derive(Debug, Deserialize)]
struct ProvincesList {
	provinces: Vec<ProvinceData>
}

pub fn parse_provinces() -> Vec<ProvinceData> {
	let toml_path = std::path::PathBuf::from(std::path::Path::new("map/provinces.toml"));
	let provinces_data : Vec<ProvinceData> = match read_toml::<ProvincesList>(toml_path.clone()) {
		Ok(provinces_list) => provinces_list.provinces,
		Err(error) => {
			error!("Error parsing {}: {}", toml_path.to_str().unwrap_or("provinces.toml"), error);
			panic!();
		}
	};
	provinces_data
}
