use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub mod camera;
pub mod gun;
pub mod gun_list;
pub mod interaction;
pub mod physics;
pub mod projectile;
pub mod ship;
pub mod ship_list;
pub mod spawning;
pub mod targeting;
pub mod turret;

// Components

#[derive(Component, Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum ItemSize {
	Small,
	Medium,
	Large,
}

// Resources

#[derive(Deref, DerefMut)]
pub struct SpawnTimer(pub Timer);
