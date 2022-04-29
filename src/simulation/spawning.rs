use super::*;

pub fn spawn_player_ship(mut commands: Commands, asset_server: Res<AssetServer>) {
	// Ship
	commands
		.spawn_bundle(SpriteBundle {
			transform: Transform {
				translation: Vec3::new(0.0, -500.0, 1.0),
				..default()
			},
			sprite: Sprite {
				custom_size: Some(Vec2::new(200.0, 200.0)),
				..default()
			},
			texture: asset_server.load("temp_ship.png"),
			..default()
		})
		.insert(physics::Velocity(Vec2::new(0.0, 40.0)))
		.insert(ship::IsPlayerShip)
		.with_children(|parent| {
			// Turrets
			parent
				.spawn_bundle(SpriteBundle {
					sprite: Sprite { ..default() },
					transform: Transform {
						translation: Vec3::new(-60.0, 21.0, 1.0),
						scale: Vec3::new(1.0, 1.0, 1.0),
						..default()
					},
					texture: asset_server.load("temp_turret.png"),
					..default()
				})
				.insert(turret::Turret{
					iff: interaction::IFF::Friendly,
					..default()
				})
				.insert(gun::ProjectileVelocity(200.0))
				.insert(gun::GunShotsPerSecond(40.0))
				.insert(gun::GunDelayTimer(Timer::from_seconds(0.0, false)));

			parent
				.spawn_bundle(SpriteBundle {
					transform: Transform {
						translation: Vec3::new(60.0, 21.0, 1.0),
						scale: Vec3::new(1.0, 1.0, 1.0),
						..default()
					},
					texture: asset_server.load("temp_turret.png"),
					..default()
				})
				.insert(turret::Turret{
					iff: interaction::IFF::Friendly,
					..default()
				})
				.insert(gun::ProjectileVelocity(200.0))
				.insert(gun::GunShotsPerSecond(40.0))
				.insert(gun::GunDelayTimer(Timer::from_seconds(0.0, false)));
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
