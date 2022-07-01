use bevy::prelude::Component;

#[derive(Component, Default, Debug)]
pub struct Ball {
    pub r: f32,
    pub base_speed: f32,
}