use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

mod simulation;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup)
		.add_startup_system(simulation::spawning::spawn_player_ship)
		.add_system(simulation::physics::object_movement_system)
		.add_system(simulation::physics::projectile_collision_system)
		.add_system(simulation::interaction::kill_system)
		.add_system(simulation::targeting::turret_targeting_system)
		.add_system(simulation::targeting::turret_firing_system)
		.insert_resource(simulation::SpawnTimer(Timer::from_seconds(0.5, true)))
		.add_system(simulation::spawning::target_spawn_system)
		.run();
}

pub fn setup(mut commands: Commands) {
	// Camera
	let mut camera = OrthographicCameraBundle::new_2d();
	camera.orthographic_projection.scaling_mode = ScalingMode::FixedHorizontal;
	camera.orthographic_projection.scale = 1000.0;
	commands.spawn_bundle(camera);
}