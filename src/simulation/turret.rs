use super::*;

#[derive(Component)]
pub struct TurretProperties {
	pub field_of_view_degreees: f32,
	pub rotation_velocity: Quat,
	pub target_entity: Option<Entity>,
	pub turret_state: TurretState,
}

impl Default for TurretProperties {
	fn default() -> Self {
		Self {
			field_of_view_degreees: 360.0,
			rotation_velocity: Quat::from_rotation_z((10.0_f32).to_radians()),
			target_entity: None,
			turret_state: TurretState::NoTarget,
		}
	}
}

#[derive(Component, PartialEq)]
pub enum TurretState{
	NoTarget,
	Laying,
	Firing,
}

#[derive(Bundle)]
pub struct TurretSingle {
	pub turret_properties: TurretProperties,
	pub gun_properties: gun::GunProperties,

	#[bundle]
	pub sprite: SpriteBundle,
}

impl Default for TurretSingle {
	fn default() -> Self {
		Self {
			turret_properties: TurretProperties::default(),

			gun_properties: gun::GunProperties {
				gun_type: gun::GunType::Kinetic,
				rate_of_fire: default(),
				projectile_velocity_mps: default(),
				gun_cycle_timer: default(),
			},

			sprite: SpriteBundle {
				sprite: Default::default(),
				transform: Default::default(),
				global_transform: Default::default(),
				texture: Default::default(),
				visibility: Default::default(),
			},
		}
	}
}
