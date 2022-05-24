use super::*;

const GUN_DATA_PATH: &str = "data/guns/";

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub enum GunName {
	TemplateGun,
	SmallMachinegun,
}

/// Gun definition for storing gun parameters as YAML
#[derive(Serialize, Deserialize, Debug)]
pub struct GunDefinition {
	pub gun_name: GunName,
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
	// Define template
	let ship_definition_template = GunDefinition {
		gun_name: GunName::TemplateGun,
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
	crate::game_io::write_definition_template(GUN_DATA_PATH, ship_definition_template);
}

/// Reads all *.yaml Gun definition files in data/guns/ and returns them as a ```GunDefinitionList```
pub fn read_gun_definitions() -> GunDefinitionList {
	let gun_definition_list = GunDefinitionList(crate::game_io::read_definitions(GUN_DATA_PATH));
	return gun_definition_list;
}

// ==========
// Systems

/// When a turret's Gun Assignment List is changed, this system consumes and spawns child guns from it.
pub fn turret_gun_spawn_system(
	mut commands: Commands,
	mut gun_assignment_lists: Query<
		(Entity, &mut turret::TurretGunAssignmentList),
		Changed<turret::TurretGunAssignmentList>,
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
