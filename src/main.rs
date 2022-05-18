use bevy::prelude::*;

mod simulation;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		// Spawning
		.add_startup_system(simulation::camera::setup_camera)
		.add_startup_system(simulation::spawning::spawn_player_ship)
		// Camera
		.add_system(simulation::camera::camera_follow_player)
		.add_system(simulation::camera::camera_zoom_system)
		// Physics
		.add_system(simulation::physics::object_movement_system)
		.add_system(simulation::physics::velocity_calculation_system)
		//Interaction
		.add_system(simulation::physics::projectile_collision_system)
		.add_system(simulation::interaction::kill_system)
		//Spawn turrets and guns
		.add_system(simulation::turret_list::ship_turret_spawn_system)
		.add_system(simulation::gun_list::turret_gun_spawn_system)
		//Turrets and guns
		.add_system(simulation::targeting::turret_target_selection)
		.add_system(simulation::targeting::turret_targeting_system)
		.add_system(simulation::gun::gun_firing_system)
		// More spawning
		.add_system(simulation::spawning::target_spawn_system)
		.insert_resource(simulation::SpawnTimer(Timer::from_seconds(0.5, true)))
		.run();
}
