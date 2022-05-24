use super::*;

const BASE_TEXTURES_PATH_SHIPS: &str = "textures/ships/";

pub fn spawn_player_ship(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	ship_definition_list: Res<ship_list::ShipDefinitionList>,
	gun_definition_list: Res<gun_list::GunDefinitionList>,
	cartridge_definition_list: Res<cartridge_list::CartridgeDefinitionList>,
) {
	let spawn_transform = Transform::from_xyz(0.0, -500.0, 0.0);

	let player_ship = spawn_ship(
		ship_list::ShipName::PlayerTempShip,
		spawn_transform,
		&asset_server,
		&ship_definition_list,
	)
	.generate_turret( // Nose
		&asset_server,
		0,
		gun_list::GunName::SmallMachinegun,
		turret::TurretNumBarrels::Double,
		&gun_definition_list,
		&cartridge_definition_list,
	)
	/* .generate_turret( // Front Left
		&asset_server,
		1,
		gun_list::GunName::SmallMachinegun,
		turret::TurretNumBarrels::Single,
		&gun_definition_list,
		&cartridge_definition_list,
	)
	.generate_turret( // Front Right
		&asset_server,
		2,
		gun_list::GunName::SmallMachinegun,
		turret::TurretNumBarrels::Single,
		&gun_definition_list,
		&cartridge_definition_list,
	)
	.generate_turret( // Rear Left
		&asset_server,
		3,
		gun_list::GunName::SmallMachinegun,
		turret::TurretNumBarrels::Double,
		&gun_definition_list,
		&cartridge_definition_list,
	)
	.generate_turret( // Rear Right
		&asset_server,
		4,
		gun_list::GunName::SmallMachinegun,
		turret::TurretNumBarrels::Double,
		&gun_definition_list,
		&cartridge_definition_list,
	)
	.generate_turret( // Tail
		&asset_server,
		5,
		gun_list::GunName::SmallMachinegun,
		turret::TurretNumBarrels::Triple,
		&gun_definition_list,
		&cartridge_definition_list,
	) */;

	commands
		.spawn_bundle(player_ship)
		.insert(ship::IsPlayerShip);
}

pub fn spawn_ship(
	ship_name: ship_list::ShipName,
	spawn_transform: Transform, // Translation and Rotation to spawn the ship at
	asset_server: &Res<AssetServer>,
	ship_definition_list: &Res<ship_list::ShipDefinitionList>,
) -> ship::ShipBundle {
	const TURRET_Z_OFFSET: f32 = 25.0;
	let mut ship_bundle = None::<ship::ShipBundle>;

	for ship_definition in ship_definition_list.iter() {
		if ship_definition.ship_name == ship_name {
			// Found ship in list, fill the Ship Bundle
			ship_bundle = Some(ship::ShipBundle {
				health: ship_definition.health,
				iff: ship_definition.iff,
				transform: spawn_transform,
				texture: asset_server
					.load(&(BASE_TEXTURES_PATH_SHIPS.to_string() + &ship_definition.texture_path)),
				sprite: Sprite {
					custom_size: Some(ship_definition.texture_scale),
					..default()
				},
				..default()
			});

			// Turret Mounts
			let mut turret_mounts = ship::ShipTurretMountList(Vec::new());
			for turret_mount in ship_definition.turret_mounts.iter() {
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
			panic!("Failed to find {:?} in Ship definitions list!", ship_name)
		}
	};

	return ship_bundle;
}

// ==========
// Systems

pub fn target_spawn_system(mut commands: Commands, time: Res<Time>, mut timer: ResMut<SpawnTimer>) {
	if timer.tick(time.delta()).just_finished() {
		commands
			.spawn_bundle(SpriteBundle {
				sprite: Sprite {
					color: (Color::YELLOW),
					..default()
				},
				transform: Transform {
					translation: Vec3::new(rand::random::<f32>() * 600.0 - 300.0, 400.0, 0.0),
					scale: Vec3::new(4.0, 4.0, 0.0),
					..default()
				},
				..default()
			})
			.insert(ship::Health(10.0))
			.insert(physics::Velocity(Vec2::new(
				rand::random::<f32>() * 80.0 - 10.0,
				rand::random::<f32>() * -80.0 - 20.0,
			)))
			.insert(interaction::IFF::Enemy);
	}
}
