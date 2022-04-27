use super::*;

#[derive(Component)]
pub struct IsTurret;

#[derive(Component, Deref, DerefMut)]
pub struct FireTurret(pub bool);

#[derive(Component, Deref, DerefMut)]
pub struct IsTargeting(pub Option<Entity>);