use super::*;

const BASE_TEXTURES_PATH_TURRETS: &str = "textures/turrets/";
const BASE_TEXTURES_PATH_GUNS: &str = "textures/guns/";

/// The number of gun barrels on the turret
#[derive(Clone, Copy, Debug)]
pub enum TurretNumBarrels {
	Single,
	Double,
	Triple,
}

/// Designates entity as a turret
#[derive(Component, Clone, Default, Debug)]
pub struct IsTurret;

/// Current target engagement state of the turret
#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub enum TurretState {
	NoTarget,
	//	Laying,
	Firing,
}

/// Holds the turret mount number the turret is assigned to
#[derive(Component, Clone, Deref, DerefMut, Debug)]
pub struct TurretMountNumber(pub usize);

/// Hold turret state data
#[derive(Component, Clone, Copy, Debug)]
pub struct TurretProperties {
	pub rotation_velocity: Quat,
	pub target_entity: Option<Entity>,
	pub turret_state: TurretState,
}

impl Default for TurretProperties {
	fn default() -> Self {
		Self {
			rotation_velocity: Quat::from_rotation_z((10.0_f32).to_radians()),
			target_entity: None,
			turret_state: TurretState::NoTarget,
		}
	}
}

#[derive(Clone, Debug)]
pub struct TurretGunAssignment {
	pub transform: Transform,
	pub texture: Handle<Image>,
	/// Defines the size the texture should be rendered at in meters.
	pub texture_size: Vec2,
}

/// List of guns on a turret
#[derive(Component, Clone, Deref, DerefMut, Debug)]
pub struct TurretGunAssignmentList(pub Option<Vec<TurretGunAssignment>>);

#[derive(Bundle, Clone, Debug)]
pub struct TurretBundle {
	pub is_turret: IsTurret,
	pub turret_size: ItemSize,
	pub turret_mount_number: TurretMountNumber,
	pub turret_properties: TurretProperties,

	pub transform: Transform,
	pub global_transform: GlobalTransform,

	/// Name of gun assigned to the turret
	pub gun_properties: gun::GunProperties,

	/// List of child gun components
	pub gun_assignment_list: TurretGunAssignmentList,

	pub sprite: Sprite,
	pub texture: Handle<Image>,
	pub visibility: Visibility,
}

impl Default for TurretBundle {
	fn default() -> Self {
		Self {
			is_turret: default(),
			turret_size: ItemSize::Small,
			turret_properties: TurretProperties::default(),
			gun_properties: gun::GunProperties::default(),
			gun_assignment_list: TurretGunAssignmentList(None),
			turret_mount_number: TurretMountNumber(0),
			transform: default(),
			global_transform: default(),
			sprite: default(),
			texture: default(),
			visibility: default(),
		}
	}
}

/// Returns a turrets texture, and texture_size based on gun_name, size, and num_barrels
pub fn lookup_turret_texture(
	asset_server: &Res<AssetServer>,
	gun_name: &str,
	turret_mount_size: ItemSize,
	number_barrels: TurretNumBarrels,
) -> (Handle<Image>, Vec2) {
	let (turret_texture, turret_texture_size) = match gun_name {
		//Exceptions for specific guns
		_ => match turret_mount_size {
			ItemSize::Small => match number_barrels {
				TurretNumBarrels::Single => (
					asset_server.load(&format!("{}temp_turret.png", BASE_TEXTURES_PATH_TURRETS)),
					Vec2::new(20.0, 20.0),
				),
				TurretNumBarrels::Double => (
					asset_server.load(&format!("{}temp_turret.png", BASE_TEXTURES_PATH_TURRETS)),
					Vec2::new(20.0, 20.0),
				),
				TurretNumBarrels::Triple => (
					asset_server.load(&format!("{}temp_turret.png", BASE_TEXTURES_PATH_TURRETS)),
					Vec2::new(20.0, 20.0),
				),
			},
			ItemSize::Medium => match number_barrels {
				TurretNumBarrels::Single => (
					asset_server.load(&format!("{}temp_turret.png", BASE_TEXTURES_PATH_TURRETS)),
					Vec2::new(40.0, 40.0),
				),
				TurretNumBarrels::Double => (
					asset_server.load(&format!("{}temp_turret.png", BASE_TEXTURES_PATH_TURRETS)),
					Vec2::new(40.0, 40.0),
				),
				TurretNumBarrels::Triple => (
					asset_server.load(&format!("{}temp_turret.png", BASE_TEXTURES_PATH_TURRETS)),
					Vec2::new(40.0, 40.0),
				),
			},
			ItemSize::Large => match number_barrels {
				TurretNumBarrels::Single => (
					asset_server.load(&format!("{}temp_turret.png", BASE_TEXTURES_PATH_TURRETS)),
					Vec2::new(80.0, 80.0),
				),
				TurretNumBarrels::Double => (
					asset_server.load(&format!("{}temp_turret.png", BASE_TEXTURES_PATH_TURRETS)),
					Vec2::new(80.0, 80.0),
				),
				TurretNumBarrels::Triple => (
					asset_server.load(&format!("{}temp_turret.png", BASE_TEXTURES_PATH_TURRETS)),
					Vec2::new(80.0, 80.0),
				),
			},
		},
	};
	return (turret_texture, turret_texture_size);
}

/// Generates a Gun List (placing guns relative to the parent turret)
pub fn generate_turret_gun_list(
	asset_server: &Res<AssetServer>,
	turret_size: ItemSize,
	turret_num_barrels: turret::TurretNumBarrels,
	gun_texture_path: &str,
) -> TurretGunAssignmentList {
	// List of default gun transforms based on turret size and number of guns
	const MAX_GUNS: usize = 3;
	// Single turrets
	const DEFAULT_SMALL_SINGLE: [Option<[f32; 3]>; MAX_GUNS] = [
		Some([0.0, 2.0, -1.0]), //
		None,                   //
		None,
	];
	const DEFAULT_MEDIUM_SINGLE: [Option<[f32; 3]>; MAX_GUNS] = [
		Some([0.0, 2.0, -1.0]), //
		None,                   //
		None,
	];
	const DEFAULT_LARGE_SINGLE: [Option<[f32; 3]>; MAX_GUNS] = [
		Some([0.0, 2.0, -1.0]), //
		None,                   //
		None,
	];
	// Double turrets
	const DEFAULT_SMALL_DOUBLE: [Option<[f32; 3]>; MAX_GUNS] = [
		Some([-1.5, 2.0, -1.0]), //
		Some([1.5, 2.0, -1.0]),  //
		None,
	];
	const DEFAULT_MEDIUM_DOUBLE: [Option<[f32; 3]>; MAX_GUNS] = [
		Some([-0.5, 2.0, -1.0]), //
		Some([0.5, 2.0, -1.0]),  //
		None,
	];
	const DEFAULT_LARGE_DOUBLE: [Option<[f32; 3]>; MAX_GUNS] = [
		Some([-0.5, 2.0, -1.0]), //
		Some([0.5, 2.0, -1.0]),  //
		None,
	];
	// Triple turrets
	const DEFAULT_SMALL_TRIPLE: [Option<[f32; 3]>; MAX_GUNS] = [
		Some([-2.0, 2.0, -1.0]),
		Some([0.0, 2.0, -1.0]),
		Some([2.0, 2.0, -1.0]),
	];
	const DEFAULT_MEDIUM_TRIPLE: [Option<[f32; 3]>; MAX_GUNS] = [
		Some([-0.8, 2.0, -1.0]),
		Some([0.0, 2.0, -1.0]),
		Some([0.8, 2.0, -1.0]),
	];
	const DEFAULT_LARGE_TRIPLE: [Option<[f32; 3]>; MAX_GUNS] = [
		Some([-0.8, 2.0, -1.0]),
		Some([0.0, 2.0, -1.0]),
		Some([0.8, 2.0, -1.0]),
	];

	// Set gun texture
	let gun_texture = asset_server.load(&(BASE_TEXTURES_PATH_GUNS.to_string() + gun_texture_path));

	let mut gun_list = Vec::new();
	let gun_transforms = match (turret_size, turret_num_barrels) {
		(ItemSize::Small, turret::TurretNumBarrels::Single) => DEFAULT_SMALL_SINGLE,
		(ItemSize::Small, turret::TurretNumBarrels::Double) => DEFAULT_SMALL_DOUBLE,
		(ItemSize::Small, turret::TurretNumBarrels::Triple) => DEFAULT_SMALL_TRIPLE,
		(ItemSize::Medium, turret::TurretNumBarrels::Single) => DEFAULT_MEDIUM_SINGLE,
		(ItemSize::Medium, turret::TurretNumBarrels::Double) => DEFAULT_MEDIUM_DOUBLE,
		(ItemSize::Medium, turret::TurretNumBarrels::Triple) => DEFAULT_MEDIUM_TRIPLE,
		(ItemSize::Large, turret::TurretNumBarrels::Single) => DEFAULT_LARGE_SINGLE,
		(ItemSize::Large, turret::TurretNumBarrels::Double) => DEFAULT_LARGE_DOUBLE,
		(ItemSize::Large, turret::TurretNumBarrels::Triple) => DEFAULT_LARGE_TRIPLE,
	};

	for &gun_transform in gun_transforms.iter() {
		if gun_transform.is_some() {
			gun_list.push(TurretGunAssignment {
				transform: Transform::from_translation(Vec3::from(gun_transform.unwrap())),
				texture: gun_texture.clone(),
				texture_size: Vec2::new(10.0, 10.0),
			});
		}
	}

	return TurretGunAssignmentList(Some(gun_list));
}

// ==========
// Systems

/// When a ship's Turret Assignment List is changed, this system consumes and spawns child turrets from it.
pub fn ship_turret_spawn_system(
	mut commands: Commands,
	mut turret_assignment_lists: Query<
		(
			Entity,
			&mut ship::ShipTurretAssignmentList,
			&mut ship::ShipTurretMountList,
			Option<&Children>,
		),
		Changed<ship::ShipTurretAssignmentList>,
	>,
) {
	// Iterate through ships with Turret Assignment Lists
	for (ship, mut turret_assignment_list, mut turret_mount_list, children) in
		turret_assignment_lists.iter_mut()
	{
		match turret_assignment_list.0.as_mut() {
			None => continue, // List is empty, skip,
			Some(turret_assignment) => {
				// At least one turret is in list, consume and spawn
				for _ in 0..turret_assignment.len() {
					match turret_assignment.pop() {
						Some(new_turret) => {
							let new_turret_mount_number = new_turret.turret_mount_number.0;
							// If turret already exists corresponding to that mount, despawn it first
							if let Some(old_turret) =
								turret_mount_list[new_turret_mount_number].mount_turret_entity
							{
								match children {
									Some(children) => {
										// A mount entity already exists in the list, and there are children, find and despawn
										for child_entity in children.iter() {
											if child_entity.id() == old_turret.id() {
											commands.entity(*child_entity).despawn_recursive();
											break;
											}
										}
									}
									None => panic!("Turret mount list contains turret entity, but no child entities exist!"),
								}
							}
							// Spawn turret
							commands.entity(ship).with_children(|parent| {
								turret_mount_list[new_turret_mount_number].mount_turret_entity =
									Some(parent.spawn_bundle(new_turret).id());
							});
						}
						None => panic!(
							"Popped a 'None' from a Turret Assignment List! Behold: {:?}",
							turret_assignment
						),
					}
				}
			}
		}
	}
}
