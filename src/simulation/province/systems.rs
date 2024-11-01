use bevy::math::Vec3;
use bevy::prelude::{default, World};
use bevy_color::palettes::basic::BLACK;
use bevy_color::palettes::css::DARK_CYAN;
use bevy_mod_picking::PickableBundle;
use bevy_prototype_lyon::prelude::{Fill, GeometryBuilder, ShapeBundle, Stroke};
use bevy_prototype_lyon::shapes;
use crate::utils::io::province_parser::parse_provinces;
use province::components::province::Province;
use province::resources::ProvincesCollection;
use crate::simulation::province;

pub fn setup_provinces(world: &mut World) {
	let mut new_provinces_collection = ProvincesCollection::default();
	let provinces_data = parse_provinces();
	for province_data in provinces_data {
		let province = Province {
			id: province_data.id.clone(),
			victory_points: province_data.vp,
			terrain_type: province_data.terrain,
		};
		let entity = world.spawn(province).id();
		new_provinces_collection.list.insert(province_data.id.clone(), entity);
		new_provinces_collection.graph.insert(province_data.id, province_data.adjacent);
	}
	let _ = world.insert_resource::<ProvincesCollection>(new_provinces_collection);
	let shape = shapes::RegularPolygon {
		sides: 6,
		feature: shapes::RegularPolygonFeature::Radius(200.0),
		..shapes::RegularPolygon::default()
	};
	world.spawn((
		ShapeBundle {
			path: GeometryBuilder::build_as(&shape),
			..default()
		},
		Fill::color(DARK_CYAN),
		Stroke::new(BLACK, 10.0),
		PickableBundle::default()
	));
}

