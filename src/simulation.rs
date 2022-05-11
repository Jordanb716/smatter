use bevy::prelude::*;

pub mod camera;
pub mod gun;
pub mod gun_list;
pub mod interaction;
pub mod physics;
pub mod ship;
pub mod ship_list;
pub mod spawning;
pub mod targeting;
pub mod turret;
pub mod turret_list;

// Components

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum ItemSize {
	Small,
	Medium,
	Large,
}

// Resources

#[derive(Deref, DerefMut)]
pub struct SpawnTimer(pub Timer);
