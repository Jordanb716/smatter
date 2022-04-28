use super::*;

#[derive(Component)]
pub struct IsTurret;

#[derive(Component)]
pub struct Turret {
	pub iff: interaction::IFF,
	pub field_of_view_degreees: f32,
	pub default_rotation: Quat,
	pub rotation_velocity: Quat,
	pub target_entity: Option<Entity>,
	pub is_firing: bool,
}

impl Default for Turret {
	fn default() -> Self {
		Self {
			iff: interaction::IFF::Neutral,
			field_of_view_degreees: 0.0,
			default_rotation: Quat::IDENTITY,
			rotation_velocity: Quat::IDENTITY,
			target_entity: None,
			is_firing: false,
		}
	}
}
