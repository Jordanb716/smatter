use super::*;

pub fn spawn_player_ship(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands
		.spawn_bundle(ship::ShipBundle {
			health: ship::Health(10),
			transform: Transform {
				translation: Vec3::new(0.0, -500.0, 0.0),
				..default()
			},
			velocity: physics::Velocity(Vec2::new(0.0, 40.0)),
			iff: interaction::IFF::Friendly,
			texture: asset_server.load("temp_ship.png"),
			..default()
		})
		.insert(ship::IsPlayerShip)
		// Turrets
		.with_children(|parent| {
			parent
				.spawn_bundle(turret::TurretSingle {
					turret_properties: turret::TurretProperties {
						field_of_view_degreees: 270.0,
						..default()
					},
					sprite: SpriteBundle {
						transform: Transform {
							translation: Vec3::new(-60.0, 21.0, 25.0),
							..default()
						},
						texture: asset_server.load("temp_turret.png"),
						..default()
					},
					gun_properties: gun::GunProperties {
						gun_type: gun::GunType::Kinetic,
						rate_of_fire: 10.0,
						projectile_velocity_mps: 400.0,
						..default()
					},
				})
				.with_children(|parent| {
					parent.spawn_bundle(gun::GunBundle { sprite: default() });
				});

			parent
				.spawn_bundle(turret::TurretSingle {
					turret_properties: turret::TurretProperties {
						field_of_view_degreees: 270.0,
						..default()
					},
					sprite: SpriteBundle {
						transform: Transform {
							translation: Vec3::new(60.0, 21.0, 25.0),
							..default()
						},
						texture: asset_server.load("temp_turret.png"),
						..default()
					},
					gun_properties: gun::GunProperties {
						gun_type: gun::GunType::Kinetic,
						rate_of_fire: 10.0,
						projectile_velocity_mps: 400.0,
						..default()
					},
				})
				.with_children(|parent| {
					parent.spawn_bundle(gun::GunBundle { sprite: default() });
				});
		});
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
