use super::*;

#[derive(Component)]
pub struct IsProjectile;

#[derive(Component, Deref, DerefMut)]
pub struct GunShotsPerSecond(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct GunDelayTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct ProjectileVelocity(pub f32);