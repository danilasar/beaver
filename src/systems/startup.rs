use bevy::prelude::{default, AssetServer, Assets, Camera2dBundle, Camera3dBundle, Color, ColorMaterial, Commands, Component, Mesh, PbrBundle, Res, ResMut, Sprite, SpriteBundle, Transform, Vec2, Vec3};
use bevy_render::camera::{Camera, ClearColorConfig};

const PADDLE_SIZE: Vec2 = Vec2::new(20.0, 20.0);
const PADDLE_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);
const BOTTOM_WALL: f32 = -300.;
const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Collider;

pub fn startup_cameras(
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


pub fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
) {
}