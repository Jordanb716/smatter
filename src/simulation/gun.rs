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

#[derive(Component, Deref, DerefMut)]
pub struct GunPosition(pub Vec2);

#[derive(Component, Clone, Debug)]
pub struct GunProperties {
	pub gun_size: ItemSize,
	pub gun_type: GunType,
	pub rate_of_fire: f32,
	pub projectile_velocity_mps: f32,
	pub gun_cycle_timer: Option<Timer>,
}

impl Default for GunProperties {
	fn default() -> Self {
		Self {
			gun_size: ItemSize::Small,
			gun_type: GunType::Kinetic,
			rate_of_fire: 10.0,
			projectile_velocity_mps: 100.0,
			gun_cycle_timer: None,
		}
	}
}

// ==========
// Systems

pub fn gun_firing_system(
	time: Res<Time>,
	asset_server: Res<AssetServer>,
	audio: Res<Audio>,
	mut commands: Commands,
	mut turret: Query<(
		&Parent,
		&GlobalTransform,
		&turret::TurretProperties,
		&mut gun::GunProperties,
	)>,
	ship_query: Query<&physics::Velocity>,
) {
	for (turret_parent, turret_transform, turret_properties, mut gun_properties) in
		turret.iter_mut()
	{
		if gun_properties.gun_cycle_timer.is_none() {
			gun_properties.gun_cycle_timer = Some(Timer::from_seconds(
				1.0 / gun_properties.rate_of_fire,
				false,
			));
		}

		gun_properties
			.gun_cycle_timer
			.as_mut()
			.unwrap()
			.tick(time.delta());

		if turret_properties.turret_state == turret::TurretState::Firing {
			if gun_properties.gun_cycle_timer.as_ref().unwrap().finished() {
				// Set timer for RoF delay.
				gun_properties.gun_cycle_timer = Some(Timer::from_seconds(
					1.0 / gun_properties.rate_of_fire,
					false,
				));

				// Calculate random spread
				let bullet_spread_degrees = 4.0;
				let shot_deviation = (((rand::random::<f32>() + rand::random::<f32>()) / 2.0
					- 0.5) * bullet_spread_degrees)
					.to_radians();

				// Add deviation to projectile velocity
				let velocity_deviation_mps = gun_properties.projectile_velocity_mps * 0.01;
				let turret_projectile_velocity = gun_properties.projectile_velocity_mps
					+ (rand::random::<f32>() - 0.5) * velocity_deviation_mps;
				let gun_velocity = physics::Velocity(ship_query.get(turret_parent.0).unwrap().0);

				commands
					.spawn_bundle(SpriteBundle {
						sprite: Sprite {
							color: (Color::RED),
							..default()
						},
						transform: Transform {
							translation: turret_transform.translation + Vec3::new(0.0, 0.0, -10.0),
							scale: Vec3::new(10.0, 10.0, 1.0),
							..default()
						},
						..default()
					})
					.insert(gun::IsProjectile)
					.insert(physics::Velocity(
						Vec2::from(
							(-turret_transform.rotation.to_scaled_axis().to_array()[2]
								+ shot_deviation)
								.sin_cos(),
						) * turret_projectile_velocity
							+ gun_velocity.0,
					))
					.insert(interaction::Damage(1));
				audio.play(asset_server.load("temp_gun_fire.ogg"));
			}
		}
	}
}
