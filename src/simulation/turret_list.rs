use super::*;

// Turret types
#[derive(Clone, Copy, Debug)]
pub enum TurretNumBarrels {
	Single,
	Double,
	Triple,
}

/// Returns a turrets properties, texture, and texture_size from a list of turret names
pub fn turret_list(
	asset_server: &Res<AssetServer>,
	gun_name: gun_list::GunName,
) -> (gun::GunProperties, Handle<Image>, Option<Vec2>) {
	match gun_name {
		gun_list::GunName::GunMachinegun => (
			gun::GunProperties::gun_machinegun(asset_server),
			asset_server.load("temp_turret.png"),
			Some(Vec2::new(20.0, 20.0)),
		),
	}
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
