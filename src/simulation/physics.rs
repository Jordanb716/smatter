use super::*;
use bevy::sprite::collide_aabb::collide;

pub fn object_movement_system(mut movement_query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
	for (velocity, mut transform) in movement_query.iter_mut() {
		transform.translation += velocity.extend(0.0) * time.delta_seconds();
	}
}

pub fn projectile_collision_system(
	mut commands: Commands,
	projectile_query: Query<(Entity, &Damage, &Transform)>,
	mut target_query: Query<(&mut Health, &Transform), With<Enemy>>,
) {
	for (projectile_entity, damage, projectile_transform) in projectile_query.iter() {
		for (mut health, target_transform) in target_query.iter_mut() {
			let collision = collide(
				target_transform.translation,
				Vec2::new(target_transform.scale.x, target_transform.scale.y),
				projectile_transform.translation,
				Vec2::new(projectile_transform.scale.x, projectile_transform.scale.y),
			);

			if collision.is_some() {
				health.0 -= damage.0;
				commands.entity(projectile_entity).despawn();
			}
		}
	}
}