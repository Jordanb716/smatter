use super::*;

pub enum IFF {
	Neutral,
	Friendly,
	Enemy,
}

#[derive(Component, Deref, DerefMut)]
pub struct Damage(pub i32);

pub fn kill_system(
	mut commands: Commands,
	enemy: Query<(Entity, &ship::Health), (Changed<ship::Health>, With<ship::Enemy>)>,
) {
	for (entity, health) in enemy.iter() {
		if health.0 <= 0 {
			commands.entity(entity).despawn();
		}
	}
}