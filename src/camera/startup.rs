use bevy::prelude::{default, Camera2dBundle, Camera3dBundle, Commands, Transform, Vec3};
use bevy_render::camera::{Camera, ClearColorConfig};
pub fn startup(
	mut commands: Commands
) {
	commands.spawn(Camera2dBundle {
		camera: Camera {
			order: 1,
			clear_color: ClearColorConfig::None,
			..default()
		},
		..default()
	});
	commands.spawn((Camera3dBundle {
		transform: Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	},));
}