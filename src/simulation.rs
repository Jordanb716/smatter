use bevy::prelude::*;

pub mod interaction;
pub mod physics;
pub mod ship;
pub mod spawning;
pub mod targeting;

// Resources

#[derive(Deref, DerefMut)]
pub struct SpawnTimer(pub Timer);

// Components

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct Turret;

#[derive(Component, Deref, DerefMut)]
pub struct FireTurret(pub bool);

#[derive(Component, Deref, DerefMut)]
pub struct GunShotsPerSecond(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct GunDelayTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct Health(pub i32);

#[derive(Component, Deref, DerefMut)]
pub struct Damage(pub i32);

#[derive(Component, Deref, DerefMut)]
pub struct GunVelocity(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct IsTargeting(pub Option<Entity>);

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);