use bevy::{prelude::*, sprite::collide_aabb::collide};
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
		.run();
}

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Projectile;

#[derive(Component)]
struct Turret;

#[derive(Component)]
struct FireTurret(bool);

#[derive(Component)]
struct GunShotsPerSecond(f32);

#[derive(Component)]
struct GunShootTimer(Timer);

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Health(i32);

#[derive(Component)]
struct Damage(i32);

#[derive(Component)]
struct GunVelocity(f32);

fn setup(mut commands: Commands) {
	// Camera
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());

	// Target
	commands
		.spawn_bundle(SpriteBundle {
			sprite: Sprite {
				color: (Color::YELLOW),
				..Default::default()
			},
			transform: Transform {
				translation: Vec3::new(-400.0, 300.0, 0.0),
				scale: Vec3::new(10.0, 10.0, 0.0),
				..Default::default()
			},
			..Default::default()
		})
		.insert(Enemy)
		.insert(Health(10))
		.insert(Velocity(Vec2::new(10.0, 0.0)));

	// Turret
	commands
		.spawn_bundle(SpriteBundle {
			sprite: Sprite {
				color: (Color::YELLOW),
				..Default::default()
			},
			transform: Transform {
				translation: Vec3::new(0.0, 0.0, 0.0),
				scale: Vec3::new(5.0, 10.0, 0.0),
				..Default::default()
			},
			..Default::default()
		})
		.insert(Turret)
		.insert(Velocity(Vec2::new(0.0, 0.0)))
		.insert(GunVelocity(50.0))
		.insert(FireTurret(true))
		.insert(GunShotsPerSecond(4.0));
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
	>, // needs target component later
	enemy: Query<(&Transform, &Velocity), With<Enemy>>,
) {
	for mut turret in turret.iter_mut() {
		for enemy in enemy.iter() {
			let gun_velocity = (turret.2).0;
			// Go through enemies to find closest in range later
			let relative_position = (enemy.0.translation - turret.0.translation).truncate();
			let relative_velocity = (enemy.1).0 - (turret.1).0;

			let dot = Vec2::dot(relative_position, relative_velocity);
			let target_distance = relative_position.length_squared();
			let i_speed2 = gun_velocity.powi(2);
			let target_speed = relative_velocity.length_squared();
			let sqrt = ((dot * dot) - target_distance * (target_speed - i_speed2)).sqrt();

			let intercept_time = (
				(-dot - sqrt) / target_distance,
				(-dot + sqrt) / target_distance,
			);

			println!("time: {} {}", intercept_time.0, intercept_time.1);

			if intercept_time.0 > 0.0 {
				turret.0.rotation = Quat::from_rotation_z(
					((intercept_time.0 * relative_position + relative_velocity).x
						/ (intercept_time.0 * relative_position + relative_velocity).y)
						.atan(),
				);
				(turret.3).0 = true;
			} else if intercept_time.1 > 0.0 {
				turret.0.rotation = Quat::from_rotation_z(
					((intercept_time.1 * relative_position + relative_velocity).x
						/ (intercept_time.1 * relative_position + relative_velocity).y)
						.atan(),
				);

				(turret.3).0 = true;
			} else {
				(turret.3).0 = false;
			}
		}
	}
}

fn turret_firing_system(
	mut commands: Commands,
	turret: Query<(&Transform, &GunVelocity, &FireTurret), With<Turret>>,
) {
	for turret in turret.iter() {
		if (turret.2).0 == true {
			commands.insert_resource(GunShootTimer(Timer::from_seconds(1.0 / 4.0, false)));
			commands
				.spawn_bundle(SpriteBundle {
					sprite: Sprite {
						color: (Color::RED),
						..Default::default()
					},
					transform: Transform {
						translation: turret.0.translation,
						scale: Vec3::new(4.0, 4.0, 0.0),
						..Default::default()
					},
					..Default::default()
				})
				.insert(Projectile)
				.insert(Velocity(
					Vec2::from(turret.0.rotation.to_scaled_axis().to_array()[2].sin_cos())
						* (turret.1).0,
				))
				.insert(Damage(1));
		}
	}
}
