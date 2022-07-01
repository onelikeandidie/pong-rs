use bevy::prelude::Component;

#[derive(Component, Default, Debug)]
pub struct Thonk {
    pub width: f32,
    pub height: f32,
    pub speed: f64
}