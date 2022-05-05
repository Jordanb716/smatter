use super::*;

#[derive(Component)]
pub struct IsPlayerShip;

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Deref, DerefMut)]
pub struct Health(pub i32);

#[derive(Bundle)]
pub struct ShipBundle{
	pub health: Health,
	pub iff: interaction::IFF,

	pub transform: Transform,
    pub global_transform: GlobalTransform,
	pub velocity: physics::Velocity,
	pub acceleration: physics::Acceleration,
	
	pub sprite: Sprite,
	pub texture: Handle<Image>,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
}

impl Default for ShipBundle {
    fn default() -> Self {
        Self {
			health: Health(1),
			iff: interaction::IFF::Neutral,

			transform: Default::default(),
            global_transform: Default::default(),
			velocity: physics::Velocity(Vec2::new(0.0, 0.0)),
			acceleration: physics::Acceleration(Vec2::new(0.0, 0.0)),

            sprite: Default::default(),
            texture: bevy::render::texture::DEFAULT_IMAGE_HANDLE.typed(),
            visibility: Default::default(),
        }
    }
}