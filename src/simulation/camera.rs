use bevy::render::camera::*;

use super::*;

#[derive(Component)]
pub struct IsCamera;

pub fn setup_camera(mut commands: Commands) {
	// Camera
	let mut camera = OrthographicCameraBundle::new_2d();
	camera.orthographic_projection.scaling_mode = ScalingMode::FixedHorizontal;
	camera.orthographic_projection.scale = 2000.0;

	commands.spawn_bundle(camera).insert(IsCamera);
}

pub fn camera_follow_player(
	mut camera: Query<&mut Transform, With<IsCamera>>,
	player_ship: Query<&Transform, (With<ship::IsPlayerShip>, Without<IsCamera>)>,
) {
	let camera_target = player_ship.single().translation;
	camera.get_single_mut().unwrap().translation = camera_target;
}
