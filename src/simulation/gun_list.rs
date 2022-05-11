use super::*;

// List of gun names
#[derive(Clone, Copy, Debug)]
pub enum GunName {
	GunMachinegun,
}

#[derive(Clone, Debug)]
pub struct TurretGunAssignment {
	pub transform: Transform,
	pub texture: Handle<Image>,
}

impl Default for TurretGunAssignment {
	fn default() -> Self {
		Self {
			transform: default(),
			texture: default(),
		}
	}
}

/// List of guns on a turret
#[derive(Component, Clone, Default, Deref, DerefMut, Debug)]
pub struct TurretGunAssignmentList(pub Vec<TurretGunAssignment>);

// Gun definitions

impl gun::GunProperties {
	pub fn gun_machinegun() -> Self {
		gun::GunProperties {
			gun_size: ItemSize::Small,
			gun_type: gun::GunType::Kinetic,
			rate_of_fire: 10.0,
			projectile_velocity_mps: 400.0,
			gun_cycle_timer: None,
		}
	}
}

/// Generates a Gun List (placing guns relative to the parent turret)
pub fn generate_gun_list(
	asset_server: &Res<AssetServer>,
	gun_name: gun_list::GunName,
	turret_size: ItemSize,
	turret_num_barrels: turret_list::TurretNumBarrels,
) -> gun_list::TurretGunAssignmentList {
	// List of default gun transforms based on turret size and number of guns
	const MAX_GUNS: usize = 3;
	// Single turrets
	const DEFAULT_SMALL_SINGLE: [Option<[f32; 3]>; MAX_GUNS] = [
		Some([0.0, 1.0, -1.0]), //
		None,                   //
		None,
	];
	const DEFAULT_MEDIUM_SINGLE: [Option<[f32; 3]>; MAX_GUNS] = [
		Some([0.0, 1.0, -1.0]), //
		None,                   //
		None,
	];
	const DEFAULT_LARGE_SINGLE: [Option<[f32; 3]>; MAX_GUNS] = [
		Some([0.0, 1.0, -1.0]), //
		None,                   //
		None,
	];
	// Double turrets
	const DEFAULT_SMALL_DOUBLE: [Option<[f32; 3]>; MAX_GUNS] = [
		Some([-0.5, 1.0, -1.0]), //
		Some([0.5, 1.0, -1.0]),  //
		None,
	];
	const DEFAULT_MEDIUM_DOUBLE: [Option<[f32; 3]>; MAX_GUNS] = [
		Some([-0.5, 1.0, -1.0]), //
		Some([0.5, 1.0, -1.0]),  //
		None,
	];
	const DEFAULT_LARGE_DOUBLE: [Option<[f32; 3]>; MAX_GUNS] = [
		Some([-0.5, 1.0, -1.0]), //
		Some([0.5, 1.0, -1.0]),  //
		None,
	];
	// Triple turrets
	const DEFAULT_SMALL_TRIPLE: [Option<[f32; 3]>; MAX_GUNS] = [
		Some([-0.8, 1.0, -1.0]),
		Some([0.0, 1.0, -1.0]),
		Some([0.8, 1.0, -1.0]),
	];
	const DEFAULT_MEDIUM_TRIPLE: [Option<[f32; 3]>; MAX_GUNS] = [
		Some([-0.8, 1.0, -1.0]),
		Some([0.0, 1.0, -1.0]),
		Some([0.8, 1.0, -1.0]),
	];
	const DEFAULT_LARGE_TRIPLE: [Option<[f32; 3]>; MAX_GUNS] = [
		Some([-0.8, 1.0, -1.0]),
		Some([0.0, 1.0, -1.0]),
		Some([0.8, 1.0, -1.0]),
	];

	// Set gun texture
	let gun_texture = match gun_name {
		GunName::GunMachinegun => asset_server.load("temp_gun"),
	};

	let mut gun_list = Vec::new();
	let gun_transforms = match (turret_size, turret_num_barrels) {
		(ItemSize::Small, turret_list::TurretNumBarrels::Single) => DEFAULT_SMALL_SINGLE,
		(ItemSize::Small, turret_list::TurretNumBarrels::Double) => DEFAULT_SMALL_DOUBLE,
		(ItemSize::Small, turret_list::TurretNumBarrels::Triple) => DEFAULT_SMALL_TRIPLE,
		(ItemSize::Medium, turret_list::TurretNumBarrels::Single) => DEFAULT_MEDIUM_SINGLE,
		(ItemSize::Medium, turret_list::TurretNumBarrels::Double) => DEFAULT_MEDIUM_DOUBLE,
		(ItemSize::Medium, turret_list::TurretNumBarrels::Triple) => DEFAULT_MEDIUM_TRIPLE,
		(ItemSize::Large, turret_list::TurretNumBarrels::Single) => DEFAULT_LARGE_SINGLE,
		(ItemSize::Large, turret_list::TurretNumBarrels::Double) => DEFAULT_LARGE_DOUBLE,
		(ItemSize::Large, turret_list::TurretNumBarrels::Triple) => DEFAULT_LARGE_TRIPLE,
	};

	for &gun_transform in gun_transforms.iter() {
		if gun_transform.is_some() {
			gun_list.push(gun_list::TurretGunAssignment {
				transform: Transform::from_translation(Vec3::from(gun_transform.unwrap())),
				texture: gun_texture.clone(),
			});
		}
	}

	return TurretGunAssignmentList(gun_list);
}
