use bevy::prelude::*;

pub mod camera;
pub mod gun;
pub mod interaction;
pub mod physics;
pub mod ship;
pub mod spawning;
pub mod targeting;
pub mod turret;

// Resources

#[derive(Deref, DerefMut)]
pub struct SpawnTimer(pub Timer);
