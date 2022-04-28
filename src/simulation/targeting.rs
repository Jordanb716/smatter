use super::*;

pub fn turret_target_selection(
	mut turrets: Query<(&mut turret::Turret, &GlobalTransform)>,
	enemies: Query<(Entity, &Transform, &physics::Velocity), With<ship::Enemy>>,
) {
	if enemies.is_empty() {
		for (mut turret, _) in turrets.iter_mut() {
			turret.is_firing = false;
		}
		return;
	} else {
		for (mut turret_properties, turret_global_transform) in turrets.iter_mut()
		{
			let mut target_candidate_entity = None::<Entity>;
			let mut target_candidate_range = 0.0;
			//Find best possible target
			for (enemy, enemy_transform, enemy_velocity) in enemies.iter() {
				let target_relative_position =
					(enemy_transform.translation - turret_global_transform.translation).truncate();
				let target_range = target_relative_position.length();
				// Check if target is in range
				// Let target degrees off current aim
				// Check if target can be aimed at rotationally

				if target_candidate_entity.is_none() || target_range < target_candidate_range {
					target_candidate_entity = Some(enemy);
					target_candidate_range = target_range;
				}
			}
			// Set turret's target to best candidate.
			turret_properties.target_entity = target_candidate_entity;
		}
	}
}

pub fn turret_targeting_system(
	mut turrets: Query<(
		&Parent,
		&GlobalTransform,
		&mut Transform,
		&mut turret::Turret,
		&gun::ProjectileVelocity,
	)>,
	enemies: Query<(Entity, &Transform, &physics::Velocity), (With<ship::Enemy>, Without<turret::Turret>)>,
	ship_query: Query<&physics::Velocity>,
) {
	for (
		turret_parent,
		turret_global_transform,
		mut turret_transform,
		mut turret_properties,
		projectile_velocity,
	) in turrets.iter_mut()
	{
		let turret_target = turret_properties.target_entity;
		if turret_target.is_none() {
			continue; //Turret has no target.
		} else {
			let turret_target = enemies.get(turret_target.unwrap());
			if turret_target.is_err() {
				continue; // Something Broke.
			}
			// Find turret's current target.
			let (_, target_transform, target_velocity) = turret_target.unwrap();

			let relative_position =
				(target_transform.translation - turret_global_transform.translation).truncate();

			let ship_velocity = ship_query.get(turret_parent.0).unwrap();
			let relative_velocity = target_velocity.0 - ship_velocity.0;

			let target_point = target_prediction_first_order(
				relative_position,
				relative_velocity,
				projectile_velocity.0,
			);
			if target_point.is_some() {
				turret_transform.rotation = target_point.unwrap();
				turret_properties.is_firing = true;
			} else {
				turret_properties.is_firing = false;
			}
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
	mut turret: Query<(
		&Parent,
		&GlobalTransform,
		&gun::ProjectileVelocity,
		&turret::Turret,
		&mut gun::GunDelayTimer,
		&gun::GunShotsPerSecond,
	)>,
	ship_query: Query<&physics::Velocity>,
) {
	for (
		turret_parent,
		turret_transform,
		turret_projectile_velocity,
		turret_properties,
		mut turret_gun_delay_timer,
		turret_shots_per_second,
	) in turret.iter_mut()
	{
		turret_gun_delay_timer.tick(time.delta());

		if turret_properties.is_firing == true {
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
						) * turret_projectile_velocity
							+ gun_velocity.0,
					))
					.insert(interaction::Damage(1));
				audio.play(asset_server.load("temp_gun_fire.ogg"));
			}
		}
	}
}
