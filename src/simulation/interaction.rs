use super::*;

pub fn kill_system(
	mut commands: Commands,
	enemy: Query<(Entity, &Health), (Changed<Health>, With<Enemy>)>,
) {
	for (entity, health) in enemy.iter() {
		if health.0 <= 0 {
			commands.entity(entity).despawn();
		}
	}
}