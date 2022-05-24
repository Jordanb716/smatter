use super::*;

const SHIP_DATA_PATH: &str = "data/ships/";

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub enum ShipName {
	TemplateShip,
	PlayerTempShip,
}

/// Turret Mount Definition for storing turret mount data in Ship Definitions
#[derive(Serialize, Deserialize, Debug)]
pub struct TurretMountDefinition {
	pub size: ItemSize,
	pub translation: Vec2,
	pub rotation_degrees: f32,
	pub field_of_view_degrees: f32,
}

/// Ship definition for storing ship hull parameters as YAML
#[derive(Serialize, Deserialize, Debug)]
pub struct ShipDefinition {
	pub ship_name: ShipName,
	pub health: ship::Health,
	pub iff: interaction::IFF,
	pub texture_path: String,
	pub texture_scale: Vec2,
	pub turret_mounts: Vec<TurretMountDefinition>,
}

/// List of Ship Definitions
#[derive(Deref, DerefMut, Debug)]
pub struct ShipDefinitionList(Vec<ShipDefinition>);

/// Generates an explanatory template for how a Ship Definition should be formatted in YAML,
/// then writes it out to a template.yaml file.
pub fn write_ship_definition_template() {
	// Define template
	let ship_definition_template = ShipDefinition {
		ship_name: ShipName::TemplateShip,
		health: ship::Health(100.0),
		iff: interaction::IFF::Friendly,
		texture_path: "template_texture.png".to_string(),
		texture_scale: Vec2::new(200.0, 200.0),
		turret_mounts: vec![
			TurretMountDefinition {
				size: ItemSize::Small,
				translation: Vec2::new(-12.3, 45.6),
				rotation_degrees: -45.0,
				field_of_view_degrees: 270.0,
			},
			TurretMountDefinition {
				size: ItemSize::Medium,
				translation: Vec2::new(78.9, 10.0),
				rotation_degrees: 45.0,
				field_of_view_degrees: 180.0,
			},
			TurretMountDefinition {
				size: ItemSize::Large,
				translation: Vec2::new(1.0, 0.1),
				rotation_degrees: 0.0,
				field_of_view_degrees: 90.0,
			},
		],
	};
	// Write out template
	crate::game_io::write_definition_template(SHIP_DATA_PATH, ship_definition_template);
}

/// Reads all *.yaml Ship definition files in data/ships/ and returns them as a ```ShipDefinitionList```
pub fn read_ship_definitions() -> ShipDefinitionList {
	let ship_definition_list = ShipDefinitionList(crate::game_io::read_definitions(SHIP_DATA_PATH));
	return ship_definition_list;
}
