use std::ops::Sub;

use super::{interaction::IFF, *};
use bevy::sprite::collide_aabb::collide;

#[derive(Component, Deref, DerefMut, Default, Debug)]
pub struct Velocity(pub Vec2);

impl Sub for Velocity {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self { 0: self.0 - rhs.0 }
	}
}

#[derive(Component, Deref, DerefMut, Default, Debug)]
pub struct Acceleration(pub Vec2);

impl Sub for Acceleration {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self { 0: self.0 - rhs.0 }
	}
}

#[derive(Component, Deref, DerefMut)]
pub struct VelocityRotational(pub Quat);

// ==========
// SYSTEMS

/// Updates the position of entities with a Velocity and a Transform
pub fn object_movement_system(
	mut movement_query: Query<(&Velocity, &mut Transform)>,
	time: Res<Time>,
) {
	for (velocity, mut transform) in movement_query.iter_mut() {
		transform.translation += velocity.extend(0.0) * time.delta_seconds();
	}
}

pub fn projectile_collision_system(
	mut commands: Commands,
	projectile_query: Query<(Entity, &interaction::Damage, &Transform, &IFF)>,
	mut target_query: Query<(&mut ship::Health, &Transform, &IFF)>,
) {
	for (projectile_entity, damage, projectile_transform, projectile_iff) in projectile_query.iter()
	{
		for (mut target_health, target_transform, target_iff) in target_query.iter_mut() {
			// Skip collision checks for friendly targets
			if projectile_iff == target_iff {
				continue;
			}

			let collision = collide(
				target_transform.translation,
				Vec2::new(target_transform.scale.x, target_transform.scale.y),
				projectile_transform.translation,
				Vec2::new(projectile_transform.scale.x, projectile_transform.scale.y),
			);

			if collision.is_some() {
				target_health.0 -= damage.0;
				commands.entity(projectile_entity).despawn();
			}
		}
	}
}
