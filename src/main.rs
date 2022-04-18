use bevy::{prelude::*, render::camera::ScalingMode, sprite::collide_aabb::collide};
use rand::prelude::*;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup)
		.add_system(object_movement_system)
		.add_system(projectile_collision_system)
		.add_system(kill_system)
		.add_system(turret_targeting_system)
		.add_system(turret_firing_system)
		.insert_resource(SpawnTimer(Timer::from_seconds(0.5, true)))
		.add_system(target_spawn_system)
		.run();
}

// Resources

#[derive(Deref, DerefMut)]
struct SpawnTimer(Timer);

// Components

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Projectile;

#[derive(Component)]
struct Turret;

#[derive(Component, Deref, DerefMut)]
struct FireTurret(bool);

#[derive(Component, Deref, DerefMut)]
struct GunShotsPerSecond(f32);

#[derive(Component, Deref, DerefMut)]
struct GunDelayTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component, Deref, DerefMut)]
struct Health(i32);

#[derive(Component, Deref, DerefMut)]
struct Damage(i32);

#[derive(Component, Deref, DerefMut)]
struct GunVelocity(f32);

fn setup(mut commands: Commands) {
	// Camera
	let mut camera = OrthographicCameraBundle::new_2d();
	camera.orthographic_projection.scaling_mode = ScalingMode::FixedHorizontal;
	camera.orthographic_projection.scale = 1000.0;
	commands.spawn_bundle(camera);

	// Turret
	commands
		.spawn_bundle(SpriteBundle {
			sprite: Sprite {
				color: (Color::YELLOW),
				..Default::default()
			},
			transform: Transform {
				translation: Vec3::new(-100.0, 0.0, 0.0),
				scale: Vec3::new(5.0, 10.0, 0.0),
				..Default::default()
			},
			..Default::default()
		})
		.insert(Turret)
		.insert(Velocity(Vec2::new(0.0, 0.0)))
		.insert(GunVelocity(200.0))
		.insert(FireTurret(false))
		.insert(GunShotsPerSecond(40.0))
		.insert(GunDelayTimer(Timer::from_seconds(0.0, false)));

	commands
		.spawn_bundle(SpriteBundle {
			sprite: Sprite {
				color: (Color::YELLOW),
				..Default::default()
			},
			transform: Transform {
				translation: Vec3::new(100.0, 0.0, 0.0),
				scale: Vec3::new(5.0, 10.0, 0.0),
				..Default::default()
			},
			..Default::default()
		})
		.insert(Turret)
		.insert(Velocity(Vec2::new(0.0, 0.0)))
		.insert(GunVelocity(200.0))
		.insert(FireTurret(false))
		.insert(GunShotsPerSecond(40.0))
		.insert(GunDelayTimer(Timer::from_seconds(0.0, false)));
}

fn target_spawn_system(mut commands: Commands, time: Res<Time>, mut timer: ResMut<SpawnTimer>) {
	if timer.tick(time.delta()).just_finished() {
		commands
			.spawn_bundle(SpriteBundle {
				sprite: Sprite {
					color: (Color::YELLOW),
					..Default::default()
				},
				transform: Transform {
					translation: Vec3::new(random::<f32>() * 600.0 - 300.0, 400.0, 0.0),
					scale: Vec3::new(4.0, 4.0, 0.0),
					..Default::default()
				},
				..Default::default()
			})
			.insert(Enemy)
			.insert(Health(10))
			.insert(Velocity(Vec2::new(
				random::<f32>() * 80.0 - 10.0,
				random::<f32>() * -80.0 - 20.0,
			)));
	}
}

fn object_movement_system(mut movement_query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
	for (velocity, mut transform) in movement_query.iter_mut() {
		transform.translation += velocity.0.extend(0.0) * time.delta_seconds();
	}
}

fn projectile_collision_system(
	mut commands: Commands,
	projectile_query: Query<(Entity, &Damage, &Transform)>,
	mut target_query: Query<(&mut Health, &Transform), With<Enemy>>,
) {
	for (projectile_entity, damage, projectile_transform) in projectile_query.iter() {
		for (mut health, target_transform) in target_query.iter_mut() {
			let collision = collide(
				target_transform.translation,
				Vec2::new(target_transform.scale.x, target_transform.scale.y),
				projectile_transform.translation,
				Vec2::new(projectile_transform.scale.x, projectile_transform.scale.y),
			);

			if collision.is_some() {
				health.0 -= damage.0;
				commands.entity(projectile_entity).despawn();
			}
		}
	}
}

fn kill_system(
	mut commands: Commands,
	enemy: Query<(Entity, &Health), (Changed<Health>, With<Enemy>)>,
) {
	for (entity, health) in enemy.iter() {
		if health.0 <= 0 {
			commands.entity(entity).despawn();
		}
	}
}

fn turret_targeting_system(
	mut turret: Query<
		(&mut Transform, &Velocity, &GunVelocity, &mut FireTurret),
		(With<Turret>, Without<Enemy>),
	>,
	enemy: Query<(&Transform, &Velocity), With<Enemy>>,
) {
	if enemy.is_empty() {
		return;
	}

	for mut turret in turret.iter_mut() {
		let gun_velocity = (turret.2).0;
		let mut target = enemy.iter().next().unwrap();
		let mut relative_position = (target.0.translation - turret.0.translation).truncate();

		//Find closest possible target
		for enemy in enemy.iter() {
			relative_position = (target.0.translation - turret.0.translation).truncate();
			let relative_enemy_position = (enemy.0.translation - turret.0.translation).truncate();
			if relative_enemy_position.length() < relative_position.length() {
				target = enemy;
			}
		}

		let relative_velocity = (target.1).0 - (turret.1).0;

		let dot = Vec2::dot(relative_position, relative_velocity);
		let target_distance = relative_position.length_squared();
		let i_speed2 = gun_velocity.powi(2);
		let target_speed = relative_velocity.length_squared();
		let sqrt = ((dot * dot) - target_distance * (target_speed - i_speed2)).sqrt();

		let whatever_the_hell_this_is = (
			(-dot - sqrt) / target_distance,
			(-dot + sqrt) / target_distance,
		);

		if whatever_the_hell_this_is.0 > 0.0 {
			turret.0.rotation = Quat::from_rotation_z(
				((whatever_the_hell_this_is.0 * relative_position + relative_velocity).x
					/ (whatever_the_hell_this_is.0 * relative_position + relative_velocity).y)
					.atan(),
			);
			(turret.3).0 = true;
		} else if whatever_the_hell_this_is.1 > 0.0 {
			turret.0.rotation = Quat::from_rotation_z(
				((whatever_the_hell_this_is.1 * relative_position + relative_velocity).x
					/ (whatever_the_hell_this_is.1 * relative_position + relative_velocity).y)
					.atan(),
			);

			(turret.3).0 = true;
		} else {
			(turret.3).0 = false;
		}
	}
}

fn turret_firing_system(
	time: Res<Time>,
	mut commands: Commands,
	mut turret: Query<
		(
			Entity,
			&Transform,
			&GunVelocity,
			&FireTurret,
			&mut GunDelayTimer,
			&GunShotsPerSecond,
		),
		With<Turret>,
	>,
) {
	for mut turret in turret.iter_mut() {
		let gun_velocity = (turret.2).0;

		(turret.4).tick(time.delta());

		if (turret.3).0 == true {
			if (turret.4).0.finished() {
				// Set timer for RoF delay.
				(turret.4).0 = Timer::from_seconds(1.0 / (turret.5).0, false);

				// Calculate random spread
				let bullet_spread_degrees = 4.0;
				let shot_deviation = (((rand::random::<f32>() + rand::random::<f32>()) / 2.0
					- 0.5) * bullet_spread_degrees)
					.to_radians();

				// Add deviation to projectile velocity
				let velocity_deviation_mps = 1.0;
				let gun_velocity =
					gun_velocity + (rand::random::<f32>() - 0.5) * velocity_deviation_mps;

				commands
					.spawn_bundle(SpriteBundle {
						sprite: Sprite {
							color: (Color::RED),
							..Default::default()
						},
						transform: Transform {
							translation: turret.1.translation,
							scale: Vec3::new(2.0, 2.0, 0.0),
							..Default::default()
						},
						..Default::default()
					})
					.insert(Projectile)
					.insert(Velocity(
						Vec2::from(
							(turret.1.rotation.to_scaled_axis().to_array()[2] + shot_deviation)
								.sin_cos(),
						) * gun_velocity,
					))
					.insert(Damage(1));
			}
		}
	}
}
