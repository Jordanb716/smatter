use super::*;

#[derive(Clone, Copy, Debug)]
pub enum GunType {
	Kinetic,
	//Laser,
	//Plasma,
}

#[derive(Component)]
pub struct IsGun;

#[derive(Component)]
pub struct IsProjectile;

//#[derive(Component, Deref, DerefMut)]
//pub struct GunPosition(pub Vec2);

#[derive(Component, Deref, DerefMut)]
pub struct GunCycleTimer(Timer);

#[derive(Component, Clone, Debug)]
pub struct GunProperties {
	pub gun_size: ItemSize,
	pub gun_type: GunType,
	pub rate_of_fire: f32,

	pub projectile_velocity_mps: f32,
	pub velocity_deviation_percent: f32,
	pub bullet_spread_degrees: f32,

	pub projectile_texture: Handle<Image>,
	/// Defines the size the texture should be rendered at in meters.
	pub projectile_texture_size: Vec2,
	pub fire_sound: Handle<AudioSource>,
	pub projectile_damage: i32,
}

impl Default for GunProperties {
	fn default() -> Self {
		Self {
			gun_size: ItemSize::Small,
			gun_type: GunType::Kinetic,
			rate_of_fire: 10.0,

			projectile_velocity_mps: 100.0,
			velocity_deviation_percent: 0.01,
			bullet_spread_degrees: 4.0,

			projectile_texture: default(),
			projectile_texture_size: Vec2::new(1.0, 1.0),
			fire_sound: default(),
			projectile_damage: 1,
		}
	}
}

#[derive(Bundle)]
pub struct GunBundle {
	//is_gun: IsGun,
	pub velocity: physics::VelocityCalculated,
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
			//is_gun: default(),
			velocity: default(),
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
	mut commands: Commands,
	mut guns: Query<(
		&Parent,
		&GlobalTransform,
		&physics::VelocityCalculated,
		&mut GunCycleTimer,
	)>,
	turrets: Query<(&turret::TurretProperties, &GunProperties)>,
) {
	for (parent_turret, gun_transform, gun_velocity, mut gun_cycle_timer) in guns.iter_mut() {
		// Get Turret and Gun properties from parent turret
		let (turret_properties, gun_properties) = turrets
			.get(parent_turret.0)
			.expect("Failed to get parent turret.");

		gun_cycle_timer.tick(time.delta());

		if turret_properties.turret_state == turret::TurretState::Firing {
			if gun_cycle_timer.finished() {
				// Set timer for RoF delay.
				gun_cycle_timer.0 = Timer::from_seconds(1.0 / gun_properties.rate_of_fire, false);

				// Calculate random spread
				let shot_deviation = (((rand::random::<f32>() + rand::random::<f32>()) / 2.0
					- 0.5) * gun_properties.bullet_spread_degrees)
					.to_radians();

				// Add deviation to projectile velocity
				let velocity_deviation_mps = gun_properties.projectile_velocity_mps
					* gun_properties.velocity_deviation_percent;
				let turret_projectile_velocity = gun_properties.projectile_velocity_mps
					+ (rand::random::<f32>() - 0.5) * velocity_deviation_mps;

				commands
					.spawn_bundle(SpriteBundle {
						sprite: Sprite {
							custom_size: Some(gun_properties.projectile_texture_size),
							..default()
						},
						transform: Transform {
							translation: gun_transform.translation + Vec3::new(0.0, 0.0, -10.0),
							rotation: gun_transform.rotation,
							..default()
						},
						texture: gun_properties.projectile_texture.clone(),
						..default()
					})
					.insert(gun::IsProjectile)
					.insert(physics::Velocity(
						Vec2::from(
							(-gun_transform.rotation.to_scaled_axis().to_array()[2]
								+ shot_deviation)
								.sin_cos(),
						) * turret_projectile_velocity
							+ gun_velocity.velocity.0,
					))
					.insert(interaction::Damage(gun_properties.projectile_damage));

				// Play gunfire sound effect
				audio.play(gun_properties.fire_sound.clone());
			}
		}
	}
}
