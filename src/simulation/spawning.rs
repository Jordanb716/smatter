use super::*;

pub fn spawn_player_ship(mut commands: Commands, asset_server: Res<AssetServer>) {
	let spawn_transform = Transform::from_xyz(0.0, -500.0, 0.0);

	let player_ship = ship::ShipBundle::ship_temp(spawn_transform, &asset_server)
		.generate_turret(
			&asset_server,
			0,
			gun_list::GunName::GunMachinegun,
			turret_list::TurretNumBarrels::Single,
		)
		.generate_turret(
			&asset_server,
			1,
			gun_list::GunName::GunMachinegun,
			turret_list::TurretNumBarrels::Single,
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
			.insert(ship::Health(10))
			.insert(physics::Velocity(Vec2::new(
				rand::random::<f32>() * 80.0 - 10.0,
				rand::random::<f32>() * -80.0 - 20.0,
			)));
	}
}
