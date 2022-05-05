use super::*;

//#[derive(Component)]
//pub enum MissileTargetingPhase{}
// Active/semi active/LOBL/LOAL?
// Relock?

pub fn turret_target_selection(
	mut turrets: Query<(&mut turret::TurretProperties, &GlobalTransform)>,
	enemies: Query<(Entity, &Transform, &physics::Velocity), With<ship::Enemy>>,
) {
	if enemies.is_empty() {
		for (mut turret, _) in turrets.iter_mut() {
			turret.turret_state = turret::TurretState::NoTarget;
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
		&mut turret::TurretProperties,
		&gun::GunProperties,
	)>,
	enemies: Query<(Entity, &Transform, &physics::Velocity), (With<ship::Enemy>, Without<turret::TurretProperties>)>,
	ship_query: Query<&physics::Velocity>,
) {
	for (
		turret_parent,
		turret_global_transform,
		mut turret_transform,
		mut turret_properties,
		gun_properties,
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
				gun_properties.projectile_velocity_mps,
			);
			if target_point.is_some() {
				turret_transform.rotation = target_point.unwrap();
				turret_properties.turret_state = turret::TurretState::Firing;
			} else {
				turret_properties.turret_state = turret::TurretState::NoTarget;
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
