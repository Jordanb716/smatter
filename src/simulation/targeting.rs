use super::*;

pub fn turret_targeting_system(
	mut turret: Query<
		(
			&Parent,
			&GlobalTransform,
			&mut Transform,
			&gun::ProjectileVelocity,
			&mut turret::FireTurret,
		),
		(With<turret::IsTurret>, Without<ship::Enemy>),
	>,
	enemy: Query<(&Transform, &physics::Velocity), With<ship::Enemy>>,
	ship_query: Query<&physics::Velocity>,
) {
	if enemy.is_empty() {
		for (_, _, _, _, mut turret_fire_turret) in turret.iter_mut() {
			turret_fire_turret.0 = false;
		}
		return;
	}

	for (
		turret_parent,
		turret_global_transform,
		mut turret_transform,
		projectile_velocity,
		mut turret_fire_turret,
	) in turret.iter_mut()
	{
		let mut target = enemy.iter().next().unwrap();
		let mut relative_position =
			(target.0.translation - turret_global_transform.translation).truncate();

		//Find closest possible target
		for enemy in enemy.iter() {
			relative_position =
				(target.0.translation - turret_global_transform.translation).truncate();
			let relative_enemy_position =
				(enemy.0.translation - turret_global_transform.translation).truncate();
			if relative_enemy_position.length() < relative_position.length() {
				target = enemy;
			}
		}

		let relative_velocity = (target.1).0 - ship_query.get(turret_parent.0).unwrap().0;

		let target_point = target_prediction_first_order(
			relative_position,
			relative_velocity,
			projectile_velocity.0,
		);
		if target_point.is_some() {
			turret_transform.rotation = target_point.unwrap();
			turret_fire_turret.0 = true;
		} else {
			turret_fire_turret.0 = false;
		}
	}
}

pub fn target_prediction_first_order(
	relative_position: Vec2,
	relative_velocity: Vec2,
	projectile_velocity: f32,
) -> Option<Quat> {
	let dot = Vec2::dot(relative_position, relative_velocity);
	let target_distance = relative_position.length_squared();
	let i_speed2 = projectile_velocity.powi(2);
	let target_speed = relative_velocity.length_squared();
	let sqrt = ((dot * dot) - target_distance * (target_speed - i_speed2)).sqrt();

	let whatever_the_hell_this_is = (
		(-dot - sqrt) / target_distance,
		(-dot + sqrt) / target_distance,
	);

	if whatever_the_hell_this_is.0 > 0.0 {
		return Some(Quat::from_rotation_z(
			-((whatever_the_hell_this_is.0 * relative_position + relative_velocity).x
				/ (whatever_the_hell_this_is.0 * relative_position + relative_velocity).y)
				.atan(),
		));
	} else if whatever_the_hell_this_is.1 > 0.0 {
		return Some(Quat::from_rotation_z(
			-((whatever_the_hell_this_is.1 * relative_position + relative_velocity).x
				/ (whatever_the_hell_this_is.1 * relative_position + relative_velocity).y)
				.atan(),
		));
	} else {
		return None;
	}
}

pub fn turret_firing_system(
	time: Res<Time>,
	asset_server: Res<AssetServer>,
	audio: Res<Audio>,
	mut commands: Commands,
	mut turret: Query<
		(
			&Parent,
			&GlobalTransform,
			&gun::ProjectileVelocity,
			&turret::FireTurret,
			&mut gun::GunDelayTimer,
			&gun::GunShotsPerSecond,
		),
		With<turret::IsTurret>,
	>,
	ship_query: Query<&physics::Velocity>,
) {
	for (
		turret_parent,
		turret_transform,
		turret_projectile_velocity,
		turret_fire_turret,
		mut turret_gun_delay_timer,
		turret_shots_per_second,
	) in turret.iter_mut()
	{
		turret_gun_delay_timer.tick(time.delta());

		if turret_fire_turret.0 == true {
			if turret_gun_delay_timer.0.finished() {
				// Set timer for RoF delay.
				turret_gun_delay_timer.0 =
					Timer::from_seconds(1.0 / turret_shots_per_second.0, false);

				// Calculate random spread
				let bullet_spread_degrees = 4.0;
				let shot_deviation = (((rand::random::<f32>() + rand::random::<f32>()) / 2.0
					- 0.5) * bullet_spread_degrees)
					.to_radians();

				// Add deviation to projectile velocity
				let velocity_deviation_mps = turret_projectile_velocity.0 * 0.01;
				let turret_projectile_velocity = turret_projectile_velocity.0
					+ (rand::random::<f32>() - 0.5) * velocity_deviation_mps;
				let gun_velocity = physics::Velocity(ship_query.get(turret_parent.0).unwrap().0);

				commands
					.spawn_bundle(SpriteBundle {
						sprite: Sprite {
							color: (Color::RED),
							..default()
						},
						transform: Transform {
							translation: turret_transform.translation + Vec3::new(0.0, 0.0, -1.0),
							scale: Vec3::new(1.0, 1.0, 1.0),
							..default()
						},
						..default()
					})
					.insert(gun::IsProjectile)
					.insert(physics::Velocity(
						Vec2::from(
							(-turret_transform.rotation.to_scaled_axis().to_array()[2]
								+ shot_deviation)
								.sin_cos(),
						) * turret_projectile_velocity + gun_velocity.0,
					))
					.insert(interaction::Damage(1));
				audio.play(asset_server.load("temp_gun_fire.ogg"));
			}
		}
	}
}
