use super::*;

const CARTRIDGE_DATA_PATH: &str = "data/cartridges/";

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum ProjectileName {
	TemplateProjectile,
	SmallMachinegunBullet,
}

impl Default for ProjectileName {
	fn default() -> Self {
		ProjectileName::TemplateProjectile
	}
}

/// Projectile definition for storing projectile parameters as YAML
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct CartridgeDefinition {
	pub num_projectiles: i32,
	pub projectile_name: ProjectileName,
	pub guidance: projectile::ProjectileGuidance,
	pub projectile_damage: f32,

	pub projectile_velocity_mps: f32,
	pub velocity_deviation_percent: f32,
	pub bullet_spread_degrees: f32,

	pub texture_path: String,
	pub texture_render_size: Vec2,
}

/// List of Gun Definitions
#[derive(Deref, DerefMut, Debug)]
pub struct CartridgeDefinitionList(Vec<CartridgeDefinition>);

/// Generates an explanatory template for how a Gun Definition should be formatted in YAML,
/// then writes it out to a template.yaml file.
pub fn write_cartridge_definition_template() {
	// Define template
	let cartridge_definition_template = CartridgeDefinition {
		num_projectiles: 1,
		projectile_name: ProjectileName::TemplateProjectile,
		guidance: projectile::ProjectileGuidance::None,
		projectile_damage: 10.0,
		projectile_velocity_mps: 100.0,
		velocity_deviation_percent: 0.01,
		bullet_spread_degrees: 4.0,
		texture_path: "template_proj_texture.png".to_string(),
		texture_render_size: Vec2::new(1.0, 1.0),
	};
	// Write out template
	crate::game_io::write_definition_template(CARTRIDGE_DATA_PATH, cartridge_definition_template);
}

/// Reads all *.yaml Gun definition files in data/guns/ and returns them as a ```GunDefinitionList```
pub fn read_cartridge_definitions() -> CartridgeDefinitionList {
	let cartridge_definition_list =
		CartridgeDefinitionList(crate::game_io::read_definitions(CARTRIDGE_DATA_PATH));
	return cartridge_definition_list;
}
