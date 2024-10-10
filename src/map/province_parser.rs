use log::error;
use serde::Deserialize;
use crate::map::components::{ProvinceId, TerrainType, VictoryPoints};

#[derive(Debug, Deserialize)]
pub(crate) struct ProvinceData {
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
    let provinces_data : std::io::Result<ProvincesList> = crate::utils::read_toml::<ProvincesList>(toml_path.clone());
    if let Err(e) = provinces_data {
        error!("Error parsing {}: {}", toml_path.to_str().unwrap_or("provinces.toml"), e);
        panic!();
    }
    let provinces_data = provinces_data.unwrap().provinces;
    provinces_data
}
