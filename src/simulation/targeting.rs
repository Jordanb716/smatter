use super::*;

pub fn turret_targeting_system(
	mut turret: Query<
		(
			&Parent,
			&GlobalTransform,
			&mut Transform,
			&GunVelocity,
			&mut FireTurret,
		),
		(With<Turret>, Without<Enemy>),
	>,
	enemy: Query<(&Transform, &Velocity), With<Enemy>>,
	ship_query: Query<&Velocity>,
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
		gun_velocity,
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

		let target_point =
			target_prediction_first_order(relative_position, relative_velocity, gun_velocity.0);
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
	gun_velocity: f32,
) -> Option<Quat> {
	let dot = Vec2::dot(relative_position, relative_velocity);
	let target_distance = relative_position.length_squared();
	let i_speed2 = gun_velocity.powi(2);
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
	mut commands: Commands,
	mut turret: Query<
		(
			&Parent,
			&GlobalTransform,
			&GunVelocity,
			&FireTurret,
			&mut GunDelayTimer,
			&GunShotsPerSecond,
		),
		With<Turret>,
	>,
	ship_query: Query<&Velocity>,
) {
	for (
		turret_parent,
		turret_transform,
		turret_gun_velocity,
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
				let velocity_deviation_mps = turret_gun_velocity.0 * 0.01;
				let turret_gun_velocity =
					turret_gun_velocity.0 + (rand::random::<f32>() - 0.5) * velocity_deviation_mps;

				commands
					.spawn_bundle(SpriteBundle {
						sprite: Sprite {
							color: (Color::RED),
							..default()
						},
						transform: Transform {
							translation: turret_transform.translation,
							scale: Vec3::new(2.0, 2.0, 0.0),
							..default()
						},
						..default()
					})
					.insert(Projectile)
					.insert(Velocity(
						Vec2::from(
							(-turret_transform.rotation.to_scaled_axis().to_array()[2]
								+ shot_deviation)
								.sin_cos(),
						) * turret_gun_velocity + ship_query.get(turret_parent.0).unwrap().0,
					))
					.insert(Damage(1));
			}
		}
	}
}
