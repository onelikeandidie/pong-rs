use bevy::prelude::Component;

#[derive(Component, Default, Debug)]
pub struct PlayerOwned {
    pub handle: u8,
}

#[derive(Component, Default, Debug)]
pub struct ServerOwned;