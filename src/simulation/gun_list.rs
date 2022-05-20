use super::*;

#[derive(Clone, Debug)]
pub struct TurretGunAssignment {
	pub transform: Transform,
	pub texture: Handle<Image>,
	/// Defines the size the texture should be rendered at in meters.
	pub texture_size: Vec2,
}

impl Default for TurretGunAssignment {
	fn default() -> Self {
		Self {
			transform: default(),
			texture: default(),
			texture_size: Vec2::new(1.0, 1.0),
		}
	}
}

/// List of guns on a turret
#[derive(Component, Clone, Deref, DerefMut, Debug)]
pub struct TurretGunAssignmentList(pub Option<Vec<TurretGunAssignment>>);

// Gun definitions

/// Gun definition for storing gun parameters as YAML
#[derive(Serialize, Deserialize, Debug)]
pub struct GunDefinition {
	pub gun_name: String,
	pub gun_type: gun::GunType,
	pub gun_size: ItemSize,

	pub projectile_damage: f32,
	pub rate_of_fire: f32,

	pub projectile_velocity_mps: f32,
	pub velocity_deviation_percent: f32,
	pub bullet_spread_degrees: f32,

	pub texture_path: String,
	pub texture_render_size: Vec2,
	pub projectile_texture: String,
	pub projectile_texture_render_size: Vec2,
	pub fire_sound_path: String,
}

/// List of Gun Definitions
#[derive(Deref, DerefMut, Debug)]
pub struct GunDefinitionList(Vec<GunDefinition>);

/// Generates an explanatory template for how a Gun Definition should be formatted in YAML,
/// then writes it out to a template.yaml file.
pub fn write_gun_definition_template() {
	const PATH: &str = "data/guns/";
	// Define template
	let ship_definition_template = GunDefinition {
		gun_name: "template_gun".to_string(),
		gun_type: gun::GunType::Kinetic,
		gun_size: ItemSize::Small,
		projectile_damage: 10.0,
		rate_of_fire: 10.0,
		projectile_velocity_mps: 400.0,
		velocity_deviation_percent: 0.01,
		bullet_spread_degrees: 4.0,
		texture_path: "template_texture.png".to_string(),
		texture_render_size: Vec2::new(10.0, 10.0),
		projectile_texture: "template_proj_texture.png".to_string(),
		projectile_texture_render_size: Vec2::new(1.0, 1.0),
		fire_sound_path: "template_audio.ogg".to_string(),
	};
	// Write out template
	crate::game_io::write_definition_template(PATH, ship_definition_template);
}

/// Reads all *.yaml Gun definition files in data/guns/ and returns them as a ```GunDefinitionList```
pub fn read_gun_definitions() -> GunDefinitionList {
	const PATH: &str = "data/guns/";
	let gun_definition_list = GunDefinitionList(crate::game_io::read_definitions(PATH));
	return gun_definition_list;
}

/// Generates a Gun List (placing guns relative to the parent turret)
pub fn generate_gun_list(
	asset_server: &Res<AssetServer>,
	turret_size: ItemSize,
	turret_num_barrels: turret_list::TurretNumBarrels,
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
	let gun_texture = asset_server.load(gun_texture_path);

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
				texture_size: Vec2::new(10.0, 10.0),
			});
		}
	}

	return TurretGunAssignmentList(Some(gun_list));
}

// ==========
// Systems

/// When a turret's Gun Assignment List is changed, this system consumes and spawns child guns from it.
pub fn turret_gun_spawn_system(
	mut commands: Commands,
	mut gun_assignment_lists: Query<
		(Entity, &mut TurretGunAssignmentList),
		Changed<TurretGunAssignmentList>,
	>,
) {
	// Iterate through turrets with Gun Assignment Lists
	for (turret, mut gun_assignmment_list_option) in gun_assignment_lists.iter_mut() {
		match gun_assignmment_list_option.0.as_mut() {
			None => continue, // List is empty, skip,,
			Some(gun_assignmment_list) => {
				// At least one gun is in list, consume and spawn
				for _ in 0..gun_assignmment_list.len() {
					match gun_assignmment_list.pop() {
						Some(gun_assignment) => {
							// Spawn turret
							commands.entity(turret).with_children(|parent| {
								parent.spawn_bundle(gun::GunBundle {
									transform: gun_assignment.transform,
									texture: gun_assignment.texture,
									sprite: Sprite {
										custom_size: Some(gun_assignment.texture_size),
										..default()
									},
									..default()
								});
							});
						}
						None => panic!(
							"Popped a 'None' from a Gun Assignment List! Behold: {:?}",
							gun_assignmment_list
						),
					}
				}
			}
		}
	}
}
