use super::*;

pub fn spawn_player_ship(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	ship_definition_list: Res<ship_list::ShipDefinitionList>,
	gun_definition_list: Res<gun_list::GunDefinitionList>,
) {
	let spawn_transform = Transform::from_xyz(0.0, -500.0, 0.0);

	let player_ship = ship_list::spawn_ship(
		"temp_ship",
		spawn_transform,
		&asset_server,
		&ship_definition_list,
	)
	.generate_turret(
		&asset_server,
		0,
		"machinegun",
		turret_list::TurretNumBarrels::Single,
		&gun_definition_list,
	)
	.generate_turret(
		&asset_server,
		1,
		"machinegun",
		turret_list::TurretNumBarrels::Double,
		&gun_definition_list,
	)
	.generate_turret(
		&asset_server,
		2,
		"machinegun",
		turret_list::TurretNumBarrels::Triple,
		&gun_definition_list,
	);

	commands
		.spawn_bundle(player_ship)
		.insert(ship::IsPlayerShip);
}

pub fn target_spawn_system(mut commands: Commands, time: Res<Time>, mut timer: ResMut<SpawnTimer>) {
	if timer.tick(time.delta()).just_finished() {
		commands
			.spawn_bundle(SpriteBundle {
				sprite: Sprite {
					color: (Color::YELLOW),
					..default()
				},
				transform: Transform {
					translation: Vec3::new(rand::random::<f32>() * 600.0 - 300.0, 400.0, 0.0),
					scale: Vec3::new(4.0, 4.0, 0.0),
					..default()
				},
				..default()
			})
			.insert(ship::Enemy)
			.insert(ship::Health(10.0))
			.insert(physics::Velocity(Vec2::new(
				rand::random::<f32>() * 80.0 - 10.0,
				rand::random::<f32>() * -80.0 - 20.0,
			)));
	}
}
