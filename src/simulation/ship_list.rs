use super::*;
use std::fs;

// Ship definitions

#[derive(Serialize, Deserialize, Debug)]
struct TurretMountDefinition {
	size: ItemSize,
	translation: Vec2,
	rotation_degrees: f32,
	field_of_view_degrees: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipDefinition {
	ship_name: String,
	health: ship::Health,
	iff: interaction::IFF,
	texture_path: String,
	texture_scale: Vec2,
	turrets: Vec<TurretMountDefinition>,
}

#[derive(Deref, DerefMut, Debug)]
pub struct ShipDefinitionList(Vec<ShipDefinition>);

pub fn generate_ship_definition_template() {
	let ship_definition_template = ShipDefinition {
		ship_name: "ship_name".to_string(),
		health: ship::Health(100),
		iff: interaction::IFF::Friendly,
		texture_path: "texture.png".to_string(),
		texture_scale: Vec2::new(200.0, 200.0),
		turrets: vec![
			TurretMountDefinition {
				size: ItemSize::Small,
				translation: Vec2::new(-31.4, 12.0),
				rotation_degrees: -45.0,
				field_of_view_degrees: 270.0,
			},
			TurretMountDefinition {
				size: ItemSize::Medium,
				translation: Vec2::new(35.0, 12.0),
				rotation_degrees: 45.0,
				field_of_view_degrees: 270.0,
			},
		],
	};

	// Convert template struct to a yaml string
	let ship_definition_template = serde_yaml::to_string(&ship_definition_template)
		.expect("Ship Template serialization failed.");
	// Try to create the directory in case it hasn't been initialized
	fs::create_dir_all("data/ships/").expect("Ship definition path creation failed.");
	// Write the template out
	fs::write("data/ships/ship_template.yaml", ship_definition_template)
		.expect("Writing Ship Template failed.");
}

pub fn read_ship_definitions() -> ShipDefinitionList {
	const PATH: &str = "data/ships/";
	let ship_definition_list = ShipDefinitionList(crate::game_io::read_definitions(PATH));
	return ship_definition_list;
}

pub fn spawn_ship(
	ship_name: &str,
	spawn_transform: Transform,
	asset_server: &Res<AssetServer>,
	ship_definition_list: &Res<ShipDefinitionList>,
) -> ship::ShipBundle {
	const TURRET_Z_OFFSET: f32 = 25.0;
	let mut ship_bundle = None::<ship::ShipBundle>;

	for ship_definition in ship_definition_list.0.iter() {
		if ship_definition.ship_name == ship_name {
			// Found ship in list, fill the Ship Bundle
			ship_bundle = Some(ship::ShipBundle {
				health: ship_definition.health,
				iff: ship_definition.iff,
				transform: spawn_transform,
				texture: asset_server.load(&ship_definition.texture_path),
				sprite: Sprite {
					custom_size: Some(ship_definition.texture_scale),
					..default()
				},
				..default()
			});

			// Turret Mounts
			let mut turret_mounts = ship::ShipTurretMountList(Vec::new());
			for turret_mount in ship_definition.turrets.iter() {
				turret_mounts.push(ship::ShipTurretMount {
					mount_size: turret_mount.size,
					mount_transform: Transform::from_translation(
						turret_mount.translation.extend(TURRET_Z_OFFSET),
					)
					.with_rotation(Quat::from_rotation_z(
						turret_mount.rotation_degrees.to_radians(),
					)),
					mount_field_of_view_degrees: turret_mount.field_of_view_degrees,
					mount_turret_entity: None,
				});
			}
			ship_bundle.as_mut().unwrap().turret_mount_list = turret_mounts;
		}
	}

	let ship_bundle = match ship_bundle {
		Some(_) => ship_bundle.unwrap(),
		None =>
		// Never found the ship in the list.
		{
			panic!("Failed to find {} in Ship definitions list!", ship_name)
		}
	};

	return ship_bundle;
}

/*
pub fn ship_temp(spawn_transform: Transform, asset_server: &Res<AssetServer>) -> Self {
	let mut ship_bundle = ship::ShipBundle {
		health: ship::Health(100),
		iff: interaction::IFF::Friendly,
		transform: spawn_transform,
		texture: asset_server.load("ship.png"),
		sprite: Sprite {
			custom_size: Some(Vec2::new(200.0, 200.0)),
			..default()
		},
		..default()
	};

	// Turret Mounts
	let mut turret_mounts = ship::ShipTurretMountList(Vec::new());
	turret_mounts.push(ship::ShipTurretMount {
		mount_size: ItemSize::Small,
		mount_transform: Transform::from_xyz(-31.4, 12.0, 25.0)
			.with_rotation(Quat::from_rotation_z(-45.0_f32.to_radians())),
		mount_field_of_view_degrees: 270.0,
		mount_turret_entity: None,
	});
	turret_mounts.push(ship::ShipTurretMount {
		mount_size: ItemSize::Small,
		mount_transform: Transform::from_xyz(35.0, 12.0, 25.0)
			.with_rotation(Quat::from_rotation_z(45.0_f32.to_radians())),
		mount_field_of_view_degrees: 270.0,
		mount_turret_entity: None,
	});

	// Set Turret Mount List
	ship_bundle.turret_mount_list = turret_mounts;

	return ship_bundle;
}
*/
