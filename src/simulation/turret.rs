use super::*;

#[derive(Component)]
pub struct IsTurret;

/// Hold the turret mount number the turret is assigned to
#[derive(Component, Clone, Deref, DerefMut, Debug)]
pub struct TurretMountNumber(pub usize);

#[derive(Component, Clone, Copy, Debug)]
pub struct TurretProperties {
	pub field_of_view_degrees: f32,
	pub rotation_velocity: Quat,
	pub target_entity: Option<Entity>,
	pub turret_state: TurretState,
}

impl Default for TurretProperties {
	fn default() -> Self {
		Self {
			field_of_view_degrees: 360.0,
			rotation_velocity: Quat::from_rotation_z((10.0_f32).to_radians()),
			target_entity: None,
			turret_state: TurretState::NoTarget,
		}
	}
}
#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub enum TurretState {
	NoTarget,
	//	Laying,
	Firing,
}

#[derive(Bundle, Clone, Debug)]
pub struct TurretBundle {
	pub turret_size: ItemSize,
	pub turret_mount_number: TurretMountNumber,
	pub turret_properties: TurretProperties,

	pub transform: Transform,
	pub global_transform: GlobalTransform,

	/// Name of gun assigned to the turret
	pub gun_properties: gun::GunProperties,

	/// List of child gun components
	pub gun_list: gun_list::TurretGunAssignmentList,

	pub sprite: Sprite,
	pub texture: Handle<Image>,
	pub visibility: Visibility,
}

impl Default for TurretBundle {
	fn default() -> Self {
		Self {
			turret_size: ItemSize::Small,
			turret_properties: TurretProperties::default(),
			gun_properties: gun::GunProperties::default(),
			gun_list: default(),
			turret_mount_number: TurretMountNumber(0),
			transform: default(),
			global_transform: default(),
			sprite: default(),
			texture: default(),
			visibility: default(),
		}
	}
}
