use bevy::prelude::*;

mod game_io;
mod simulation;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		// Write out definition templates
		.add_startup_system(simulation::cartridge_list::write_cartridge_definition_template)
		.add_startup_system(simulation::gun_list::write_gun_definition_template)
		.add_startup_system(simulation::ship_list::write_ship_definition_template)
		// Load definitions
		.insert_resource(simulation::cartridge_list::read_cartridge_definitions())
		.insert_resource(simulation::gun_list::read_gun_definitions())
		.insert_resource(simulation::ship_list::read_ship_definitions())
		// Spawning
		.add_startup_system(simulation::camera::setup_camera)
		.add_startup_system(simulation::spawning::spawn_player_ship)
		// Camera
		.add_system(simulation::camera::camera_follow_player)
		.add_system(simulation::camera::camera_zoom_system)
		// Physics
		.add_system(simulation::physics::object_movement_system)
		//Interaction
		.add_system(simulation::physics::projectile_collision_system)
		.add_system(simulation::interaction::kill_system)
		//Spawn turrets and guns
		.add_system(simulation::turret::ship_turret_spawn_system)
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
