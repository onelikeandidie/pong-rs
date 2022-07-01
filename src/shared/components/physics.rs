use bevy::prelude::Component;

#[derive(Component, Default, Debug)]
pub struct PhysicsObject {
    pub vx: f64,
    pub vy: f64,
    pub drag: f64,
}

#[derive(Component, Default, Debug)]
pub struct Collisionable {
    pub collision_mask: u16,
}

#[derive(Component, Default, Debug)]
pub struct Bouncy {
    pub multiplier: f32,
}

#[derive(Component, Default, Debug)]
pub struct HitBox {
    pub height: f64,
    pub width: f64,
    pub shape: HitBoxShapes
}

#[derive(Default, Debug)]
pub enum HitBoxShapes {
    #[default]
    Box,
    Sphere,
    /**@deprecated Not Implemented*/
    Capsule
}