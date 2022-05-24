use super::*;

const BASE_TEXTURE_PATH_PROJECTILES: &str = "textures/projectiles/";

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum GunType {
	Kinetic,
	//Beam,
}

impl Default for GunType {
	fn default() -> Self {
		GunType::Kinetic
	}
}

/// Marks entity as a gun
#[derive(Component, Clone, Default, Debug)]
pub struct IsGun;

/// Timer that tracks time for a gun to cycle and be ready to fire again
#[derive(Component, Deref, DerefMut)]
pub struct GunCycleTimer(Timer);

/// Weapon properties of a gun and data needed for its operation and projectile spawning
#[derive(Component, Clone, Default, Debug)]
pub struct GunProperties {
	pub gun_type: GunType,
	pub gun_size: ItemSize,
	pub rate_of_fire: f32,
	pub cartridge_data: cartridge_list::CartridgeDefinition,
	pub fire_sound: Handle<AudioSource>,
}

/// Bundle of components needed to spawn a gun
#[derive(Bundle)]
pub struct GunBundle {
	pub is_gun: IsGun,
	pub transform: Transform,
	pub global_transform: GlobalTransform,

	pub gun_cycle_timer: GunCycleTimer,

	pub texture: Handle<Image>,
	pub sprite: Sprite,
	pub visibility: Visibility,
}

impl Default for GunBundle {
	fn default() -> Self {
		Self {
			is_gun: default(),
			transform: default(),
			global_transform: default(),

			gun_cycle_timer: GunCycleTimer(Timer::default()),

			texture: default(),
			sprite: default(),
			visibility: default(),
		}
	}
}

// ==========
// Systems

pub fn gun_firing_system(
	time: Res<Time>,
	audio: Res<Audio>,
	asset_server: Res<AssetServer>,
	mut commands: Commands,
	mut guns: Query<(&Parent, &GlobalTransform, &mut GunCycleTimer)>,
	turrets: Query<(&Parent, &turret::TurretProperties, &GunProperties)>,
	ships: Query<(&interaction::IFF, &physics::Velocity)>,
) {
	for (parent_turret, gun_transform, mut gun_cycle_timer) in guns.iter_mut() {
		// Get Turret and Gun properties from parent turret
		let (turret_parent, turret_properties, gun_properties) = turrets
			.get(parent_turret.0)
			.expect("Failed to get parent turret.");

		gun_cycle_timer.tick(time.delta());

		let (ship_iff, ship_velocity) = ships
			.get(turret_parent.0)
			.expect("Failed to get turret's parent ship");

		if turret_properties.turret_state == turret::TurretState::Firing {
			if gun_cycle_timer.finished() {
				// Set timer for RoF delay.
				gun_cycle_timer.0 = Timer::from_seconds(gun_properties.rate_of_fire.recip(), false);

				// Calculate random spread
				let shot_deviation = (((rand::random::<f32>() + rand::random::<f32>()) / 2.0
					- 0.5) * gun_properties.cartridge_data.bullet_spread_degrees)
					.to_radians();

				// Add deviation to projectile velocity
				let velocity_deviation_mps = gun_properties.cartridge_data.projectile_velocity_mps
					* gun_properties.cartridge_data.velocity_deviation_percent;
				let turret_projectile_velocity =
					gun_properties.cartridge_data.projectile_velocity_mps
						+ (rand::random::<f32>() - 0.5) * velocity_deviation_mps;

				commands.spawn_bundle(projectile::ProjectileBundle {
					damage: interaction::Damage(gun_properties.cartridge_data.projectile_damage),
					iff: ship_iff.clone(),
					transform: Transform {
						translation: gun_transform.translation + Vec3::new(0.0, 0.0, -10.0),
						rotation: gun_transform.rotation,
						..default()
					},
					velocity: physics::Velocity(
						Vec2::from(
							(-gun_transform.rotation.to_scaled_axis().to_array()[2]
								+ shot_deviation)
								.sin_cos(),
						) * turret_projectile_velocity
							+ ship_velocity.0,
					),
					sprite: Sprite {
						custom_size: Some(gun_properties.cartridge_data.texture_render_size),
						..default()
					},
					texture: asset_server.load(
						&(BASE_TEXTURE_PATH_PROJECTILES.to_string()
							+ &gun_properties.cartridge_data.texture_path),
					),
					..default()
				});

				// Play gunfire sound effect
				audio.play(gun_properties.fire_sound.clone());
			}
		}
	}
}
