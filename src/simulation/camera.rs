use bevy::render::camera::*;

use super::*;

#[derive(Component)]
pub struct IsCamera;

pub fn setup_camera(mut commands: Commands) {
	// Camera
	let mut camera = OrthographicCameraBundle::new_2d();
	camera.orthographic_projection.scaling_mode = ScalingMode::FixedHorizontal;
	camera.orthographic_projection.scale = 1000.0;

	commands.spawn_bundle(camera).insert(IsCamera);
}

pub fn camera_follow_player(
	mut camera: Query<&mut Transform, With<IsCamera>>,
	player_ship: Query<&Transform, (With<ship::IsPlayerShip>, Without<IsCamera>)>,
) {
	let camera_target = player_ship.single().translation;
	let mut camera = camera
		.get_single_mut()
		.expect("Camera follow broke on getting the camera");
	camera.translation.x = camera_target.x;
	camera.translation.y = camera_target.y;
}

/// Camera Zoom System
/// Lets the mouse scrool wheel control camera zoom
pub fn camera_zoom_system(
	mut mouse_wheel_events: EventReader<bevy::input::mouse::MouseWheel>,
	mut camera: Query<&mut OrthographicProjection, With<IsCamera>>,
) {
	const CAMERA_ZOOM_SENSITIVITY: f32 = 2.0;
	const CAMERA_SCALE_MAX: f32 = 2000.0; // Zoomed all the way out
	const CAMERA_SCALE_MIN: f32 = 200.0; // Zoomed all the way in

	for mouse_wheel_event in mouse_wheel_events.iter() {
		let mut camera = camera
			.get_single_mut()
			.expect("Zoom broke on getting the camera");
		let dy = match mouse_wheel_event.unit {
			bevy::input::mouse::MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
			bevy::input::mouse::MouseScrollUnit::Pixel => mouse_wheel_event.y,
		};
		camera.scale -= dy * CAMERA_ZOOM_SENSITIVITY;
		camera.scale = camera.scale.clamp(CAMERA_SCALE_MIN, CAMERA_SCALE_MAX);
	}
}
