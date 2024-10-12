use bevy::math::Vec3;
use bevy::prelude::World;
use crate::map::province_parser::parse_provinces;
use crate::province::components::{Province, ProvinceBundle};
use crate::province::resources::ProvincesCollection;

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

		)).id();
		new_provinces_collection.list.insert(province_data.id.clone(), entity);
		new_provinces_collection.graph.insert(province_data.id, province_data.adjacent);
	}
	let _ = world.insert_resource::<ProvincesCollection>(new_provinces_collection);
}

