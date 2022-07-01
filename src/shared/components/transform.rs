use bevy::prelude::Component;

#[derive(Component, Default, Debug)]
pub struct Transform {
    pub x: f64,
    pub y: f64
}
