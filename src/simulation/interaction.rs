use super::*;

#[derive(Component, Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum IFF {
	Neutral,
	Friendly,
	Enemy,
}

impl Default for IFF{
    fn default() -> Self {
        IFF::Neutral
    }
}

#[derive(Component, Deref, DerefMut, Default, Debug)]
pub struct Damage(pub f32);

pub fn kill_system(
	mut commands: Commands,
	enemy: Query<(Entity, &ship::Health), Changed<ship::Health>>,
) {
	for (entity, health) in enemy.iter() {
		if health.0 <= 0.0 {
			commands.entity(entity).despawn();
		}
	}
}
