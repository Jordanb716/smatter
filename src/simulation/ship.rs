use super::*;

const BASE_TEXTURE_PATH_PROJECTILES: &str = "textures/projectiles/";
const BASE_AUDIO_PATH_GUNS: &str = "audio/sounds/guns/";

#[derive(Component)]
pub struct IsPlayerShip;

#[derive(Component, Clone, Copy, Deref, DerefMut, Serialize, Deserialize, Debug)]
pub struct Health(pub f32);

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
	pub visibility: Visibility,
}

impl Default for ShipBundle {
	fn default() -> Self {
		Self {
			health: Health(1.0),
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
		gun_name: &str,
		turret_num_barrels: turret::TurretNumBarrels,
		gun_definition_list: &Res<gun_list::GunDefinitionList>,
	) -> Self {
		let mount_size = self.turret_mount_list[mount_number].mount_size;
		let gun_definition = match gun_definition_list
			.iter()
			.find(|&gun_definition| gun_definition.gun_name == gun_name)
		{
			Some(val) => val,
			None => panic!(
				"Failed to find gun named {} in gun definitions list!",
				gun_name
			),
		};

		let (turret_texture, turret_texture_size) =
			turret::lookup_turret_texture(asset_server, gun_name, mount_size, turret_num_barrels);

		// Build turret
		let turret_bundle = turret::TurretBundle {
			turret_size: mount_size,
			turret_mount_number: turret::TurretMountNumber(mount_number),
			turret_properties: self.generate_turret_properties(mount_number),
			transform: self.turret_mount_list[mount_number].mount_transform,
			gun_properties: gun::GunProperties {
				gun_type: gun_definition.gun_type,
				gun_size: gun_definition.gun_size,
				rate_of_fire: gun_definition.rate_of_fire,
				projectile_damage: gun_definition.projectile_damage,
				projectile_velocity_mps: gun_definition.projectile_velocity_mps,
				velocity_deviation_percent: gun_definition.velocity_deviation_percent,
				bullet_spread_degrees: gun_definition.bullet_spread_degrees,
				projectile_texture: asset_server.load(
					&(BASE_TEXTURE_PATH_PROJECTILES.to_string()
						+ &gun_definition.projectile_texture),
				),
				projectile_texture_render_size: gun_definition.projectile_texture_render_size,
				fire_sound: asset_server
					.load(&(BASE_AUDIO_PATH_GUNS.to_string() + &gun_definition.fire_sound_path)),
			},
			gun_assignment_list: turret::generate_turret_gun_list(
				asset_server,
				mount_size,
				turret_num_barrels,
				&gun_definition.texture_path,
			),
			sprite: Sprite {
				custom_size: Some(turret_texture_size),
				..default()
			},
			texture: turret_texture,
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
					rotation_velocity: Quat::from_rotation_z(ROTATION_VELOCITY.to_radians()),
					..default()
				}
			}
			ItemSize::Medium => {
				const ROTATION_VELOCITY: f32 = 5.0;
				turret::TurretProperties {
					rotation_velocity: Quat::from_rotation_z(ROTATION_VELOCITY.to_radians()),
					..default()
				}
			}
			ItemSize::Large => {
				const ROTATION_VELOCITY: f32 = 2.5;
				turret::TurretProperties {
					rotation_velocity: Quat::from_rotation_z(ROTATION_VELOCITY.to_radians()),
					..default()
				}
			}
		}
	}
}
