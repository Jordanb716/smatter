use super::*;

// Ship definitions

// Player ships

impl ship::ShipBundle {
	pub fn ship_temp(spawn_transform: Transform, asset_server: &Res<AssetServer>) -> Self {
		let mut ship_bundle = ship::ShipBundle {
			health: ship::Health(100),
			iff: interaction::IFF::Friendly,
			transform: spawn_transform,
			texture: asset_server.load("temp_ship.png"),
			..default()
		};

		// Turret Mounts
		let mut turret_mounts = ship::ShipTurretMountList(Vec::new());
		turret_mounts.push(ship::ShipTurretMount {
			mount_size: ItemSize::Small,
			mount_transform: Transform::from_xyz(-60.0, 21.0, 25.0)
				.with_rotation(Quat::from_rotation_z(-45.0_f32.to_radians())),
			mount_field_of_view_degrees: 270.0,
			mount_turret_entity: None,
		});
		turret_mounts.push(ship::ShipTurretMount {
			mount_size: ItemSize::Small,
			mount_transform: Transform::from_xyz(60.0, 21.0, 25.0)
				.with_rotation(Quat::from_rotation_z(45.0_f32.to_radians())),
			mount_field_of_view_degrees: 270.0,
			mount_turret_entity: None,
		});

		// Set Turret Mount List
		ship_bundle.turret_mount_list = turret_mounts;

		return ship_bundle;
	}
}

// Hegemonizing Swarm ships

impl ship::ShipBundle {}
