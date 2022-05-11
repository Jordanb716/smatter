use super::*;

#[derive(Component)]
pub struct IsPlayerShip;

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Deref, DerefMut, Debug)]
pub struct Health(pub i32);

#[derive(Debug)]
pub struct ShipTurretMount {
	pub mount_size: ItemSize,
	pub mount_transform: Transform,
	pub mount_field_of_view_degrees: f32,
	pub mount_turret_entity: Option<Entity>,
}

#[derive(Component, Deref, DerefMut, Debug)]
pub struct ShipTurretMountList(pub Vec<ShipTurretMount>);

/// A list of Turret Bundles that should be spawned as children for the ship
#[derive(Component, Deref, DerefMut, Clone, Debug)]
pub struct ShipTurretAssignmentList(pub Option<Vec<turret::TurretBundle>>);

impl ShipTurretAssignmentList {
	pub fn new(&mut self) {
		*self = ShipTurretAssignmentList(Some(Vec::new()));
	}
	pub fn push(&mut self, turret_bundle: turret::TurretBundle) {
		if self.is_none() {
			self.new();
		}
		self.as_mut().unwrap().push(turret_bundle);
	}
}

#[derive(Bundle)]
pub struct ShipBundle {
	pub health: Health,
	pub iff: interaction::IFF,
	pub turret_mount_list: ShipTurretMountList,
	pub turret_assignment_list: ShipTurretAssignmentList,

	pub transform: Transform,
	pub global_transform: GlobalTransform,
	pub velocity: physics::Velocity,
	pub acceleration: physics::Acceleration,

	pub sprite: Sprite,
	pub texture: Handle<Image>,
	/// User indication of whether an entity is visible
	pub visibility: Visibility,
}

impl Default for ShipBundle {
	fn default() -> Self {
		Self {
			health: Health(1),
			iff: interaction::IFF::Neutral,
			turret_mount_list: ShipTurretMountList(Vec::new()),
			turret_assignment_list: ShipTurretAssignmentList(None),

			transform: Default::default(),
			global_transform: Default::default(),
			velocity: physics::Velocity(Vec2::new(0.0, 0.0)),
			acceleration: physics::Acceleration(Vec2::new(0.0, 0.0)),

			sprite: Default::default(),
			texture: bevy::render::texture::DEFAULT_IMAGE_HANDLE.typed(),
			visibility: Default::default(),
		}
	}
}

impl ShipBundle {
	pub fn generate_turret(
		mut self,
		asset_server: &Res<AssetServer>,
		mount_number: usize,
		gun_name: gun_list::GunName,
		turret_num_barrels: turret_list::TurretNumBarrels,
	) -> Self {
		let mount_size = self.turret_mount_list[mount_number].mount_size;

		// Build turret
		let turret_bundle = turret::TurretBundle {
			turret_size: mount_size,
			turret_mount_number: turret::TurretMountNumber(mount_number),
			turret_properties: self.generate_turret_properties(mount_number),
			gun_properties: match gun_name {
				gun_list::GunName::GunMachinegun => gun::GunProperties::gun_machinegun(),
			},
			gun_list: gun_list::generate_gun_list(
				asset_server,
				gun_name,
				mount_size,
				turret_num_barrels,
			),
			transform: self.turret_mount_list[mount_number].mount_transform,
			texture: asset_server.load("temp_turret.png"),
			..default()
		};

		// Ensure selected gun is correct size for mount.
		assert!(turret_bundle.gun_properties.gun_size == mount_size);

		// Add turret to ship's TurretAssignmentList
		self.turret_assignment_list.push(turret_bundle);

		return self;
	}

	pub fn generate_turret_properties(&self, mount_number: usize) -> turret::TurretProperties {
		match self.turret_mount_list[mount_number].mount_size {
			ItemSize::Small => {
				const ROTATION_VELOCITY: f32 = 10.0;
				turret::TurretProperties {
					field_of_view_degrees: self.turret_mount_list[mount_number]
						.mount_field_of_view_degrees,
					rotation_velocity: Quat::from_rotation_z(ROTATION_VELOCITY.to_radians()),
					..default()
				}
			}
			ItemSize::Medium => {
				const ROTATION_VELOCITY: f32 = 5.0;
				turret::TurretProperties {
					field_of_view_degrees: self.turret_mount_list[mount_number]
						.mount_field_of_view_degrees,
					rotation_velocity: Quat::from_rotation_z(ROTATION_VELOCITY.to_radians()),
					..default()
				}
			}
			ItemSize::Large => {
				const ROTATION_VELOCITY: f32 = 2.5;
				turret::TurretProperties {
					field_of_view_degrees: self.turret_mount_list[mount_number]
						.mount_field_of_view_degrees,
					rotation_velocity: Quat::from_rotation_z(ROTATION_VELOCITY.to_radians()),
					..default()
				}
			}
		}
	}
}
