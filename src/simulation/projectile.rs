use super::*;

/// Marks entity as a projectile
#[derive(Component, Clone, Default, Debug)]
pub struct IsProjectile;

#[derive(Component, Clone, Serialize, Deserialize, Debug)]
pub enum ProjectileGuidance {
	None,
	ProportionalNavigation,
}

impl Default for ProjectileGuidance {
	fn default() -> Self {
		ProjectileGuidance::None
	}
}

#[derive(Bundle, Default, Debug)]
pub struct ProjectileBundle {
	pub is_projectile: projectile::IsProjectile,
	pub guidance: ProjectileGuidance,
	pub damage: interaction::Damage,
	pub iff: interaction::IFF,

	pub transform: Transform,
	pub global_transform: GlobalTransform,
	pub velocity: physics::Velocity,
	pub acceleration: physics::Acceleration,

	pub sprite: Sprite,
	pub texture: Handle<Image>,
	pub visibility: Visibility,
}
